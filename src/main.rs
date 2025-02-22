mod triangle_test;

extern crate sdl2;

use std::time::Duration;
use sdl2::event::Event;
use sdl2::event::Event::MouseMotion;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;


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
        .opengl()
        .build().expect("Error creating window ");

    let mut canvas=window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

    let mut event_pump=sdl_init.event_pump().unwrap();

    'running: loop {

        canvas.set_draw_color(Color::RGB(0, 0, 0)); // Set black for clearing
        canvas.clear();


        let mut tri = Tri {
            vertex1: Points { x: 200, y: 200 },
            vertex2: Points { x: 300, y: 200 },
            vertex3: Points { x: 250, y: 300 }
        };

        for event in event_pump.poll_iter(){
            match event {
                MouseMotion {x,y,..}=>{
                    tri.vertex1.x=x;
                    tri.vertex1.y=y;
                },
                _=>{}
            }
        }

        canvas.set_draw_color(Color::RGB(255, 0, 0)); // Red color

        tri.draw(&mut canvas).expect("Failed to draw triangle");


        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                MouseMotion {x,y, .. } => {
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
