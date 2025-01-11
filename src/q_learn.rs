use std::collections::HashMap;
use std::fs::File;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::io::Read;
use std::io::Write;
use std::path::Path;

use rand::{thread_rng, Rng};

use crate::game::opposite_color;
use crate::game::Color;
use crate::game::GameState;

const LEARNING_RATE: f32 = 0.1;
const GAMMA: f32 = 0.99;
const INIT_EXPLORATION_PROB: f32 = 1.0;
const LAMBDA: f32 = 0.00000005;
const TRAINING_GAMES: usize = 60000000;
//const LAMBDA: f32 = 0.000003;
//const TRAINING_GAMES: usize = 1000000;

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
struct QState {
    board: GameState,
    roll: i32,
    turn: Color,
}

pub fn q_select_move(
    state: &GameState,
    roll: i32,
    color: Color,
    q_matrix: &HashMap<u64, [f32; 5]>,
    exploration_prob: f32,
) -> (i32, usize) {
    let mut legal_moves = state.get_legal_moves(color, roll);
    if legal_moves.len() == 0 {
        return (-2, 0);
    }

    legal_moves.sort();

    let mut rng = thread_rng();
    if rng.gen_bool(exploration_prob as f64) {
        let move_idx = rng.gen_range(0..(legal_moves.len()));
        (legal_moves[move_idx], move_idx)
    } else {
        let mut hasher = DefaultHasher::new();
        let q_state = QState {
            board: state.clone(),
            roll: roll,
            turn: color,
        };
        q_state.hash(&mut hasher);
        let hash = hasher.finish();
        let mut max_score = f32::MIN;
        let mut max_idx = 0;
        let q_matrix_row = q_matrix
            .get(&hash)
            .cloned()
            .unwrap_or([0.0, 0.0, 0.0, 0.0, 0.0]);
        for idx in 0..(legal_moves.len()) {
            if q_matrix_row[idx] > max_score {
                max_idx = idx;
                max_score = q_matrix_row[idx];
            }
        }
        if exploration_prob == 0.0 {
            println!("{q_matrix_row:?}");
            println!("Selecting move idx {} ({})", max_idx, legal_moves[max_idx]);
        }
        (legal_moves[max_idx], max_idx)
    }
}

fn self_play_game(q_matrix: &mut HashMap<u64, [f32; 5]>, exploration_prob: f32) {
    let mut history: Vec<(QState, usize)> = vec![];

    let mut state = GameState::new();
    let mut rng = thread_rng();
    let mut turn = Color::Black;
    let mut is_first_turn = true;

    let winner = loop {
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

        let (selected_move, move_idx) =
            q_select_move(&state, roll, turn, q_matrix, exploration_prob);
        if selected_move != -2 {
            history.push((
                QState {
                    board: state.clone(),
                    roll: roll,
                    turn: turn,
                },
                move_idx,
            ));

            if state.exec_move(turn, roll, selected_move) {
                turn = opposite_color(turn);
            }
        }
        turn = opposite_color(turn);

        if let Some(winner) = state.get_winner() {
            break winner;
        }
    };

    for step in history {
        let reward = state.finished[step.0.turn as usize] as f32
            - state.finished[opposite_color(step.0.turn) as usize] as f32;
        let mut hasher = DefaultHasher::new();
        step.0.hash(&mut hasher);
        let hash = hasher.finish();
        let mut q_matrix_row = q_matrix
            .get(&hash)
            .cloned()
            .unwrap_or([0.0, 0.0, 0.0, 0.0, 0.0]);
        q_matrix_row[step.1] = (1.0 - LEARNING_RATE) * q_matrix_row[step.1]
            + LEARNING_RATE * (reward + GAMMA * q_matrix_row.into_iter().fold(f32::MIN, f32::max));
        q_matrix.insert(hash, q_matrix_row);
    }
}

fn train(q_matrix: &mut HashMap<u64, [f32; 5]>) {
    let mut exploration_prob = INIT_EXPLORATION_PROB;
    for i in 0..TRAINING_GAMES {
        if i % (TRAINING_GAMES / 100) == 0 {
            println!(
                "Training: {}%    Exploration prob: {}",
                i as f32 / TRAINING_GAMES as f32 * 100.0,
                exploration_prob
            );
        }
        self_play_game(q_matrix, exploration_prob);
        exploration_prob =
            INIT_EXPLORATION_PROB * std::f32::consts::E.powf(-1.0 * LAMBDA * (i as f32));
    }

    println!("Done training! {} entries in q_matrix", q_matrix.len());
}

pub fn get_q_matrix() -> HashMap<u64, [f32; 5]> {
    let mut ret: HashMap<u64, [f32; 5]> = HashMap::new();

    let cache_path = Path::new("q_matrix_cache");
    match File::open(&cache_path) {
        Ok(mut q_matrix_cache) => {
            let mut key_buf: [u8; 8] = [0, 0, 0, 0, 0, 0, 0, 0];
            let mut value_buf: [u8; 4] = [0, 0, 0, 0];
            let mut key: u64 = 0;
            let mut value: [f32; 5] = [0.0, 0.0, 0.0, 0.0, 0.0];
            'read_loop: loop {
                if q_matrix_cache.read_exact(&mut key_buf).is_err() {
                    break 'read_loop;
                }
                key = u64::from_le_bytes(key_buf);
                for i in 0..value.len() {
                    if q_matrix_cache.read_exact(&mut value_buf).is_err() {
                        break 'read_loop;
                    }
                    value[i] = f32::from_le_bytes(value_buf);
                }
                ret.insert(key, value.clone());
            }
        }
        Err(_) => {
            let mut q_matrix_cache = File::create(&cache_path).unwrap();

            train(&mut ret);

            for (key, val) in ret.iter() {
                q_matrix_cache.write_all(&u64::to_le_bytes(*key)).unwrap();
                for entry in val {
                    q_matrix_cache.write_all(&f32::to_le_bytes(*entry)).unwrap();
                }
            }
        }
    };

    ret
}
