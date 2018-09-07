//! Manage the windows on screen

extern crate rayon;
extern crate sdl2;

use rayon::prelude::*;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::EventPump;

use std::thread::sleep;
use std::time::{Duration, Instant};

use drawable::{DrawSettings, DSETTINGS_MAIN, DSETTINGS_NOTES};
use latex::render::render_all_equations;
use scene::{Action, Scene};

const BACKGROUND: (u8, u8, u8) = (255, 248, 234);
const FPS_PRINT_RATE: Duration = Duration::from_millis(1000);

/// An event. Passed into the `Drawable::event` and `Scene::event` functions
pub enum YEvent {
    /// A special event that is emmitted when the user advances the state of the presentation
    Step,
    /// Anything else
    Other(Event),
}

/// Settings for how a window should behave
pub struct WindowSettings {
    draw_settings: DrawSettings,
    window_size: (u32, u32),
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
pub struct WindowManager {
    /// All canvases, together with their respective settings
    pub canvases: Vec<(WindowSettings, Canvas<Window>)>,
    /// The event pump
    pub event_pump: EventPump,

    /// A list of the scenes that is not currently presented, or has been presented, in order
    pub other_scenes: Vec<Box<dyn Scene>>,
    /// The current scene that is being presented
    pub curr_scene: Box<dyn Scene>,

    time_manager: Option<TimeManager>,
    tick: usize,
}

struct TimeManager {
    last_time: Instant,

    last_fps_print: Instant,
    durs: Vec<Duration>,
}

impl WindowManager {
    /// Shorthand for `WindowManager::init_window(scenes, vec![SETTINGS_MAIN, SETTINGS_NOTES])`,
    /// creating two windows, one for the main presentation and one for notes
    pub fn init_main_note(scenes: Vec<Box<dyn Scene>>, title: String) -> WindowManager {
        let mut notes_title = title.clone();
        notes_title.push_str(" - Notes");
        WindowManager::init_window(
            scenes,
            vec![(title, WSETTINGS_MAIN), (notes_title, WSETTINGS_NOTES)],
        )
    }
    /// Create a window manager
    ///
    /// This loads all scences and creates the windows according to the settings
    pub fn init_window(
        mut scenes: Vec<Box<dyn Scene>>,
        windows: Vec<(String, WindowSettings)>,
    ) -> WindowManager {
        // Load everything

        for scene in &mut scenes {
            scene.register();
        }

        let start = Instant::now();
        eprintln!("Loading...");
        render_all_equations().expect("Can't render!");

        let mut scenes = scenes
            .into_par_iter()
            .enumerate()
            .map(|(i, mut scene)| {
                eprintln!("Scene {}...", i + 1);
                scene.load();
                scene
            }).collect::<Vec<Box<dyn Scene>>>();

        let delta = Instant::now() - start;
        eprintln!(
            "Done! Took {:.2}s",
            delta.as_secs() as f64 + delta.subsec_millis() as f64 / 1000.
        );

        let mut canvases = Vec::with_capacity(windows.len());

        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        for (title, settings) in windows {
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

        let curr_scene = scenes.remove(0);

        WindowManager {
            canvases,
            event_pump,
            curr_scene,
            other_scenes: scenes,
            time_manager: None,
            tick: 0,
        }
    }

    fn process_events(&mut self) -> bool {
        if let Some(ref mut tm) = self.time_manager {
            let dt = tm.dt();

            self.curr_scene.update(dt);
            match self.curr_scene.action() {
                Action::Next => {
                    if self.other_scenes.is_empty() {
                        return false;
                    }
                    self.curr_scene = self.other_scenes.remove(0);
                }
                Action::Continue => {}
            }

            for event in self.event_pump.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => return false,
                    _ => {}
                }

                match event {
                    Event::KeyDown {
                        keycode: Some(Keycode::Return),
                        ..
                    } => {
                        if self.other_scenes.is_empty() {
                            return false;
                        }
                        self.curr_scene = self.other_scenes.remove(0);
                    }
                    Event::KeyDown {
                        keycode: Some(Keycode::Space),
                        ..
                    }
                    | Event::MouseButtonDown { .. } => self.curr_scene.event(YEvent::Step),
                    e => self.curr_scene.event(YEvent::Other(e)),
                };
            }
        } else {
            self.time_manager = Some(TimeManager::new());
        }

        true
    }

    fn draw(&mut self) {
        for (ref mut settings, ref mut canvas) in &mut self.canvases {
            canvas.set_draw_color(Color::RGBA(BACKGROUND.0, BACKGROUND.1, BACKGROUND.2, 255));
            canvas.clear();

            self.curr_scene.draw(canvas, settings.draw_settings);

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
