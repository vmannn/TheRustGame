extern crate rand;
extern crate piston_window;
extern crate opengl_graphics;
extern crate graphics;




use opengl_graphics::{GlGraphics, OpenGL};
use opengl_graphics::glyph_cache::GlyphCache;



mod renderings;
mod game;
use piston_window::types::Color;
use piston_window::*;
use game::Game;

const Background: Color =  [0.00, 0.00, 0.00, 0.00];




fn main() {


     let mut space: PistonWindow =
         WindowSettings::new("snakegame", [((30 as f64 * 15.00) as u32), ((26 as f64 * 15.00) as u32)])
         .exit_on_esc(true).build().unwrap();



      let opengl = OpenGL::V3_2;
      let mut gl = GlGraphics::new(opengl);
    let mut glyph_cache = GlyphCache::new("assets/Roboto-Regular.ttf").expect("Error unwraping fonts");
     let mut mygame = Game::new();
     while let Some(thing) = space.next()
     {

         if let Some(Button::Keyboard(press)) = thing.press_args()
         {

             mygame.key_input(press);
         }
         space.draw_2d(&thing, |context, g2d|
         {

             mygame.drawgame(&context, g2d, &mut gl, &mut glyph_cache);



         });

         thing.update(|arg| { mygame.update(arg.dt);});

     }



}
