mod board;
mod game;

#[macro_use]
extern crate lazy_static;

use board::{Coordinate, GamePiece, Move, PieceColor};
use game::GameEngine;
use mut_static::MutStatic;

pub const PIECEFLAG_BLACK: u8 = 1;
pub const PIECEFLAG_WHITE: u8 = 2;
pub const PIECEFLAG_CROWN: u8 = 4;

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

    // NOTE: There is a typo in the book here which lists current turn as a method
    GamePiece::new(engine.current_turn).into()
}

#[no_mangle]
pub extern "C" fn move_piece(fx: i32, fy: i32, tx: i32, ty: i32) -> i32 {
    unimplemented!();
}

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
}
