extern crate sdl3;

use sdl3::rect::Rect;
use sdl3::pixels::Color;
use sdl3::event::Event;
use sdl3::keyboard::Keycode;
use sdl3::keyboard::Scancode;
use std::time::Duration;

pub fn main() {
    let sdl_context = sdl3::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("rust-sdl3 demo", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let player_speed = 4;

    let mut canvas = window.into_canvas();

    let mut rect = Rect::new(64, 64, 128, 128);

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut i = 0;
    'running: loop {
        i = (i + 1) % 255;
        canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
        canvas.clear();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running;
                },
                _ => {}
            }
        }


        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.fill_rect(rect).unwrap();

        let ks = event_pump.keyboard_state();
        for sc in ks.pressed_scancodes() {
            // println!("{}", sc.name());

            match sc {
                Scancode::W => rect.y -= player_speed,
                Scancode::S => rect.y += player_speed,
                Scancode::A => rect.x -= player_speed,
                Scancode::D => rect.x += player_speed,
                _ => {}
            }
        }


        // The rest of the game loop goes here...

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}