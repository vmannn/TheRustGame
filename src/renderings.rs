use piston_window::{rectangle, Context, G2d};
use piston_window::types::Color;

const SQUARES: f64 = 25.0;


pub fn square_render(color: Color, x: i32, y: i32, g2d: &mut G2d, context: &Context)
     {
         let mapx = (x as f64) * SQUARES;
         let mapy = (y as f64) * SQUARES;


         rectangle(

             color,
             [mapx, mapy, SQUARES, SQUARES],
             context.transform,
              g2d,
         );

     }
