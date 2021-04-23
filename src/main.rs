use std::io::stdin;

use board::TicTacToeBoard;
use players::Player;

use crate::board::CellValue;

mod players;
mod board;


fn main() {
    let player1 = Player::new("Jhonny", "Doe", 30, &CellValue::X);
    let player2 = Player::new("Gina", "Day", 36, &CellValue::O);
    let players: [&Player; 2] = [&player1, &player2];

    let mut replay = true;

    while replay {
        let mut game_board: TicTacToeBoard = TicTacToeBoard::new();
        game_board.print_board();
        let mut game_over = false;

        for i in players.iter().cycle().enumerate() {
            let p = i.1;
            p.say_hello();

            println!("Now is {}'s Turn!", p.full_name());

            // read user input ....
            loop {
                let mut txt_in = String::new();
                match stdin().read_line(&mut txt_in) {
                    Ok(_) => {
                        txt_in.pop();
                        let success_move = game_board.add_move(&txt_in, p);
                        if success_move {
                            let winner = game_board.player_wins(p);
                            match winner {
                                None => {}
                                Some(w) => {
                                    game_over = true;
                                    println!("Player {} Wins!", w.full_name());
                                    break;
                                }
                            }
                            game_board.print_board();
                            break;
                        } else {
                            println!("Uhm... You cannot do that!...");
                            game_board.print_board();
                        }
                    }

                    Err(error) => println!("error: {}", error),
                }
            }

            if game_over {
                println!("Game Over!");
                break;
            }
        }
        println!("Play again? (y/n)");
        loop {
            let mut txt_in = String::new();
            match stdin().read_line(&mut txt_in) {
                Ok(_) => {
                    txt_in.pop();

                    if txt_in.to_lowercase() == "n" {
                        replay = false;
                        break;
                    } else if txt_in.to_lowercase() == "y" {
                        break;
                    } else {
                        println!("Not a valid option y/n?")
                    }
                }
                Err(_) => {
                    println!("Something went wrong! :(")
                }
            }
        }
    }
    println!("Good bye!!! :)")
}
