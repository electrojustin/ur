use std::fmt;
use std::vec;

pub const MAX_PIECES: u8 = 5;
pub const SHARE_TRACK_START: i32 = 4;
pub const SHARE_TRACK_END: i32 = 12;
pub const FIRST_ROSE: i32 = 3;
pub const SHARED_ROSE: i32 = 7;
pub const LAST_ROSE: i32 = 13;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Color {
    Black = 0,
    White = 1,
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Color::Black => "Black",
                Color::White => "White",
            }
        )
    }
}

pub fn opposite_color(color: Color) -> Color {
    match color {
        Color::Black => Color::White,
        Color::White => Color::Black,
    }
}

#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct GameState {
    pub pending: [u8; 2],
    pub finished: [u8; 2],
    pub tracks: [[bool; LAST_ROSE as usize + 1]; 2],
}

impl GameState {
    pub fn new() -> Self {
        GameState {
            pending: [MAX_PIECES, MAX_PIECES],
            ..Default::default()
        }
    }

    pub fn get_winner(&self) -> Option<Color> {
        if self.finished[Color::Black as usize] == MAX_PIECES {
            Some(Color::Black)
        } else if self.finished[Color::White as usize] == MAX_PIECES {
            Some(Color::White)
        } else {
            None
        }
    }

    pub fn get_legal_moves(&self, color: Color, roll: i32) -> Vec<i32> {
        let mut ret: Vec<i32> = Vec::new();

        if roll == 0 {
            return ret;
        }

        if self.pending[color as usize] != 0 {
            if !self.tracks[color as usize][(roll - 1) as usize] {
                ret.push(-1);
            }
        }

        for piece in 0..(LAST_ROSE + 1) {
            if !self.tracks[color as usize][piece as usize] {
                continue;
            }

            if piece + roll == LAST_ROSE + 1 {
                ret.push(piece);
            } else if piece + roll == SHARED_ROSE {
                if !self.tracks[0][(piece + roll) as usize]
                    && !self.tracks[1][(piece + roll) as usize]
                {
                    ret.push(piece);
                }
            } else if piece + roll <= LAST_ROSE
                && !self.tracks[color as usize][(piece + roll) as usize]
            {
                ret.push(piece);
            }
        }

        ret
    }

    // Returns true if we should roll again.
    pub fn exec_move(&mut self, color: Color, roll: i32, to_exec: i32) -> bool {
        if to_exec == -1 {
            self.tracks[color as usize][(roll - 1) as usize] = true;
            self.pending[color as usize] -= 1;

            roll - 1 == FIRST_ROSE
        } else if roll + to_exec == LAST_ROSE + 1 {
            self.tracks[color as usize][to_exec as usize] = false;
            self.finished[color as usize] += 1;
            false
        } else {
            self.tracks[color as usize][to_exec as usize] = false;
            self.tracks[color as usize][(roll + to_exec) as usize] = true;
            if roll + to_exec >= SHARE_TRACK_START
                && roll + to_exec < SHARE_TRACK_END
                && self.tracks[opposite_color(color) as usize][(roll + to_exec) as usize]
            {
                self.tracks[opposite_color(color) as usize][(roll + to_exec) as usize] = false;
                self.pending[opposite_color(color) as usize] += 1;
            }

            roll + to_exec == FIRST_ROSE
                || roll + to_exec == SHARED_ROSE
                || roll + to_exec == LAST_ROSE
        }
    }
}

impl fmt::Display for GameState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let black_pattern: [usize; 14] = [3, 2, 1, 0, 8, 9, 10, 11, 12, 13, 14, 15, 7, 6];
        let white_pattern: [usize; 14] = [19, 18, 17, 16, 8, 9, 10, 11, 12, 13, 14, 15, 23, 22];
        let mut out_buf = [
            '*', '#', '#', '#', ' ', ' ', '*', '#', '#', '#', '#', '*', '#', '#', '#', '#', '*',
            '#', '#', '#', ' ', ' ', '*', '#',
        ];
        for i in 0..(LAST_ROSE as usize + 1) {
            if self.tracks[Color::Black as usize][i] {
                out_buf[black_pattern[i]] = 'B';
            }
            if self.tracks[Color::White as usize][i] {
                out_buf[white_pattern[i]] = 'W';
            }
        }
        write!(
            f,
            "{}\n{}\n{}\nBlack: {}   White: {}",
            out_buf[..8].iter().collect::<String>(),
            out_buf[8..16].iter().collect::<String>(),
            out_buf[16..].iter().collect::<String>(),
            self.finished[Color::Black as usize],
            self.finished[Color::White as usize]
        )
    }
}
