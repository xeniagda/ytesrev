extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::Canvas;
use sdl2::pixels::Color;
use sdl2::video::Window;
use sdl2::rect::Rect;
use sdl2::EventPump;

use std::time::{Duration, Instant};
use std::thread::sleep;

use scene::{Scene, Action};
use latex::render_all_eqations;
use drawable::Position;

const SIZE: (usize, usize) = (1200, 800);
const BACKGROUND: (u8, u8, u8) = (255, 248, 234);

pub enum YEvent {
    Step,
    Other(Event),
}

pub struct WindowManager {
    pub canvas: Canvas<Window>,
    pub event_pump: EventPump,

    pub other_scenes: Vec<Box<dyn Scene>>,
    pub curr_scene: Box<dyn Scene>,

    time_manager: Option<TimeManager>,
}


struct TimeManager {
    last_time: Instant,

    last_fps_print: Instant,
    durs: Vec<Duration>,
}

impl WindowManager {
    pub fn init_window(
            mut curr_scene: Box<dyn Scene>,
            mut other_scenes: Vec<Box<dyn Scene>>
    ) -> WindowManager {
        // Load everything

        curr_scene.as_mut_drawable().register();
        for scene in &mut other_scenes {
            scene.as_mut_drawable().register();
        }

        println!("Loading...");
        render_all_eqations().expect("Can't render!");

        println!("Scene 1...");
        curr_scene.as_mut_drawable().load();
        for (i, scene) in other_scenes.iter_mut().enumerate() {
            println!("Scene {}...", i + 2);
            scene.as_mut_drawable().load();
        }
        println!("Done!");


        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem.window("Ytesrev", SIZE.0 as u32, SIZE.1 as u32)
            .position_centered()
            .resizable()
            .build()
            .unwrap();

        let canvas = window.into_canvas().build().unwrap();

        let event_pump = sdl_context.event_pump().unwrap();

        WindowManager {
            canvas,
            event_pump,
            other_scenes,
            curr_scene,
            time_manager: None,
        }
    }

    pub fn process_events(&mut self) -> bool {
        match &mut self.time_manager {
            Some(ref mut tm) => {
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
                        Event::Quit {..} |
                        Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                            return false
                        },
                        _ => {}
                    }


                    match event {
                        Event::KeyDown { keycode: Some(Keycode::Return), ..} => {
                            if self.other_scenes.is_empty() {
                                return false;
                            }
                            self.curr_scene = self.other_scenes.remove(0);
                        }
                        Event::KeyDown { keycode: Some(Keycode::Space), ..}
                        | Event::MouseButtonDown { ..} => {
                            self.curr_scene.event(YEvent::Step)
                        }
                        e => {
                            self.curr_scene.event(YEvent::Other(e))
                        }
                    };

                }

            },
            None => {
                self.time_manager = Some(TimeManager::new());
            }
        }

        true
    }

    pub fn draw(&mut self) {

        self.canvas.set_draw_color(Color::RGBA(BACKGROUND.0, BACKGROUND.1, BACKGROUND.2, 255));
        self.canvas.clear();

        let (w, h) = self.canvas.window().size();
        self.curr_scene.as_mut_drawable().draw(&mut self.canvas, &Position::Rect(Rect::new(0, 0, w, h)));

        self.canvas.present();
    }

    pub fn start(&mut self) {
        loop {
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
            durs: Vec::new()
        }
    }

    fn dt(&mut self) -> f64 {
        let now = Instant::now();

        let diff = now - self.last_time;
        self.last_time = now;

        self.durs.push(diff);
        if now - self.last_fps_print > Duration::from_secs(5) {
            let num_dur = self.durs.len() as u32;

            let avg_dur: Duration = self.durs.drain(..).sum::<Duration>() / num_dur;

            let fps = Duration::from_secs(1).as_millis() as f64 / avg_dur.as_millis() as f64;

            eprintln!("FPS: {:.2}", fps);

            self.last_fps_print = now;
        }

        diff.as_secs() as f64 + diff.as_millis() as f64 / 1000.
    }
}

