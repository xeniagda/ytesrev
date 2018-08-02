extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::Canvas;
use sdl2::pixels::Color;
use sdl2::video::Window;
use sdl2::EventPump;

use std::time::{Duration, Instant};
use std::thread::sleep;

use scene::Scene;
use latex::render_all_eqations;

const SIZE: (usize, usize) = (800, 600);

pub struct WindowManager<'a> {
    pub canvas: Canvas<Window>,
    pub event_pump: EventPump,

    pub other_scenes: Vec<&'a mut dyn Scene>,
    pub curr_scene: &'a mut dyn Scene,

    time_manager: TimeManager,
}


struct TimeManager {
    last_time: Instant,

    last_fps_print: Instant,
    durs: Vec<Duration>,
}

impl <'a> WindowManager<'a> {
    pub fn init_window(
            curr_scene: &'a mut dyn Scene,
            mut other_scenes: Vec<&'a mut dyn Scene>
    ) -> WindowManager<'a> {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem.window("Ytesrev", SIZE.0 as u32, SIZE.1 as u32)
            .position_centered()
            .resizable()
            .build()
            .unwrap();

        let canvas = window.into_canvas().build().unwrap();

        let event_pump = sdl_context.event_pump().unwrap();

        let time_manager = TimeManager::new();

        // Load everything

        curr_scene.register();
        for scene in other_scenes.iter_mut() {
            scene.register();
        }

        render_all_eqations().expect("Can't render!");

        curr_scene.load();
        for scene in other_scenes.iter_mut() {
            scene.load();
        }

        WindowManager {
            canvas: canvas,
            event_pump: event_pump,
            other_scenes: other_scenes,
            curr_scene: curr_scene,
            time_manager: time_manager,
        }
    }

    pub fn process_events(&mut self) -> bool {
        let dt = self.time_manager.dt();

        self.curr_scene.update(dt);

        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    return false
                },
                _ => {}
            }
        }

        true
    }

    pub fn draw(&mut self) {

        self.canvas.set_draw_color(Color::RGBA(255, 255, 255, 255));
        self.canvas.clear();

        self.curr_scene.draw(&mut self.canvas);

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

