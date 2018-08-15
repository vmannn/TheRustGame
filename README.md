# TheRustGame (Snake game with a twist)

This is a simple snake game implemented with an interesting twist. The game features the traditional style in which one
travels as a snake and collects apples. The twist is that there are poisonous apples as well which are designed to be very similar to 
the original apple. The only difference is a small tint in color so to play the game, you mus keep a sharp eye open. As you collect the regular apples, more and more poisonous apples appear on the screen. The score is currently displayed in the terminal. A fun challenge.

Use the up, down and side arrows to play. Snake moves very slow by itself allowing optimal precision when swerving through poisonous apples. There are no borders as it is already difficult enough to dodge the bad apples.

<img src="https://github.com/vmannn/TheRustGame/blob/master/theworm.gif" width="550" height="300" />



# Dependencies and how to run game

This project was created using Piston, a 2D graphics tool available in rust. Below are the crates used:

### dependencie

rand = "0.3.18"
piston_window = "0.74.0"

### How to play it
clone the repository, cd into the directory from a terminal or IDE and simply type "cargo run" to play.


# Future plans

Make the score onscreen. I would also like to seperate the game by levels. Perhaps more poisonous apples will appear at a time. Maybe 
even moving apples. Shortage of time was a factor in class, but I had fun working on my first game in a new language. I plan to continue learning more about Rust in the future.




