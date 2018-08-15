use graphics::text::Text;
use graphics::{DrawsState, Context, Transformed};
use piston_window::keyboard::Key;
use std::collections::LinkedList;
use piston_window::{Context, G2d};
use piston_window::types::Color;
use renderings::square_render;
use renderings::sequence_render;
use rand::{thread_rng, Rng};
use opengl_graphics::{GlGraphics, OpenGL};
use opengl_graphics::glyph_cache::GlyphCache;
const COLOR: Color = [0.00, 0.80, 0.00, 1.0];
const ENCLOSURE: Color = [0.80, 0.00, 0.00, 1.0];
const APPLE_COLOR: Color = [0.80, 0.00, 0.00, 1.0];
const POISONED_APPLE_COLOR: Color = [0.5, 0.0, 0.00, 1.0];
const Game_time: f64 = 3.0;
const Restart_time: f64 = 1.0;


pub struct Game
{
    snake: mysnake,
    lose: bool,
    wait_time: f64,
    apple_rendered: bool,
    applex: i32,
    appley: i32,
    poisoned_apples: LinkedList<square>,
    score: i32,


}


impl Game
{
    pub fn new() -> Game
    {


       let mut poison: LinkedList<square> = LinkedList::new();
       poison.push_back(square
       {
           x: 6,
           y: 7,
       });


        Game
        {

            snake: mysnake::create(3,3),
            lose: false,
            wait_time: 0.0,
            apple_rendered: true,
            applex: 3,
            appley: 5,
            poisoned_apples: poison,



        }
    }




    pub fn random_poison_add(&mut self)
    {
        let mut poisons: LinkedList<square> = LinkedList::new();

        let mut random = thread_rng();
        let mut applexx = random.gen_range(1, 14);
        let mut appleyy = random.gen_range(1, 14);

        //what if food'\ cant overlaps snake
        while self.snake.overlap(applexx, appleyy)
        {
            applexx = random.gen_range(1, 14);
            appleyy = random.gen_range(1, 14);
        }



        let pois = (square
        {
            x: applexx,
            y: appleyy,
        });

        self.poisoned_apples.push_back(pois);



    }

    pub fn render_poison(&self, g2d: &mut G2d, context: &Context)
    {
        for square in &self.poisoned_apples
        {

            square_render(POISONED_APPLE_COLOR, square.x, square.y, g2d, context);

        }
    }

    pub fn drawgame(&self, context: &Context, g2d: &mut G2d, mut gl: &mut GlGraphics,
               mut glyph_cache: &mut GlyphCache)
    {


        let transform = context.transform.trans([0], [1]);
        Text::new_color()
        self.snake.render(g2d, context);
        self.render_poison(g2d, context);
        if self.apple_rendered
        {
            square_render(APPLE_COLOR, self.applex, self.appley, g2d, context);


        }



    //gameover

    }


    pub fn snake_poisoned(&self) -> bool
    {

        for square in &self.snake.snake
        {

            for square2 in &self.poisoned_apples
            {
                if square.x == square2.x && square.y == square2.y
                {
                    return true;
                }


            }


        }



        return false;


    }

    pub fn update(&mut self, time: f64)
    {
        self.wait_time = self.wait_time + time;


        let (blockx, blocky): (i32, i32) = self.snake.head_coordinates();

        if self.lose
        {

            if self.wait_time > Restart_time
            {
                self.snake = mysnake::create(3, 3);
                self.apple_rendered = true;
                self.applex = 8;
                self.appley = 8;
                self.lose = false;
                self.wait_time = 0.0;
            }

            return;



        }




        if !self.apple_rendered
        {

            self.snake.place_tail();
            let mut random = thread_rng();
            let mut applexx = random.gen_range(1, 14);
            let mut appleyy = random.gen_range(1, 14);

            //what if food'\ cant overlaps snake
            while self.snake.overlap(applexx, appleyy)
            {
                applexx = random.gen_range(1, 14);
                appleyy = random.gen_range(1, 14);
            }

            self.applex = applexx;
            self.appley = applexx;
            self.apple_rendered = true;
            self.score +=1;
            self.random_poison_add();

        }

        if self.wait_time > Game_time{
            self.snake.advance(None);
            if self.snake_poisoned()
            {
                self.lose = true;
                self.snake = mysnake::create(3, 3);
                self.apple_rendered = true;
                self.applex = 8;
                self.appley = 8;
                self.lose = false;
                let mut poison: LinkedList<square> = LinkedList::new();
                poison.push_back(square
                {
                    x: 6,
                    y: 7,
                });

                self.poisoned_apples = poison;
                self.wait_time = 0.0;

            }
            self.wait_time = 0.0;
        }

    }



