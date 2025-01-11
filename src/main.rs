use std::collections::HashMap;
use std::io;

use rand::{thread_rng, Rng};

use crate::game::opposite_color;
use crate::game::Color;
use crate::game::GameState;
use crate::minmax::minmax_select_move;
use crate::q_learn::get_q_matrix;
use crate::q_learn::q_select_move;

mod game;
mod minmax;
mod q_learn;

const NUM_GAMES: usize = 100;

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

fn main() {
    let q_matrix = get_q_matrix();

    let mut black_wins = 0;
    let mut white_wins = 0;
    for _i in 0..NUM_GAMES {
        let mut state = GameState::new();
        let mut rng = thread_rng();
        let q_ai = Color::Black;
        let minmax_ai = Color::White;
        let mut turn = Color::Black;
        let mut is_first_turn = true;
        'game_loop: loop {
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
                let chosen_move = if turn == q_ai {
                    q_select_move(
                        &state, roll, q_ai, &q_matrix, /*explore_probability=*/ 0.0,
                    )
                    .0
                } else {
                    minmax_select_move(&state, minmax_ai, roll, /*max_depth=*/ 5)
                };
                if state.exec_move(turn, roll, chosen_move) {
                    // Roll again
                    turn = opposite_color(turn);
                }
            }
            turn = opposite_color(turn);
            if let Some(winner) = state.get_winner() {
                println!("{winner} wins!");
                match winner {
                    Color::Black => black_wins += 1,
                    Color::White => white_wins += 1,
                }
                break;
            }
        }
    }

    println!("Black wins: {}\nWhite wins: {}", black_wins, white_wins);
}
