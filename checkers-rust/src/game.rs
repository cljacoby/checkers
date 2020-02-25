use super::board::{Coordinate, GamePiece, Move, PieceColor};

pub struct GameEngine {
    board: [[Option<GamePiece>; 8]; 8],
    current_turn: PieceColor,
    move_count: u32,
}

pub struct MoveResult {
    pub mv: Move,
    pub crowned: bool,
}

impl GameEngine {
    pub fn new() -> Self {
        
        let mut engine = GameEngine {
            board: [[None; 8]; 8],
            current_turn: PieceColor::Black,
            move_count: 0,
        };

        engine.init_pieces();
        engine
    }

    pub fn init_pieces(&mut self) {
        // Set the initial position of all the white pieces
        [1, 3, 5, 7, 0, 2, 4, 6, 1, 3, 5, 7]
            .iter()
            .zip([0, 0, 0, 0, 1, 1, 1, 1, 2, 2, 2, 2].iter())
            .map(|(a, b)|  (*a as usize, *b as usize))
            .for_each(|(x, y)| {
                self.board[x][x] = Some(GamePiece::new(PieceColor::White));
            });

        // Set the initial position of all the black pieces
        [0, 2, 4, 6, 1, 3, 5, 7, 0, 2, 4, 6]
            .iter()
            .zip([5, 5, 5, 5, 6, 6, 6, 6, 7, 7, 7, 7].iter())
            .map(|(a, b)|  (*a as usize, *b as usize))
            .for_each(|(x, y)| {
                self.board[x][x] = Some(GamePiece::new(PieceColor::White));
            });

    }
}