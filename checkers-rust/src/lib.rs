mod board;
mod game;

// #[macro_use]
// extern crate lazy_static;
extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

use board::{Coordinate, GamePiece, Move, PieceColor};
use game::GameEngine;
// use mut_static::MutStatic;

pub const PIECEFLAG_BLACK: u8 = 1;
pub const PIECEFLAG_WHITE: u8 = 2;
pub const PIECEFLAG_CROWN: u8 = 4;

// lazy_static! {
//     pub static ref GAME_ENGINE: MutStatic<GameEngine> = {
//         MutStatic::from(GameEngine::new())
//     };
// }

extern "C" {
    fn host_piecemoved(fromX: i32, fromY: i32, toX: i32, toY: i32);
    fn host_piececrowned(x: i32, t: i32);
}

// Import 'window.alert'
#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

// Export a 'hello' function
#[wasm_bindgen]
pub fn hello(name: &str) {
    alert(&format!("Hello, {}!", name));
}

#[wasm_bindgen]
pub fn console_log(s: &str) {
    log(s);
}


// // TODO: Remove use of unwrap, or at least justify use.
// // Author actually recommends using unwrap, as failing to read global state should cause a trap
// #[no_mangle]
// pub extern "C" fn get_piece(x:i32, y:i32) -> i32 {
//     let engine = GAME_ENGINE.read().unwrap();

//     let piece = engine.get_piece(Coordinate(x as usize, y as usize));
//     match piece {
//         Ok(Some(p)) => { p.into() },
//         Ok(None) => -1,
//         Err(_) => -1,
//     }
// }

// #[no_mangle]
// pub extern "C" fn get_current_turn() -> i32 {
//     let engine = GAME_ENGINE.read().unwrap();

//     // NOTE: There is a typo in the book here which lists current turn as a method
//     GamePiece::new(engine.current_turn).into()
// }

// #[no_mangle]
// pub extern "C" fn move_piece(fx: i32, fy: i32, tx: i32, ty: i32) {
// // pub extern "C" fn move_piece(fx: i32, fy: i32, tx: i32, ty: i32) -> i32 {
//     let mut engine = GAME_ENGINE.read().unwrap();
//     let mv =  Move::new(
//         (fx as usize, fy as usize),
//         (tx as usize, ty as usize)
//     );
//     // let result = engine.move_piece(&mv);
//     // match result {
//     //     Err(_) => 0,
//     //     Ok(mr) => {
//     //         unsafe { host_piecemoved(fx, fy, tx, ty); }
//     //         if mr.crowned {
//     //             unsafe { host_piececrowned(tx, ty);  }
//     //         }
//     //         1
//     //     },
//     // }
//     // 1

// }

impl Into<i32> for GamePiece {
    fn into(self) -> i32 {
        let mut val: u8 = 0;
        match self.color {
            PieceColor::White => { val += PIECEFLAG_WHITE; },
            PieceColor::Black => { val += PIECEFLAG_BLACK; },
        }
        match self.crowned {
            true => { val += PIECEFLAG_CROWN; }
            false => {},
        }

        val as i32
    }
}


#[cfg(test)]
mod tests {

    use super::*;
    
    fn test_move_piece() {
        let mut engine = GameEngine::new();
        let mv =  Move::new(
            (0 as usize, 1 as usize),
            (0 as usize, 1 as usize)
        );
        println!("{:?}", engine);
        engine.move_piece(&mv);
    }
}


