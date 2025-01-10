use std::cmp;

use crate::game::opposite_color;
use crate::game::Color;
use crate::game::GameState;

const BINOM_PROB: [f64; 5] = [0.0625, 0.25, 0.375, 0.25, 0.0625];

fn eval_move(
    state: &GameState,
    color: Color,
    roll: i32,
    to_eval: i32,
    ply_depth: usize,
    max_depth: usize,
) -> f64 {
    let mut new_state = state.clone();
    let second_turn = if to_eval == -2 {
        // We use -2 as special code for "this turn got skipped, leave the game as-is."
        false
    } else {
        new_state.exec_move(color, roll, to_eval)
    };

    if ply_depth == max_depth {
        // Base case, just give the current score.
        (new_state.finished[color as usize] as f64)
            - (new_state.finished[opposite_color(color) as usize] as f64)
    } else {
        let mut ret = 0.0;

        // If you land on a rosette, then you get to go again.
        let next_color = if second_turn {
            color
        } else {
            opposite_color(color)
        };

        // What's good for our opponent is bad for us, and vice versa.
        let sign = if next_color == color { 1.0 } else { -1.0 };

        for roll in 0..5 {
            let mut legal_moves = new_state.get_legal_moves(next_color, roll);

            if legal_moves.len() == 0 {
                // If you have no legal moves then your turn gets skipped. Signal this with move -2.
                ret += sign
                    * BINOM_PROB[roll as usize]
                    * eval_move(&new_state, next_color, roll, -2, ply_depth + 1, max_depth);
            } else {
                let mut move_scores: Vec<f64> = vec![];
                for legal_move in legal_moves {
                    move_scores.push(eval_move(
                        &new_state,
                        next_color,
                        roll,
                        legal_move,
                        ply_depth + 1,
                        max_depth,
                    ));
                }
                ret += sign
                    * BINOM_PROB[roll as usize]
                    * move_scores.into_iter().fold(f64::MIN, f64::max);
            }
        }

        ret
    }
}

pub fn select_ai_move(state: &GameState, color: Color, roll: i32, max_depth: usize) -> i32 {
    let legal_moves = state.get_legal_moves(color, roll);
    if legal_moves.len() == 0 {
        return 0;
    }

    let mut best_move = legal_moves[0];
    let mut best_score = f64::MIN;

    for legal_move in legal_moves {
        let score = eval_move(&state, color, roll, legal_move, 0, max_depth);
        if score > best_score {
            best_score = score;
            best_move = legal_move;
        }
    }

    println!("Selecting move with score {best_score}");
    best_move
}
