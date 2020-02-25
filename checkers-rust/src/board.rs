// TODO:
// * Parameterize board dimensions

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum PieceColor {
    White,
    Black,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GamePiece {
    pub color: PieceColor,
    pub crowned: bool,
}

impl GamePiece {
    pub fn new(color: PieceColor) -> Self {
        Self {
            color,
            crowned: false,
        }
    }

    pub fn crowned(p: Self) -> Self {
        Self {
            color: p.color,
            crowned: true,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Coordinate(pub usize, pub usize);

impl Coordinate {
    // Determine if the position on the board
    pub fn on_board(self) -> bool {
        let Coordinate(x, y) = self;
        x <= 7 && y <= 7
    }

    // Get a vector of all jumps from the current position (i.e. capture a piece by jumping it)
    pub fn jump_targets_from(&self) -> impl Iterator<Item = Coordinate> {
        let mut jumps = Vec::new();
        let Coordinate(x, y) = *self;

        // Jump southeast
        if y >= 2 {
            jumps.push(Coordinate(x + 2, y - 2));
        }
        // Jump southwest
        if x >= 2 && y >= 2 {
            jumps.push(Coordinate(x - 2, y - 2));
        }
        // Jump northwest
        if x >= 2 {
            jumps.push(Coordinate(x - 2, y + 2));
        }
        // Jump northeast
        jumps.push(Coordinate(x + 2, y + 2));

        jumps.into_iter()
    }

    pub fn move_targets_from(&self) -> impl Iterator<Item = Coordinate> {
        let mut moves = Vec::new();
        let Coordinate(x, y) = *self;

        // Moves northwest
        if x >= 1 {
            moves.push(Coordinate(x - 1, y + 1));
        }
        // Moves southeast
        if y >= 1 {
            moves.push(Coordinate(x + 1, y - 1));
        }
        // Moves southwest
        if x >= 1 && y >= 1 {
            moves.push(Coordinate(x - 1, y - 1));
        }
        // Moves northeast
        moves.push(Coordinate(x + 1, y + 1));

        moves.into_iter()
    }
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Move {
    pub from: Coordinate,
    pub to: Coordinate,
}

impl Move {
    pub fn new(from: (usize, usize), to: (usize, usize)) -> Self {
        Self {
            from: Coordinate(from.0, from.1),
            to: Coordinate(to.0, to.1),
        }
    }
}
