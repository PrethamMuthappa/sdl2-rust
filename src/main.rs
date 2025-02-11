extern crate sdl2;

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
        .build().expect("Error creating window ");

    let mut canvas=window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0,255,255));
    canvas.clear();
    canvas.present();

}
