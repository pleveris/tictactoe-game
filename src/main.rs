// Main: main file for application to run

extern crate rand;

mod ttt;

use ttt::Game;

fn main() {
    println!("Welcome to TicTacToe! Have fun! :-)");
    let mut game = Game::new();
    game.play_game();
}
