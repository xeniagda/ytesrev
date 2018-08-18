//! Manage the windows on screen

extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::EventPump;

use std::thread::sleep;
use std::time::{Duration, Instant};

use drawable::{Position, SETTINGS_MAIN, SETTINGS_NOTES};
use latex::render::render_all_equations;
use scene::{Action, Scene};

const SIZE: (usize, usize) = (1200, 800);
const BACKGROUND: (u8, u8, u8) = (255, 248, 234);
const FPS_PRINT_RATE: Duration = Duration::from_millis(1000);

const NOTES: bool = true;

/// An event. Passed into the `Drawable::event` and `Scene::event` functions
pub enum YEvent {
    /// A special event that is emmitted when the user advances the state of the presentation
    Step,
    /// Anything else
    Other(Event),
}

/// The manager of the entire presentation.
pub struct WindowManager {
    /// The canvas of the main window
    pub canvas: Canvas<Window>,
    /// The (optinal) canvas of the notes window
    pub notes_canvas: Option<Canvas<Window>>,
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
    /// Create a window manager
    ///
    /// This loads all scences and creates the main and notes window
    pub fn init_window(
        mut curr_scene: Box<dyn Scene>,
        mut other_scenes: Vec<Box<dyn Scene>>,
    ) -> WindowManager {
        // Load everything

        curr_scene.as_mut_drawable().register();
        for scene in &mut other_scenes {
            scene.as_mut_drawable().register();
        }

        let start = Instant::now();
        eprintln!("Loading...");
        render_all_equations().expect("Can't render!");

        eprintln!("Scene 1...");
        curr_scene.as_mut_drawable().load();
        for (i, scene) in other_scenes.iter_mut().enumerate() {
            eprintln!("Scene {}...", i + 2);
            scene.as_mut_drawable().load();
        }
        let delta = Instant::now() - start;
        eprintln!("Done! Took {:.2}s", delta.as_secs() as f64 + delta.subsec_millis() as f64 / 1000.);


        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let main_window = video_subsystem
            .window("Ytesrev", SIZE.0 as u32, SIZE.1 as u32)
            .position_centered()
            .resizable()
            .build()
            .unwrap();

        let canvas = main_window.into_canvas().build().unwrap();

        let notes_canvas =
            if NOTES {
                let notes_window = video_subsystem
                    .window("Ytesrev - Notes", SIZE.0 as u32 / 2, SIZE.1 as u32 / 2)
                    .position_centered()
                    .resizable()
                    .build()
                    .unwrap();
                Some(notes_window.into_canvas().build().unwrap())
            } else {
                None
            };

        let event_pump = sdl_context.event_pump().unwrap();

        WindowManager {
            canvas,
            notes_canvas,
            event_pump,
            other_scenes,
            curr_scene,
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
        self.canvas
            .set_draw_color(Color::RGBA(BACKGROUND.0, BACKGROUND.1, BACKGROUND.2, 255));
        self.canvas.clear();

        let (w, h) = self.canvas.window().size();
        self.curr_scene.as_mut_drawable().draw(
            &mut self.canvas,
            &Position::Rect(Rect::new(0, 0, w, h)),
            SETTINGS_MAIN,
        );

        self.canvas.present();

        if let Some(ref mut notes_canvas) = self.notes_canvas {
            if self.tick % 5 == 0 {
                notes_canvas.set_draw_color(Color::RGBA(
                    BACKGROUND.0,
                    BACKGROUND.1,
                    BACKGROUND.2,
                    255,
                ));
                notes_canvas.clear();

                let (w, h) = notes_canvas.window().size();
                self.curr_scene.as_mut_drawable().draw(
                    notes_canvas,
                    &Position::Rect(Rect::new(0, 0, w, h)),
                    SETTINGS_NOTES,
                );

                notes_canvas.present();
            }
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
