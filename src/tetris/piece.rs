use rand::Rng;
use std::ops::{AddAssign, Add};
use crate::tetris::Tetris;

#[derive(Copy)]
#[derive(Clone)]
#[derive(PartialEq)]
#[derive(Debug)]
pub struct Pos(pub i8, pub i8);
impl AddAssign for Pos {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            0: self.0 + other.0,
            1: self.1 + other.1,
        };
    }
}
impl Add for Pos {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            0: self.0 + other.0,
            1: self.1 + other.1,
        }
    }
}

const UP: Pos = Pos(-1,0);
const DOWN: Pos = Pos(1,0);
const LEFT: Pos = Pos(0,-1);
const RIGHT: Pos = Pos(0,1);

pub enum Rotate {
    Left,
    Right
}

pub enum Dir {
    Up,
    Down,
    Left,
    Right,
}
impl Dir {
    fn get(&self) -> Pos {
        match self {
            Dir::Up => UP,
            Dir::Down => DOWN,
            Dir::Left => LEFT,
            Dir::Right => RIGHT,
        }
    }
}

const NORMALL: [[i8; 3]; 3] = [[0,1,0],[0,1,0],[0,1,1]];
const REVERSEL: [[i8; 3]; 3] = [[0,1,0],[0,1,0],[1,1,0]];
const CUBE: [[i8; 3]; 3] = [[0,1,1],[0,1,1],[0,0,0]];
const TEE: [[i8; 3]; 3] = [[0,1,0],[1,1,1],[0,0,0]];
const DIAG: [[i8; 3]; 3] = [[0,1,1],[1,1,0],[0,0,0]];
const REVERSEDIAG: [[i8; 3]; 3] = [[1,1,0],[0,1,1],[0,0,0]];

pub enum Pieces {
    NormalL,
    ReverseL,
    Cube,
    Tee,
    Diag,
    ReverseDiag,
}
impl Pieces {
    pub fn random() -> Pieces {
        use Pieces::*;
        let mut rand = rand::thread_rng();
        match rand.gen_range(0..6) {
            0 => NormalL,
            1 => ReverseL,
            2 => Cube,
            3 => Tee,
            4 => Diag,
            5 => ReverseDiag,
            _ => NormalL,
        }
    }

    fn get(&self) -> [[i8; 3]; 3] {
        use Pieces::*;
        match &self {
            NormalL => NORMALL,
            ReverseL => REVERSEL,
            Cube => CUBE,
            Tee => TEE,
            Diag => DIAG,
            ReverseDiag => REVERSEDIAG,
        }
    }

    fn get_origin(&self) -> Pos {
        use Pieces::*;
        match &self {
            NormalL => Pos(0,0),
            ReverseL => Pos(0,0),
            Cube => Pos(0,0),
            Tee => Pos(0,0),
            Diag => Pos(0,0),
            ReverseDiag => Pos(0,0),
        }
    }
}

pub struct Piece {
    area: [[i8; 3]; 3], // A static 3x3 area
    position: Pos,
    origin: Pos,
    alive: bool,
}
impl Piece {
    pub fn new(p: Pieces) -> Piece {
        Piece{
            area: p.get(),
            position: Pos(0,5),
            origin: p.get_origin(),
            alive: true,
        }
    }
    
    // Return a random piece
    // out of the enum Pieces
    pub fn random() -> Piece {
        let piece = Pieces::random();
        Piece{
            area: piece.get(),
            position: Pos(0,4),
            origin: piece.get_origin(),
            alive: true,
        }
    }

    // Using a grid and a direction;
    // See if the new position would hit
    // something in the grid
    // If so, return Err
    // otherwise return Ok
    fn hit_detect(&mut self, dir: &Dir, grid: &Tetris) -> Result<(), ()> {        
        for pos in self.get_bits_pos() {
            let new_pos = pos + dir.get();

            // Detect blocks and kill
            if grid.get_grid_pos(new_pos) == 1 {
                self.kill();
                return Err(())
            } else if new_pos.0 > 19 { // Detect bottom and kill
                self.kill();
                return Err(())
            } else if new_pos.1 > 9 || new_pos.1 < 0 { // Detect sides
                return Err(())
            }
        }
        Ok(())
    }

    pub fn apply_to_grid(&self, grid: &mut Tetris) {
        for pos in self.get_bits_pos() {
            grid.set_grid(pos, 1)
        }
    }

    fn apply_dir(&mut self, dir: &Dir) {
        self.position += dir.get();
    }

    pub fn get_bits_pos(&self) -> Vec<Pos> {
        let mut piece_pos_vec = vec![];

        let mut y = -1;
        for posy in self.get_area() {
            let mut x = -1;
            for posx in posy {
                x += 1;
                if posx == 1 {
                    piece_pos_vec.push(self.get_pos() + Pos(y, x) + self.origin)
                }
            }
            y += 1;
        }

        return piece_pos_vec
    }

    pub fn rotate(&mut self, r: Rotate) {
        // Must call hit detect before applying
    }

    pub fn get_pos(&self) -> Pos {
        self.position
    }

    pub fn get_area(&self) -> [[i8; 3]; 3] {
        self.area
    }

    pub fn is_alive(&self) -> bool {
        self.alive
    }

    fn kill(&mut self) {
        self.alive = false
    }

    // Return false if the piece cannot move
    pub fn r#move(&mut self, dir: Dir, grid: &Tetris) {
        // If a hit is detected, don't move
        // Otherwise move
        match self.hit_detect(&dir, &grid) {
            Ok(_) => self.apply_dir(&dir),
            Err(_) => (),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn apply_down() {
        let mut p = Piece::new(Pieces::Cube);
        let d = Dir::Down;
        p.apply_dir(&d);

        if p.get_pos() != Pos(1,5) {
            panic!("{:?} did not move down properly", p.get_pos())
        }
    }

    #[test]
    fn hit_test() {
        let mut p = Piece::new(Pieces::Cube);
        let d = Dir::Down;

        let tetris = Tetris::new();

        p.r#move(d, &tetris);

        if p.is_alive() != false {
            panic!("Piece did not die, is_alive == {}", p.is_alive())
        }

    }
    #[test]
    fn hit_nothing_test() {
        let mut p = Piece::new(Pieces::Cube);
        let d = Dir::Down;

        let tetris = Tetris::new();

        p.r#move(d, &tetris);

        if p.is_alive() == false {
            panic!("Piece died, is_alive == {}", p.is_alive())
        }
    }
}