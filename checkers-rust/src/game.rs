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

// TODO: Evaulaute public setting of all methods. I've been setting them all to
// pub while prototyping

// NOTE: Board size is 8x8. This should probably be parameterized to a property,
// even if checkers has a standard board size. It makes the code more readable.
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

    // Change turn owner to the other color
    pub fn advance_turn(&mut self) {
        match (self.current_turn) {
            PieceColor::White => { self.current_turn = PieceColor::Black },
            PieceColor::Black => { self.current_turn = PieceColor::White },
        }
    }

    pub fn init_pieces(&mut self) {
        // Set the initial position of all the white pieces
        [1, 3, 5, 7, 0, 2, 4, 6, 1, 3, 5, 7] // x coordinates
            .iter()
            .zip([0, 0, 0, 0, 1, 1, 1, 1, 2, 2, 2, 2].iter()) // y coordinates
            .map(|(a, b)|  (*a as usize, *b as usize))
            .for_each(|(x, y)| {
                self.board[x][x] = Some(GamePiece::new(PieceColor::White));
            });

        // Set the initial position of all the black pieces
        [0, 2, 4, 6, 1, 3, 5, 7, 0, 2, 4, 6] // x coordinates
            .iter()
            .zip([5, 5, 5, 5, 6, 6, 6, 6, 7, 7, 7, 7].iter()) // y coordiantes
            .map(|(a, b)|  (*a as usize, *b as usize))
            .for_each(|(x, y)| {
                self.board[x][x] = Some(GamePiece::new(PieceColor::White));
            });

    }

    pub fn move_piece(&mut self, mv: &Move) -> Result<MoveResult, ()> {
        // Err(())
        let legal_moves = self.legal_moves();
        
        if !legal_moves.contains(mv) {
            return Err(());
        }

        // Get 'to' and 'from' coordinates
        let Coordinate(fx, fy) = mv.from;
        let Coordinate(tx, ty) = mv.to;
        // TODO: Remove use of unwrap with better validation
        let piece = self.board[fx][fy].unwrap();

        // Assess whether a piece is being jumped, and if so remove it from the board
        let midpiece_coordinate = self.midpiece_coordinate(fx, fy, tx, ty);
        if let Some(Coordinate(x, y)) = midpiece_coordinate {
            // Remove the piece which was jumped by setting space to None
            self.board[x][y] = None;
        }

        // Moves piece from soure to destination
        self.board[tx][ty] = Some(piece);
        self.board[fx][fy] = None;

        // Assess whether or not to crown piece
        let crowned = if self.should_crown(piece, mv.to) {
            self.crown_piece(mv.to);
            true
        } else {
            false
        };
        self.advance_turn();

        Ok(MoveResult {
            mv: mv.clone(),
            crowned,
        })
    }

    // Boolean indicating if piece should be crowned when moved to target location
    fn should_crown(&self, piece: GamePiece, loc: Coordinate) -> bool {
        let Coordinate(x, y) = loc;
        match piece.color {
            PieceColor::White => { y == 7 },
            PieceColor::Black => { y == 0 },
        }
    }

    // TOOD: Remove unwrap.
    // TODO: I'm following the function signature in the book, which uses a Coordiante
    // argument, but it seems easier to just pass the piece to crown, or make a method on GamePiece
    // Crown a piece at a given coordinate
    fn crown_piece(&self, loc: Coordinate) {
        let Coordinate(x, y) = loc;
        let mut piece = self.board[x][y].unwrap();
        piece.crowned = true;
    }

    fn legal_moves(&self) -> Vec<Move> {
        let mut moves: Vec<Move> = Vec::new();
        for col in 0..8 {
            for row in 0..8 {
                if let Some(piece) = self.board[col][row] {
                    if piece.color == self.current_turn {
                        let loc = Coordinate(col, row);
                        let mut valid_moves = self.valid_moves_from(loc);
                        moves.append(&mut valid_moves);
                    }
                }
            }
        }

        moves
    }

    fn valid_moves_from(&self, loc:Coordinate) -> Vec<Move> {
        let Coordinate(x, y) = loc;
        if let Some(p) = self.board[x][y] {
            let mut jumps = loc
                .jump_targets_from()
                .filter(|t| self.valid_jump(&p, &loc, &t))
                .map(|ref t| Move {
                    from: loc.clone(),
                    to: t.clone(),
                }).collect::<Vec<Move>>();
            let mut moves = loc
                .move_targets_from()
                .filter(|t| self.valid_move(&p, &loc, &t))
                .map(|ref t| Move {
                    from: loc.clone(),
                    to: t.clone(),
                }).collect::<Vec<Move>>();
            jumps.append(&mut moves);
            jumps
        } else {
            Vec::new()
        }
    }

    // TODO: Implement
    pub fn valid_move(&self, piece: &GamePiece, loc: &Coordinate, target: &Coordinate) -> bool {
        false
    }

    // TODO: Implement
    pub fn valid_jump(&self, piece: &GamePiece, loc: &Coordinate, target: &Coordinate) -> bool {
        false
    }

    pub fn midpiece_coordinate(&self, fx: usize, fy: usize, tx: usize, ty: usize) -> Option<Coordinate> {
        let mx = fx + tx / 2;
        let my = fy + ty / 2;
        match self.board[mx][my] {
            Some(_piece) => { Some(Coordinate(mx, my)) },
            None => { None },
        }
    }





}