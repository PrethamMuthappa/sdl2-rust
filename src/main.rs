extern crate sdl2;

use std::time::Duration;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;

fn main() {

    let sdl_init=sdl2::init().expect("error initializing sdl context");
    let videosub=sdl_init.video().expect("error initializing video sub");

    let window=videosub.window(
        "Graphic programming in rust using sdl2",
        500,
        500,
    ).position_centered().
        allow_highdpi()
        .resizable()
        .build().expect("Error creating window ");

    let mut canvas=window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0,255,255));
    canvas.clear();
    canvas.present();

    let mut event_pump=sdl_init.event_pump().unwrap();
    let mut i=0;

    'running: loop {
        i = (i + 1) % 255;
        // canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
        // canvas.clear();

        canvas.draw_line((200, 250), (300, 400)).expect("TODO: panic message");

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }
        canvas.clear();
        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
