use std::io;
use std::process::ExitCode;

use rand::{thread_rng, Rng};

use crate::ai::select_ai_move;
use crate::game::opposite_color;
use crate::game::Color;
use crate::game::GameState;

mod ai;
mod game;

fn get_user_move(legal_moves: Vec<i32>) -> i32 {
    'move_select: loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        match input.trim().parse() {
            Ok(number) => {
                for legal_move in legal_moves.iter() {
                    if *legal_move == number {
                        break 'move_select number;
                    }
                }
                println!("Please enter a legal move");
            }
            Err(_) => println!("Please enter a number"),
        };
    }
}

fn main() -> ExitCode {
    let mut state = GameState::new();
    let mut rng = thread_rng();
    //    let human = Color::Black;
    let dumb_ai = Color::Black;
    let smart_ai = Color::White;
    let mut turn = Color::White;
    let mut is_first_turn = true;
    loop {
        println!("{state}");
        println!("Turn: {turn}");

        let mut roll = 0;
        let num_dice = if is_first_turn {
            is_first_turn = false;
            2
        } else {
            4
        };
        for _i in 0..num_dice {
            roll += rng.gen_range(0..2);
        }
        println!("Roll: {roll}");

        let legal_moves = state.get_legal_moves(turn, roll);
        if legal_moves.len() != 0 {
            //            let chosen_move = if turn == human {
            //                get_user_move(legal_moves)
            //            } else {
            let chosen_move = if turn == dumb_ai {
                select_ai_move(&state, dumb_ai, roll, 2)
            } else {
                select_ai_move(&state, smart_ai, roll, 5)
            };
            if state.exec_move(turn, roll, chosen_move) {
                // Roll again
                turn = opposite_color(turn);
            }
        }
        turn = opposite_color(turn);
        if let Some(winner) = state.get_winner() {
            println!("{winner} wins!");
            return ExitCode::from(match winner {
              Color::Black => 0,
              Color::White => 1,
            });
        }
    }
}
