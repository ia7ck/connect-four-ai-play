use ::std::fmt;

use ::serde::{Deserialize, Serialize};

use ::thunder_book_game_search::game::alternate::{AlternateGameState, WinningStatus};

#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct DropPiece {
    y: usize,
    x: usize,
}

impl DropPiece {
    fn new(y: usize, x: usize) -> Self {
        Self { y, x }
    }
}

#[derive(Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum PiecePutBy {
    First,
    Second,
}

#[derive(Clone)]
pub struct ConnectFourState {
    h: usize,
    w: usize,
    board: Vec<Vec<Option<PiecePutBy>>>,
    winning_status_cache: Option<WinningStatus>,
    turn: usize,
}

impl ConnectFourState {
    pub fn new(h: usize, w: usize) -> Self {
        Self {
            h,
            w,
            board: vec![vec![None; w]; h],
            winning_status_cache: None,
            turn: 0,
        }
    }
    pub fn from_state(
        h: usize,
        w: usize,
        board: Vec<Vec<Option<PiecePutBy>>>,
        turn: usize,
    ) -> Self {
        Self {
            h,
            w,
            board,
            winning_status_cache: None, // 実際は勝敗決まってるかもしれないのでboardを見たほうがいい
            turn,
        }
    }
    pub fn h(&self) -> usize {
        self.h
    }
    pub fn w(&self) -> usize {
        self.w
    }
    pub fn turn(&self) -> usize {
        self.turn
    }
    pub fn get_board(&self) -> Vec<Vec<Option<PiecePutBy>>> {
        self.board.clone()
    }
}

impl AlternateGameState for ConnectFourState {
    type Action = DropPiece;

    fn legal_actions(&self) -> Vec<Self::Action> {
        let mut actions = Vec::new();
        for x in 0..self.w {
            if let Some(y) = (0..self.h).rev().find(|&y| self.board[y][x].is_none()) {
                actions.push(DropPiece::new(y, x));
            }
        }
        actions
    }

    fn advance(&mut self, action: Self::Action) {
        let (y, x) = (action.y, action.x);
        assert!(self.board[y][x].is_none());

        let player = if self.turn % 2 == 0 {
            PiecePutBy::First
        } else {
            PiecePutBy::Second
        };
        self.board[y][x] = Some(player);
        let my_piece = |target| target == Some(player);

        let left = || (0..=x).rev();
        let right = || x..self.w;
        let up = || (0..=y).rev();
        let down = || y..self.h;

        // 横
        let yoko = {
            let left = left().take_while(|&x| my_piece(self.board[y][x])).count();
            let right = right().take_while(|&x| my_piece(self.board[y][x])).count();
            left + right - 1
        };
        // 左上から右下
        let naname = {
            let upper_left = up()
                .clone()
                .zip(left())
                .take_while(|&(y, x)| my_piece(self.board[y][x]))
                .count();
            let lower_right = down()
                .zip(right())
                .take_while(|&(y, x)| my_piece(self.board[y][x]))
                .count();
            upper_left + lower_right - 1
        };
        // 右上から左下
        let menana = {
            let upper_right = up()
                .zip(right())
                .take_while(|&(y, x)| my_piece(self.board[y][x]))
                .count();
            let lower_left = down()
                .zip(left())
                .take_while(|&(y, x)| my_piece(self.board[y][x]))
                .count();
            upper_right + lower_left - 1
        };
        // 縦
        let tate = down()
            .take_while(|&y| my_piece(self.board[y][action.x]))
            .count();

        if yoko >= 4 || naname >= 4 || menana >= 4 || tate >= 4 {
            // 今回駒が揃ったので次に打つ側の負け
            self.winning_status_cache = Some(WinningStatus::Lose);
        } else if self.legal_actions().is_empty() {
            self.winning_status_cache = Some(WinningStatus::Draw);
        }
        self.turn += 1;
    }

    fn done(&self) -> bool {
        self.winning_status_cache.is_some()
    }

    fn score(&self) -> i16 {
        unimplemented!()
    }

    fn winning_status(&self) -> Option<WinningStatus> {
        self.winning_status_cache
    }
}

// 先手を x, 後手を o で表示する
impl fmt::Debug for ConnectFourState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in 0..self.h {
            for x in 0..self.w {
                let c = match self.board[y][x] {
                    Some(PiecePutBy::First) => 'x',
                    Some(PiecePutBy::Second) => 'o',
                    None => '.',
                };
                write!(f, "{}", c)?;
            }
            if y + 1 < self.h {
                writeln!(f)?;
            }
        }
        Ok(())
    }
}
