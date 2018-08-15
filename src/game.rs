use piston_window::keyboard::Key;
use std::collections::LinkedList;
use piston_window::{Context, G2d};
use piston_window::types::Color;
use renderings::square_render;
use rand::{thread_rng, Rng};
const COLOR: Color = [0.00, 0.80, 0.00, 1.0];
const APPLE_COLOR: Color = [0.80, 0.00, 0.00, 1.0];
const POISONED_APPLE_COLOR: Color = [0.5, 0.0, 0.00, 1.0];
const GAME_TIME: f64 = 3.0;
const RESTART_TIME: f64 = 1.0;


pub struct Game
{
    snake: Mysnake,
    lose: bool,
    wait_time: f64,
    apple_rendered: bool,
    applex: i32,
    appley: i32,
    poisoned_apples: LinkedList<Square>,
    score: i32,


}


impl Game
{
    pub fn new() -> Game
    {


       let mut poison: LinkedList<Square> = LinkedList::new();
       poison.push_back(Square
       {
           x: 6,
           y: 7,
       });


        Game
        {

            snake: Mysnake::create(3,3),
            lose: false,
            wait_time: 0.0,
            apple_rendered: true,
            applex: 3,
            appley: 5,
            poisoned_apples: poison,
            score: 0,
        }
    }




    pub fn random_poison_add(&mut self)
    {

        let mut random = thread_rng();
        let mut applexx = random.gen_range(1, 14);
        let mut appleyy = random.gen_range(1, 14);

        //what if food'\ cant overlaps snake
        while self.snake.overlap(applexx, appleyy)
        {
            applexx = random.gen_range(1, 14);
            appleyy = random.gen_range(1, 14);
        }



        let pois = Square
        {
            x: applexx,
            y: appleyy,
        };

        self.poisoned_apples.push_back(pois);



    }

    pub fn render_poison(&self, g2d: &mut G2d, context: &Context)
    {
        for square in &self.poisoned_apples
        {

            square_render(POISONED_APPLE_COLOR, square.x, square.y, g2d, context);

        }
    }

    pub fn drawgame(&self, context: &Context, g2d: &mut G2d)
    {

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




        if self.lose
        {
            println!("GAME OVER. Your score is {}", self.score);
            if self.wait_time > RESTART_TIME
            {
                self.snake = Mysnake::create(3, 3);
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

        if self.wait_time > GAME_TIME{
            self.snake.advance(None);
            if self.snake_poisoned()
            {
                self.lose = true;
                println!("GAME OVER. Your score is {}", self.score);
                self.snake = Mysnake::create(3, 3);
                self.apple_rendered = true;
                self.applex = 8;
                self.appley = 8;
                self.lose = false;
                let mut poison: LinkedList<Square> = LinkedList::new();
                poison.push_back(Square
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
              println!("GAME OVER. Your score is {}", self.score);
              let mut poison: LinkedList<Square> = LinkedList::new();
              poison.push_back(Square
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
           println!("Your score is {}", self.score);
           self.apple_rendered = false;
       }

    }



}



pub struct Mysnake
{
    direction: Facing,
    tail: Option<Square>,
    snake: LinkedList<Square>,
}

impl Mysnake{
    pub fn create(x: i32, y: i32) -> Mysnake
    {
          let mut snake: LinkedList<Square> = LinkedList::new();

        snake.push_back(
            Square
            {
                x: x + 2,
                y,
            }
        );


        snake.push_back(
            Square
            {
                x: x + 1,
                y,
            }
        );

        snake.push_back(
            Square
            {
                x,
                y,
            }
        );

       Mysnake{

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
             Facing::right => Square
             {
                 x: prevx + 1,
                 y: prevy,
             },

             Facing::left => Square
             {
                 x: prevx - 1,
                 y: prevy,
             },


             Facing::up => Square
             {
                 x: prevx,
                 y: prevy - 1,
             },


             Facing::down => Square
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


#[derive(Clone, PartialEq, Copy, Debug)]
pub enum Facing
{
    left,
    up,
    right,
    down,
}

#[derive(Debug, Clone)]
pub struct Square
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


//snake shouldnt be poisoned at the start of a game
#[test]
fn test_snake_poisoned_true()
{
    let mygame = Game::new();
    assert_eq!(mygame.snake_poisoned(), false);

}

//our snake is originally created at coordinates 3, 3 so function should overlap
#[test]
fn test_for_overlap()
{

    let mygame = Game::new();
    assert_eq!(mygame.snake.overlap(3, 3), true);

}

//snake should be facing right when game begins
#[test]
fn test_facing_direction()
{
    let mygame = Game::new();
    assert_eq!(mygame.snake.facing(), Facing::right);


}


#[test]
fn three_snake_squares()
{
    let mysnake = Mysnake::create(3 , 3);
    assert_eq!(mysnake.snake.len(), 3);

}

//head should be at 5,3 coordinates when beginning
#[test]
fn beginning_head_coordinates()
{

    let mysnake = Mysnake::create(3, 3);
    assert_eq!(mysnake.head_coordinates(), (5, 3));


}
//next direction towards the right when starting off
#[test]
fn next_block_to_travel()
{
    let mysnake = Mysnake::create(3,3);
    assert_eq!(mysnake.next_block(Some(Facing::right)), (6 , 3));

}
