mod state;

use ::serde::{Deserialize, Serialize};
use ::wasm_bindgen::{prelude::wasm_bindgen, JsValue};

use ::thunder_book_game_search::{
    game::alternate::{AlternateGameState, WinningStatus},
    search::alternate::{mcts::MCTS, ChooseAction},
};

use crate::state::{ConnectFourState, DropPiece, PiecePutBy};

#[allow(unused_macros)]
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "kind", content = "action")]
enum Hand {
    Human(DropPiece),
    Ai,
}

#[derive(Serialize, Deserialize)]
enum Status {
    LastPlayerWin,
    Draw,
    Ongoing,
}

#[derive(Serialize, Deserialize)]
struct Data {
    h: usize,
    w: usize,
    board: Vec<Vec<Option<PiecePutBy>>>,
    turn: usize,
    status: Status,
}

// ううううう
impl From<ConnectFourState> for Data {
    fn from(s: ConnectFourState) -> Self {
        Self {
            h: s.h(),
            w: s.w(),
            board: s.get_board(),
            turn: s.turn(),
            status: match s.winning_status() {
                Some(WinningStatus::Win) => unreachable!(),
                Some(WinningStatus::Lose) => Status::LastPlayerWin,
                Some(WinningStatus::Draw) => Status::Draw,
                None => Status::Ongoing,
            },
        }
    }
}

impl From<Data> for ConnectFourState {
    fn from(data: Data) -> Self {
        Self::from_state(data.h, data.w, data.board, data.turn)
    }
}

#[wasm_bindgen(skip_typescript)]
pub fn start(h: usize, w: usize) -> Result<JsValue, JsValue> {
    let game_state = ConnectFourState::new(h, w);
    let data = Data::from(game_state);
    Ok(serde_wasm_bindgen::to_value(&data)?)
}

#[wasm_bindgen(skip_typescript)]
pub fn legal_actions(data: JsValue) -> Result<JsValue, JsValue> {
    let data = serde_wasm_bindgen::from_value::<Data>(data)?;
    assert!(matches!(data.status, Status::Ongoing));
    let game_state = ConnectFourState::from(data);
    let legal_actioins = game_state.legal_actions();
    Ok(serde_wasm_bindgen::to_value(&legal_actioins)?)
}

#[wasm_bindgen(skip_typescript)]
pub fn advance(data: JsValue, hand: JsValue) -> Result<JsValue, JsValue> {
    let data = serde_wasm_bindgen::from_value::<Data>(data)?;
    let hand = serde_wasm_bindgen::from_value::<Hand>(hand)?;
    assert!(matches!(data.status, Status::Ongoing));
    let mut game_state = ConnectFourState::from(data);
    let action = match hand {
        Hand::Human(action) => action,
        Hand::Ai => {
            // 制限時間を指定できたほうがいい
            let mcts = MCTS::new(2000);
            mcts.choose(&game_state)
        }
    };
    game_state.advance(action);
    let data = Data::from(game_state);
    Ok(serde_wasm_bindgen::to_value(&data)?)
}

#[wasm_bindgen(skip_typescript)]
pub fn debug(data: JsValue) -> Result<String, JsValue> {
    let data = serde_wasm_bindgen::from_value::<Data>(data)?;
    let game_state = ConnectFourState::from(data);
    Ok(format!("{game_state:?}"))
}

// えええええ
#[wasm_bindgen(typescript_custom_section)]
const TS_APPEND_CONTENT: &'static str = r#"

export type DropPiece = Readonly<{ y: number, x: number }>;
export type PiecePutBy = "First" | "Second";
export type Hand = Readonly<{ kind: "Human", action: DropPiece } | { kind: "Ai" }>;
export type Status = "LastPlayerWin" | "Draw" | "Ongoing";
export type Data = Readonly<{ h: number, w: number, board: (undefined|PiecePutBy)[][], turn: number, status: Status }>;
export function start(h: number, w: number): Data;
export function legal_actions(data: Data): DropPiece[];
export function advance(data: Data, hand: Hand): Data;
export function debug(data: Data): string;

"#;