    pub fn key_input(&mut self, key: Key)
    {
       //gameover

       let direction = match key
       {
           Key::Right => Some(Facing::right),
           Key::Up => Some(Facing::up),
           Key::Down => Some(Facing::down),
           Key::Left => Some(Facing::left),
           _=> None
       };


       if direction.unwrap() == self.snake.facing().inverse()
       {
           return;
       }

       let (newx, newy) = self.snake.next_block(direction);
       if !self.snake.overlap(newx, newy){
          self.snake.advance(direction);
          if self.snake_poisoned()
          {
              self.lose = true;
              let mut poison: LinkedList<square> = LinkedList::new();
              poison.push_back(square
              {
                  x: 6,
                  y: 7,
              });

              self.poisoned_apples = poison;
              return;
          }
          if self.apple_rendered && self.applex == newx && self.appley == newy ||
          self.snake.overlap(self.applex, self.appley)
          {

              self.snake.place_tail();
              let mut random = thread_rng();
              let mut applexx = random.gen_range(1, 14);
              let mut appleyy = random.gen_range(1, 14);

              //what if food'\ cant overlaps snake
              while self.snake.overlap(applexx, appleyy)
              {
                  applexx = random.gen_range(1, 14);
                  appleyy = random.gen_range(1, 14);
              }

              self.applex = applexx;
              self.appley = applexx;
              self.apple_rendered = true;
              self.score += 1;
              self.random_poison_add();
              println!("Add the posion");
              self.wait_time = 0.0;

          }



      }
       else{
           self.lose = true;
           self.apple_rendered = false;
       }

    }



}



pub struct mysnake
{
    direction: Facing,
    tail: Option<square>,
    snake: LinkedList<square>,
}

impl mysnake{
    pub fn create(x: i32, y: i32) -> mysnake
    {
          let mut snake: LinkedList<square> = LinkedList::new();

        snake.push_back(
            square
            {
                x: x + 2,
                y,
            }
        );


        snake.push_back(
            square
            {
                x: x + 1,
                y,
            }
        );

        snake.push_back(
            square
            {
                x,
                y,
            }
        );

       mysnake{

           direction: Facing::right,
           snake,
           tail: None,
       }

    }

    pub fn render(&self, g2d: &mut G2d, context: &Context)
    {
        for square in &self.snake
        {
            square_render(COLOR, square.x, square.y, g2d, context);
        }
    }

    pub fn next_block(&self, directional: Option<Facing>) -> (i32, i32)
    {
        let (topx, topy): (i32, i32) = self.head_coordinates();
        let mut advance = self.direction;
        match directional
        {
            Some(d) => advance = d,
            None => {},
        }

        match advance
        {
            Facing::right => (topx + 1, topy),
            Facing::left => (topx - 1, topy),
            Facing::up => (topx, topy - 1),
            Facing::down => (topx, topy + 1),
        }

    }


    pub fn place_tail(&mut self)
    {
        let tailspace = self.tail.clone().unwrap();
        self.snake.push_back(tailspace);

    }

    pub fn head_coordinates(&self) -> (i32, i32)
    {
        let top_square = self.snake.front().unwrap();
        (top_square.x, top_square.y)
    }


    pub fn facing(&self) -> Facing
    {
        self.direction
    }

    pub fn overlap(&self, x: i32, y: i32) -> bool
    {
        let mut counter = 0;
        for square in &self.snake
        {

            if x == square.x && y == square.y
            {
                return true;
            }

         counter = counter + 1;
         if counter == self.snake.len() - 1
         {
             break;


         }
        }



        return false;


    }

    pub fn advance(&mut self, path: Option<Facing>)
    {
         match path
         {
             Some(p) => self.direction = p,
             None => (),
         }

         let(prevx, prevy): (i32, i32) = self.head_coordinates();

         let adv_square = match self.direction
         {
             Facing::right => square
             {
                 x: prevx + 1,
                 y: prevy,
             },

             Facing::left => square
             {
                 x: prevx - 1,
                 y: prevy,
             },


             Facing::up => square
             {
                 x: prevx,
                 y: prevy - 1,
             },


             Facing::down => square
             {
                 x: prevx,
                 y: prevy + 1,
             },
         };

         self.snake.push_front(adv_square);

         let deletedsq = self.snake.pop_back().unwrap();
         self.tail = Some(deletedsq);

    }



}


#[derive(Clone, PartialEq, Copy)]
pub enum Facing
{
    left,
    up,
    right,
    down,
}

#[derive(Debug, Clone)]
pub struct square
{
    x: i32,
    y: i32,
}

impl Facing
{
    pub fn inverse(&self) -> Facing
    {
        match *self
        {
            Facing::left => Facing::right,
            Facing::up => Facing::down,
            Facing::right => Facing::left,
            Facing::down => Facing::up,
        }
    }
}
