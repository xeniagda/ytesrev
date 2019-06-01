//! Manage the windows on screen

extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::EventPump;
use sdl2::Sdl;

use std::thread::sleep;
use std::time::{Duration, Instant};

use drawable::{DrawSettings, DSETTINGS_MAIN, DSETTINGS_NOTES};
use latex::render::render_all_equations;
use scene::{Action, Scene};

const FPS_PRINT_RATE: Duration = Duration::from_millis(1000);

/// An event. Passed into the `Drawable::event` and `Scene::event` functions
pub enum YEvent {
    /// A special event that is emmitted when the user advances the state of the presentation
    Step,
    /// Step to the next slide in the presentation
    StepSlide,
    /// Anything else
    Other(Event),
}

/// Settings for how a single window should behave
pub struct WindowSettings {
    /// The inner `DrawSettings` to be used for this window
    pub draw_settings: DrawSettings,
    /// The size of this window
    pub window_size: (u32, u32),
}

/// Settings for the manager in general
pub struct WindowManagerSettings {
    /// Title and settings of each window
    pub windows: Vec<(String, WindowSettings)>,
    /// What events should make the presentation step forward? Default: The space button or mouse
    /// press
    pub event_step_rule: Box<dyn Fn(Event) -> bool>,
    /// What events should make the presentation go forward one slide? Default: enter
    pub event_step_slide_rule: Box<dyn Fn(Event) -> bool>,
    /// At what event should the presentation quit? Default: On escape or the window is closed.
    pub quit_rule: Box<dyn Fn(Event) -> bool>,
}

/// Create default settings for a given title
pub fn default_settings(title: &str) -> WindowManagerSettings {
    let title = title.to_string();
    let mut notes_title = title.clone();
    notes_title.push_str(" - Notes");
    WindowManagerSettings {
        windows: vec![(title, WSETTINGS_MAIN), (notes_title, WSETTINGS_NOTES)],
        event_step_rule: Box::new(|event| match event {
            Event::KeyDown {
                keycode: Some(Keycode::Space),
                ..
            } => true,
            Event::MouseButtonDown { .. } => true,
            _ => false,
        }),
        event_step_slide_rule: Box::new(|event| match event {
            Event::KeyDown {
                keycode: Some(Keycode::Return),
                ..
            } => true,
            _ => false,
        }),
        quit_rule: Box::new(|event| match event {
            Event::KeyDown {
                keycode: Some(Keycode::Escape),
                ..
            } => true,
            Event::Quit { .. } => true,
            _ => false,
        }),
    }
}

/// The default window settings for the main window
pub const WSETTINGS_MAIN: WindowSettings = WindowSettings {
    draw_settings: DSETTINGS_MAIN,
    window_size: (1200, 800),
};

/// The default window settings for the notes window
pub const WSETTINGS_NOTES: WindowSettings = WindowSettings {
    draw_settings: DSETTINGS_NOTES,
    window_size: (600, 400),
};

/// The manager of the entire presentation.
pub struct WindowManager<T: Scene> {
    /// The sdl context from `sdl2::init`
    pub context: Sdl,

    /// All canvases, together with their respective settings
    pub canvases: Vec<(WindowSettings, Canvas<Window>)>,

    /// What events should make the presentation step forward?
    pub event_step_rule: Box<dyn Fn(Event) -> bool>,
    /// What events should make the presentation step one slide forward?
    pub event_step_slide_rule: Box<dyn Fn(Event) -> bool>,
    /// At what event should the presentation quit? Default: On escape or the window is closed.
    pub quit_rule: Box<dyn Fn(Event) -> bool>,

    /// The event pump
    pub event_pump: EventPump,

    /// The scene being presented
    pub scene: T,

    time_manager: Option<TimeManager>,
    tick: usize,
}

struct TimeManager {
    last_time: Instant,

    last_fps_print: Instant,
    durs: Vec<Duration>,
}
impl<T: Scene> WindowManager<T> {
    /// Create a window manager
    ///
    /// This loads all scences and creates the windows according to the settings
    pub fn init_window(mut scene: T, settings: WindowManagerSettings) -> WindowManager<T> {
        // Load everything
        scene.register();

        let start = Instant::now();
        eprintln!("Loading...");
        render_all_equations().expect("Can't render!");

        scene.load();

        let delta = Instant::now() - start;
        eprintln!(
            "Done! Took {:.2}s",
            delta.as_secs() as f64 + delta.subsec_millis() as f64 / 1000.
        );

        let mut canvases = Vec::with_capacity(settings.windows.len());

        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        for (title, settings) in settings.windows {
            let window = video_subsystem
                .window(&title, settings.window_size.0, settings.window_size.1)
                .position_centered()
                .resizable()
                .build()
                .unwrap();
            let canvas = window.into_canvas().build().unwrap();

            canvases.push((settings, canvas));
        }

        let event_pump = sdl_context.event_pump().unwrap();

        WindowManager {
            context: sdl_context,
            canvases,
            event_step_rule: settings.event_step_rule,
            event_step_slide_rule: settings.event_step_slide_rule,
            quit_rule: settings.quit_rule,
            event_pump,
            scene,
            time_manager: None,
            tick: 0,
        }
    }

    fn process_events(&mut self) -> bool {
        if let Some(ref mut tm) = self.time_manager {
            let dt = tm.dt();

            self.scene.update(dt);
            match self.scene.action() {
                Action::Done => {
                    return false;
                }
                _ => {}
            }

            for event in self.event_pump.poll_iter() {
                if (*self.quit_rule)(event.clone()) {
                    return false;
                } else if (*self.event_step_slide_rule)(event.clone()) {
                    self.scene.event(YEvent::StepSlide)
                } else if (*self.event_step_rule)(event.clone()) {
                    self.scene.event(YEvent::Step)
                } else {
                    self.scene.event(YEvent::Other(event))
                }
            }
        } else {
            self.time_manager = Some(TimeManager::new());
        }

        true
    }

    fn draw(&mut self) {
        for (ref mut settings, ref mut canvas) in &mut self.canvases {
            canvas.set_draw_color(Color::RGBA(
                settings.draw_settings.background_color.0,
                settings.draw_settings.background_color.1,
                settings.draw_settings.background_color.2,
                255,
            ));
            canvas.clear();

            settings.window_size = canvas.window().size();

            self.scene.draw(canvas, settings.draw_settings);

            canvas.present();
        }
    }

    /// Starts the entire presentation. This will block the current thread until the presentation
    /// has ended.
    pub fn start(&mut self) {
        loop {
            self.tick += 1;
            self.draw();
            if !self.process_events() {
                break;
            }

            sleep(Duration::from_millis(5));
        }
    }
}

impl TimeManager {
    fn new() -> TimeManager {
        TimeManager {
            last_time: Instant::now(),
            last_fps_print: Instant::now(),
            durs: Vec::new(),
        }
    }

    fn dt(&mut self) -> f64 {
        let now = Instant::now();

        let diff = now - self.last_time;
        self.last_time = now;

        self.durs.push(diff);
        if now - self.last_fps_print > FPS_PRINT_RATE {
            let num_dur = self.durs.len() as u32;

            let avg_dur: Duration = self.durs.drain(..).sum::<Duration>() / num_dur;

            let fps = 1. / (avg_dur.as_secs() as f64 + avg_dur.subsec_millis() as f64 / 1000.);

            eprintln!("FPS: {:.2}", fps);

            self.last_fps_print = now;
        }

        diff.as_secs() as f64 + diff.subsec_millis() as f64 / 1000.
    }
}
