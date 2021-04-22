use core::fmt;
use std::fmt::{Display, Formatter};

use crate::players::Player;

#[derive(PartialEq, Eq)]
pub enum CellValue {
    Blank,
    X,
    O,
}

impl Display for CellValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let tick = match self {
            CellValue::Blank => { " " }
            CellValue::X => { "X" }
            CellValue::O => { "O" }
        };
        write!(f, "{}", tick)
    }
}

pub struct TicTacToeBoard<'b> {
    a1: &'b CellValue,
    a2: &'b CellValue,
    a3: &'b CellValue,

    b1: &'b CellValue,
    b2: &'b CellValue,
    b3: &'b CellValue,

    c1: &'b CellValue,
    c2: &'b CellValue,
    c3: &'b CellValue,
}

impl<'c> TicTacToeBoard<'c> {
    pub fn new() -> TicTacToeBoard<'c> {
        TicTacToeBoard {
            a1: &CellValue::Blank,
            a2: &CellValue::Blank,
            a3: &CellValue::Blank,
            b1: &CellValue::Blank,
            b2: &CellValue::Blank,
            b3: &CellValue::Blank,
            c1: &CellValue::Blank,
            c2: &CellValue::Blank,
            c3: &CellValue::Blank,
        }
    }

    pub fn get_display_value(cell: &CellValue) -> &'static str {
        match cell {
            CellValue::Blank => { " " }
            CellValue::X => { "X" }
            CellValue::O => { "O" }
        }
    }


    pub fn add_move(&mut self, position: &str, player: &'c Player) -> bool {
        return match position {
            "a1" => {
                if self.a1 == &CellValue::Blank {
                    self.a1 = player.tick;
                    true
                } else { false }
            }

            "a2" => {
                if self.a2 == &CellValue::Blank {
                    self.a2 = player.tick;
                    true
                } else { false }
            }
            "a3" => {
                if self.a3 == &CellValue::Blank {
                    self.a3 = player.tick;
                    true
                } else { false }
            }
            "b1" => {
                if self.b1 == &CellValue::Blank {
                    self.b1 = player.tick;
                    true
                } else { false }
            }
            "b2" => {
                if self.b2 == &CellValue::Blank {
                    self.b2 = player.tick;
                    true
                } else { false }
            }
            "b3" => {
                if self.b3 == &CellValue::Blank {
                    self.b3 = player.tick;
                    true
                } else { false }
            }
            "c1" => {
                if self.c1 == &CellValue::Blank {
                    self.c1 = player.tick;
                    true
                } else { false }
            }
            "c2" => {
                if self.c2 == &CellValue::Blank {
                    self.c2 = player.tick;
                    true
                } else { false }
            }
            "c3" => {
                if self.c3 == &CellValue::Blank {
                    self.c3 = player.tick;
                    true
                } else { false }
            }
            _ => { false }
        };
    }


    pub fn player_wins(&self, player: &'c Player) -> Option<&'c Player> {
        let board_array: [&CellValue; 9] = [
            self.a1,
            self.a2,
            self.a3,
            self.b1,
            self.b2,
            self.b3,
            self.c1,
            self.c2,
            self.c3,
        ];


        println!("Player tick: {:?}", player.get_tick());

        for n in 0..3 {
            let mut r1 = board_array[n];
            let mut r2 = board_array[n + 3];
            let mut r3 = board_array[n + 6];


            if r1 == r2 && r2 == r3 && !(r3 == &CellValue::Blank) {
                return Some(&player);
            } else {
                r1 = board_array[n];
                r2 = board_array[n + 1];
                r3 = board_array[n + 2];

                // Checking for line win...
                if r1 == r2 && r2 == r3 && !(r3 == &CellValue::Blank) {
                    return Some(&player);
                }
            }
        }

        let mut r1 = board_array[0];
        let mut r2 = board_array[4];
        let mut r3 = board_array[8];
        if r1 == r2 && r2 == r3 && !(r3 == &CellValue::Blank) {
            return Some(&player);
        }


        r1 = board_array[2];
        r2 = board_array[4];
        r3 = board_array[5];
        if r1 == r2 && r2 == r3 && !(r3 == &CellValue::Blank) {
            return Some(&player);
        }
        None
    }

    pub fn print_board(&self) {
        let head = "   1 2 3 \n";
        let div = "  _______\n";
        let ln1 = format!("a |{}|{}|{}|\n",
                          TicTacToeBoard::get_display_value(&self.a1),
                          TicTacToeBoard::get_display_value(&self.a2),
                          TicTacToeBoard::get_display_value(&self.a3));
        let ln2 = format!("b |{}|{}|{}|\n",
                          TicTacToeBoard::get_display_value(&self.b1),
                          TicTacToeBoard::get_display_value(&self.b2),
                          TicTacToeBoard::get_display_value(&self.b3));
        let ln3 = format!("c |{}|{}|{}|\n",
                          TicTacToeBoard::get_display_value(&self.c1),
                          TicTacToeBoard::get_display_value(&self.c2),
                          TicTacToeBoard::get_display_value(&self.c3));

        let mut board: String = String::new();

        board.push_str(head);
        board.push_str(div);
        board.push_str(ln1.as_str());
        board.push_str(div);
        board.push_str(ln2.as_str());
        board.push_str(div);
        board.push_str(ln3.as_str());
        board.push_str(div);

        println!("{}", board);
    }
}