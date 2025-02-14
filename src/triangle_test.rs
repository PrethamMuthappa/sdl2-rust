use sdl2::render::Canvas;
use sdl2::video::Window;

pub struct Tri {
    pub(crate) vertex1:Points,
    pub(crate) vertex2:Points,
    pub(crate) vertex3:Points
}
pub struct Points {
 pub(crate) x:i32,
 pub(crate) y:i32,
}

impl Tri {
    pub fn draw(&self, canvas: &mut Canvas<Window>) -> Result<(), String> {
        // Draw three lines connecting all vertices
        canvas.draw_line(
            sdl2::rect::Point::new(self.vertex1.x, self.vertex1.y),
            sdl2::rect::Point::new(self.vertex2.x, self.vertex2.y)
        )?;

        canvas.draw_line(
            sdl2::rect::Point::new(self.vertex2.x, self.vertex2.y),
            sdl2::rect::Point::new(self.vertex3.x, self.vertex3.y)
        )?;

        canvas.draw_line(
            sdl2::rect::Point::new(self.vertex3.x, self.vertex3.y),
            sdl2::rect::Point::new(self.vertex1.x, self.vertex1.y)
        )?;

        Ok(())
    }
}