pub struct Pos(i8, i8);

pub const UP: Pos = Pos(1,0);
pub const DOWN: Pos = Pos(-1,0);
pub const LEFT: Pos = Pos(0,-1);
pub const RIGHT: Pos = Pos(0,1);

pub enum Dir {
    Up,
    Down,
    Left,
    Right,
}

const NORMALL: [[i8; 3]; 3] = [[0,1,0],[0,1,0],[0,1,1]];
const REVERSEL: [[i8; 3]; 3] = [[0,1,0],[0,1,0],[1,1,0]];
const CUBE: [[i8; 3]; 3] = [[0,1,1],[0,1,1],[0,0,0]];
const TEE: [[i8; 3]; 3] = [[0,1,0],[1,1,1],[0,0,0]];
const DIAG: [[i8; 3]; 3] = [[0,1,1],[1,1,0],[0,0,0]];
const REVERSEDIAG: [[i8; 3]; 3] = [[1,1,0],[0,1,1],[0,0,0]];

enum Pieces {
    NormalL,
    ReverseL,
    Cube,
    Tee,
    Diag,
    ReverseDiag,
}

pub struct Piece {
    area: [[i8; 3]; 3], // A static 3x3 area
    position: Pos,
    origin: Pos,
    alive: bool,
}
impl Piece {
    pub fn new() -> Piece {
        Piece{
            area: NORMALL,
            position: Pos(0,5),
            origin: Pos(1,1),
            alive: true,
        }
    }

    // Like new(), but return a random piece
    // out of the enum Pieces
    pub fn random() -> Piece {
        Piece{
            area: NORMALL,
            position: Pos(0,5),
            origin: Pos(1,1),
            alive: true,
        }
    }

    // Using a grid and a direction;
    // See if the new position would hit
    // something in the grid
    // If so, return err
    // otherwise return Some(Pos)
    fn hit_detect(&self, grid: &Vec<Vec<i8>>) -> Option<Pos> {
        None
    }

    // Return false if the piece cannot move
    pub fn r#move(&self, dir: Dir) -> bool {
        false
    }
}