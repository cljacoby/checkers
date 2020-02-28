mod board;
mod game;

#[macro_use]
extern crate lazy_static;

use board::{Coordinate, GamePiece, Move, PieceColor};
use game::GameEngine;
use mut_static::MutStatic;

const PIECEFLAG_BLACK: u8 = 1;
const PIECEFLAG_WHITE: u8 = 2;
const PIECEFLAG_CROWN: u8 = 4;

lazy_static! {
    pub static ref GAME_ENGINE: MutStatic<GameEngine> = {
        MutStatic::from(GameEngine::new())
    };
}

// TODO: Remove use of unwrap, or at least justify use.
// Author actually recommends using unwrap, as failing to read global state should cause a trap
#[no_mangle]
pub extern "C" fn get_piece(x:i32, y:i32) -> i32 {
    let engine = GAME_ENGINE.read().unwrap();

    let piece = engine.get_piece(Coordinate(x as usize, y as usize));
    match piece {
        Ok(Some(p)) => { p.into() },
        Ok(None) => -1,
        Err(_) => -1,
    }
}

#[no_mangle]
pub extern "C" fn get_current_turn() -> i32 {
    let engine = GAME_ENGINE.read().unwrap();

    GamePiece::new(engine.current_turn()).into()
}


#[cfg(test)]
mod tests {
}
