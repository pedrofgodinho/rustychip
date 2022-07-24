use std::sync::{Arc, mpsc, RwLock};
use std::thread;
use std::time::Duration;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;
use crate::emulator::Emulator;


pub struct Interface {
    running: bool,
    emulator: Arc<RwLock<Emulator>>,
}

impl Interface {
    pub fn new(emulator: Emulator) -> Interface {
        Interface {
            running: true,
            emulator: Arc::new(RwLock::new(emulator)),
        }
    }

    pub fn run(mut self) {

        let (display_tx, display_rx) = mpsc::channel();
        let (clock_tx, clock_rx) = mpsc::channel();
        let (key_tx, key_rx) = mpsc::channel();
        let (run_tx, run_rx) = mpsc::channel();

        let emulator = self.emulator.clone();
        let handle = thread::spawn(move || {
            while run_rx.try_recv().is_err() {
                if emulator.write().unwrap().step().unwrap() {
                    display_tx.send(()).unwrap();
                }
                if clock_rx.try_recv().is_ok() {
                    emulator.write().unwrap().tick_clock();
                }
                while let Ok((key, state)) = key_rx.try_recv() {
                    emulator.write().unwrap().keypad[key as usize] = state;
                }
                thread::sleep(Duration::from_millis(1));
            }
        });

        let sdl = sdl2::init().unwrap();
        let video_subsystem = sdl.video().unwrap();
        let window = video_subsystem.window("RustyChip", 640, 320)
            .position_centered()
            .build()
            .unwrap();

        let mut canvas = window.into_canvas().build().unwrap();

        let mut event_pump = sdl.event_pump().unwrap();

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        canvas.present();
        while self.running {
            for event in event_pump.poll_iter() {
                self.handle_event(&event, &key_tx);
            }

            if display_rx.try_recv().is_ok() {
                self.draw(&mut canvas);
            }
            // If multiple instructions trigger a redraw, we redraw only once and consume the redraw requests
            while display_rx.try_recv().is_ok() {
            }

            clock_tx.send(()).unwrap();
            thread::sleep(Duration::from_nanos(1_000_000_000u64 / 60));
        }
        run_tx.send(()).unwrap();
        handle.join().unwrap();
    }

    fn handle_event(&mut self, event: &Event, key_tx: &mpsc::Sender<(u8, bool)>) {
        match event {
            Event::Quit {..} |
            Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                self.running = false;
            },
            Event::KeyDown { keycode: Some(Keycode::Num1), .. } => {
                key_tx.send((0, true)).unwrap();
            },
            Event::KeyDown { keycode: Some(Keycode::Num2), .. } => {
                key_tx.send((1, true)).unwrap();
            },
            Event::KeyDown { keycode: Some(Keycode::Num3), .. } => {
                key_tx.send((2, true)).unwrap();
            },
            Event::KeyDown { keycode: Some(Keycode::Num4), .. } => {
                key_tx.send((3, true)).unwrap();
            },
            Event::KeyDown { keycode: Some(Keycode::Q), .. } => {
                key_tx.send((4, true)).unwrap();
            },
            Event::KeyDown { keycode: Some(Keycode::W), .. } => {
                key_tx.send((5, true)).unwrap();
            },
            Event::KeyDown { keycode: Some(Keycode::E), .. } => {
                key_tx.send((6, true)).unwrap();
            },
            Event::KeyDown { keycode: Some(Keycode::R), .. } => {
                key_tx.send((7, true)).unwrap();
            },
            Event::KeyDown { keycode: Some(Keycode::A), .. } => {
                key_tx.send((8, true)).unwrap();
            },
            Event::KeyDown { keycode: Some(Keycode::S), .. } => {
                key_tx.send((9, true)).unwrap();
            },
            Event::KeyDown { keycode: Some(Keycode::D), .. } => {
                key_tx.send((10, true)).unwrap();
            },
            Event::KeyDown { keycode: Some(Keycode::F), .. } => {
                key_tx.send((11, true)).unwrap();
            },
            Event::KeyDown { keycode: Some(Keycode::Z), .. } => {
                key_tx.send((12, true)).unwrap();
            },
            Event::KeyDown { keycode: Some(Keycode::X), .. } => {
                key_tx.send((13, true)).unwrap();
            },
            Event::KeyDown { keycode: Some(Keycode::C), .. } => {
                key_tx.send((14, true)).unwrap();
            },
            Event::KeyDown { keycode: Some(Keycode::V), .. } => {
                key_tx.send((15, true)).unwrap();
            },
            Event::KeyUp { keycode: Some(Keycode::Num1), .. } => {
                key_tx.send((0, false)).unwrap();
            },
            Event::KeyUp { keycode: Some(Keycode::Num2), .. } => {
                key_tx.send((1, false)).unwrap();
            },
            Event::KeyUp { keycode: Some(Keycode::Num3), .. } => {
                key_tx.send((2, false)).unwrap();
            },
            Event::KeyUp { keycode: Some(Keycode::Num4), .. } => {
                key_tx.send((3, false)).unwrap();
            },
            Event::KeyUp { keycode: Some(Keycode::Q), .. } => {
                key_tx.send((4, false)).unwrap();
            },
            Event::KeyUp { keycode: Some(Keycode::W), .. } => {
                key_tx.send((5, false)).unwrap();
            },
            Event::KeyUp { keycode: Some(Keycode::E), .. } => {
                key_tx.send((6, false)).unwrap();
            },
            Event::KeyUp { keycode: Some(Keycode::R), .. } => {
                key_tx.send((7, false)).unwrap();
            },
            Event::KeyUp { keycode: Some(Keycode::A), .. } => {
                key_tx.send((8, false)).unwrap();
            },
            Event::KeyUp { keycode: Some(Keycode::S), .. } => {
                key_tx.send((9, false)).unwrap();
            },
            Event::KeyUp { keycode: Some(Keycode::D), .. } => {
                key_tx.send((10, false)).unwrap();
            },
            Event::KeyUp { keycode: Some(Keycode::F), .. } => {
                key_tx.send((11, false)).unwrap();
            },
            Event::KeyUp { keycode: Some(Keycode::Z), .. } => {
                key_tx.send((12, false)).unwrap();
            },
            Event::KeyUp { keycode: Some(Keycode::X), .. } => {
                key_tx.send((13, false)).unwrap();
            },
            Event::KeyUp { keycode: Some(Keycode::C), .. } => {
                key_tx.send((14, false)).unwrap();
            },
            Event::KeyUp { keycode: Some(Keycode::V), .. } => {
                key_tx.send((15, false)).unwrap();
            },
            _ => {}
        }
    }

    fn draw(&mut self, canvas: &mut WindowCanvas) {
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        let emulator = self.emulator.read().unwrap();
        for x in 0..64 {
            for y in 0..32 {
                if emulator.display[y][x] {
                    canvas.fill_rect(sdl2::rect::Rect::new(x as i32 * 10, y as i32 * 10, 10, 10)).unwrap();
                }
            }
        }
        canvas.present();
    }
}