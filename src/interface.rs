use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;
use crate::emulator::Emulator;

pub struct Interface {
    emulator: Emulator,
    running: bool,
}


impl Interface {
    pub fn new(emulator: Emulator) -> Interface {
        Interface {
            emulator,
            running: false,
        }
    }

    pub fn run(&mut self) {
        let sdl = sdl2::init().unwrap();
        let video_subsystem = sdl.video().unwrap();
        let window = video_subsystem
            .window("RustyChip", 640, 320)
            .build()
            .unwrap();

        let mut canvas = window.into_canvas().build().unwrap();

        let mut event_pump = sdl.event_pump().unwrap();

        self.running = true;
        while self.running {
            for event in event_pump.poll_iter() {
                self.handle_event(&event);
            }

            if self.emulator.step().unwrap() {
                self.draw(&mut canvas);
            }
        }
    }

    fn handle_event(&mut self, event: &Event) {
        match event {
            Event::Quit {..} |
            Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                self.running = false;
            },
            _ => {}
        }
    }

    fn draw(&mut self, canvas: &mut WindowCanvas) {
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        for x in 0..64 {
            for y in 0..32 {
                if self.emulator.display[y][x] {
                    canvas.fill_rect(sdl2::rect::Rect::new(x as i32 * 10, y as i32 * 10, 10, 10)).unwrap();
                }
            }
        }
        canvas.present();
    }
}