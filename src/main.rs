mod triangle_test;

extern crate sdl2;

use std::time::Duration;
use sdl2::event::Event;
use sdl2::event::Event::MouseMotion;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};

use triangle_test::Tri;
use crate::triangle_test::Points;

fn main() {

   let screen_width:u32=800;
   let screen_height:u32=800;

    let sdl_init=sdl2::init().expect("error initializing sdl context");
    let videosub=sdl_init.video().expect("error initializing video sub");

    let window=videosub.window(
        "Graphic programming in rust using sdl2",
        screen_width,
        screen_height,
    ).position_centered().
        allow_highdpi()
        .build().expect("Error creating window ");

    let mut canvas=window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

    let mut event_pump=sdl_init.event_pump().unwrap();

    'running: loop {

        canvas.set_draw_color(Color::RGB(0, 0, 0)); // Set black for clearing
        canvas.clear();

        //
        // canvas.set_draw_color(Color::RGB(255, 0, 0)); // Bright red line
        //
        // canvas.draw_line(
        //     Point::new((screen_width / 2) as i32, (screen_height / 2) as i32),
        //     Point::new((screen_width / 3) as i32, (screen_height / 3) as i32))
        //     .expect("Failed to draw line");
        //
        //
        //  canvas.draw_rect((Rect::new(120, 100, 200, 300))).expect("TODO: panic message");

        let tri = Tri {
            vertex1: Points { x: 100, y: 100 },
            vertex2: Points { x: 200, y: 100 },
            vertex3: Points { x: 150, y: 200 }
        };

        canvas.set_draw_color(Color::RGB(255, 0, 0)); // Red color

        tri.draw(&mut canvas).expect("Failed to draw triangle");

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::MouseMotion {x,y, .. } => {
                    println!("x:{}",x);
                    println!("y {}",y);
                },
                _ => {}
            }
        }
        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
