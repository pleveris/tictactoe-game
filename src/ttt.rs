// TTT: module for handleing all the TicTacToe game logic

use rand;
use std::io;

// 3x3 game board (alias-type)
type Board = Vec<Vec<String>>;

// An enum which defines a turn of players (human or bot)
#[derive(Debug, PartialEq)]
enum Turn {
    PlayerHuman,
    PlayerBot,
}

// Main game structure (struct-type)
#[derive(Debug)]
pub struct Game {
    board: Board,
    current_turn: Turn,
}

impl Game {
    // Main construction of a new game happens there
    // The board is constructed of a vector of chars, where possible moves and results of shots taken will be handled.
    // When starting a game, the priority to start goes to human player, but in order for game to be more fun to play, the order could be reversed
    pub fn new() -> Game {
        Game {
            board: vec![
                // For now, 3x3 grid is represented as numbers from 1 to 9 (I hope to improve this in the future)
                vec![String::from("1"), String::from("2"), String::from("3")],
                vec![String::from("4"), String::from("5"), String::from("6")],
                vec![String::from("7"), String::from("8"), String::from("9")],
            ],
            current_turn: Turn::PlayerHuman,
        }
    }

    // Function to start the game
    pub fn play_game(&mut self) {
        let mut complete = false;
        while !complete {
            self.play_turn();
            if self.check_winner() {
                self.output_board();
                match self.current_turn {
                    Turn::PlayerHuman => println!("Wow! You're a winner! Congratulations!"),
                    Turn::PlayerBot => println!("Ouch... It seems a bot played better this time... Try again!"),
                };
                self.restore(); // restoring game states to their defaults if the game is over
                complete = Self::ask_for_playing_again();
            }
            self.current_turn = self.get_next_turn();
        }
    }

    // Plays a turn of the game, getting moves from the player or from the bot.
    fn play_turn(&mut self) {
        self.output_board();
        let (chosen_mark, valid_move) = match self.current_turn {
            Turn::PlayerHuman => (String::from("X"), self.get_player_move()), // Human player mark is going to be X,
            Turn::PlayerBot => (String::from("O"), self.get_bot_move()), // while a bot mark is signed as O.
        };

        let (row, col) = Self::convert_to_board_coords(valid_move);
        self.board[row][col] = chosen_mark;
    }

    // function for board output to the console screen
    fn output_board(&self) {
        println!("\n");
        for row in &self.board {
            println!("{} _\n", row.join(" _ "));
        }
        print!("\n");
    }

    // function to get a moving place from human
    fn get_player_move(&self) -> u32 {
        loop {
            let mut player_choice = String::new();
            println!("Where would you like to put your mark? Please enter a number from 1 to 9: ");
            match io::stdin().read_line(&mut player_choice) {
                Err(_) => println!("Error reading input, try again!"),
                Ok(_) => match self.validate_player_choice(&player_choice) {
                    Err(err) => println!("{}", err),
                    Ok(num) => return num,
                },
            }
        }
    }

    // function for checking if player chooses a correct cell to make a shot at
    fn validate_player_choice(&self, player_choice: &str) -> Result<u32, String> {
        match player_choice.trim().parse::<u32>() {
            Err(_) => Err(String::from("Wrong place chosen, try again!")),
            Ok(number) => {
                if self.is_move_possible(number) {
                    Ok(number)
                } else {
                    Err(String::from(
                        "This cell is already occupied... Try again!",
                    ))
                }
            }
        }
    }

    // function to get a moving place from bot
    fn get_bot_move(&self) -> u32 {
        let mut bot_move: u32 = rand::random::<u32>() % 9 + 1;
        while !self.is_move_possible(bot_move) {
            bot_move = rand::random::<u32>() % 9 + 1;
        }
        println!("Bot made a shot at cell {}", bot_move);
        bot_move
    }

    // function to make sure if a chosen move is possible
    fn is_move_possible(&self, free_move: u32) -> bool {
        match free_move {
            1..=9 => {
                let result_on_board = Self::convert_to_board_coords(free_move);
                match self.board[result_on_board.0][result_on_board.1].as_str() {
                    "X" | "O" => false,
                    _ => true,
                }
            }
            _ => false,
        }
    }

    // function to convert single cell from the input to a 3X3 board representation
    fn convert_to_board_coords(game_move: u32) -> (usize, usize) {
        let row = (game_move - 1) / 3; // Get a number-row like in 3X3 tavle
        let col = (game_move - 1) % 3; // get a number-col like in 3X3 table
        (row as usize, col as usize)
    }

    // function to get the next turn (opposit of current player)
    fn get_next_turn(&self) -> Turn {
        match self.current_turn {
            Turn::PlayerHuman => Turn::PlayerBot,
            Turn::PlayerBot => Turn::PlayerHuman,
        }
    }

    // function to check the winner
    fn check_winner(&self) -> bool {
        let mut vertical = false; // Checks for shots in vertical position
        let mut horizontal = false; // checks for shots in horizontal position
        for item in 0..3 {
            vertical |= self.board[item][0] == self.board[item][1]
                && self.board[item][1] == self.board[item][2];
            horizontal |= self.board[0][item] == self.board[1][item]
                && self.board[1][item] == self.board[2][item];
        }
        // checking for diagonal positions
        let diagonal_part1 =
            self.board[0][0] == self.board[1][1] && self.board[1][1] == self.board[2][2];
        let diagonal_part2 =
            self.board[0][2] == self.board[1][1] && self.board[1][1] == self.board[2][0];
        vertical || horizontal || diagonal_part1 || diagonal_part2
    }

    // function to ask if player would like to play the game again
    fn ask_for_playing_again() -> bool {
        let mut player_choice = String::new();
        println!("Do you wish to play the game again? Type [y/n] to choose.");
        match io::stdin().read_line(&mut player_choice) {
            Ok(_) => {
                let answer = player_choice.to_lowercase();
                answer.trim() == "n"
            }
            Err(_) => false
        }
    }

    // Function to restore the default starting player, and cleen the board
    fn restore(&mut self) {
        self.current_turn = Turn::PlayerHuman;
        self.board = vec![
            vec![String::from("1"), String::from("2"), String::from("3")],
            vec![String::from("4"), String::from("5"), String::from("6")],
            vec![String::from("7"), String::from("8"), String::from("9")],
        ];
    }
}
