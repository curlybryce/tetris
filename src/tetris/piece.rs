use rand::Rng;
use std::ops::{AddAssign, Sub, Add};
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
impl Sub for Pos {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            0: self.0 - other.0,
            1: self.1 - other.1,
        }
    }
}

const DOWN: Pos = Pos(1,0);
const LEFT: Pos = Pos(0,-1);
const RIGHT: Pos = Pos(0,1);

pub enum Rotate {
    Left,
    Right
}

#[derive(PartialEq)]
pub enum Dir {
    Down,
    Left,
    Right,
}
impl Dir {
    fn get(&self) -> Pos {
        match self {
            Dir::Down => DOWN,
            Dir::Left => LEFT,
            Dir::Right => RIGHT,
        }
    }
}

const NORMALL: [[i8; 4]; 4] = [
    [0,0,0,0],
    [1,0,0,0],
    [1,0,0,0],
    [1,1,0,0],
];
const REVERSEL: [[i8; 4]; 4] = [
    [0,0,0,0],
    [0,0,0,1],
    [0,0,0,1],
    [0,0,1,1],
];
const CUBE: [[i8; 4]; 4] = [
    [0,0,0,0],
    [0,1,1,0],
    [0,1,1,0],
    [0,0,0,0],
];
const TEE: [[i8; 4]; 4] = [
    [0,0,0,0],
    [0,0,0,0],
    [0,1,0,0],
    [1,1,1,0],
];
const DIAG: [[i8; 4]; 4] = [
    [0,0,0,0],
    [0,0,0,0],
    [1,1,0,0],
    [0,1,1,0],
];
const REVERSEDIAG: [[i8; 4]; 4] = [
    [0,0,0,0],
    [0,0,0,0],
    [0,0,1,1],
    [0,1,1,0],
];
const STRAIGHT: [[i8; 4]; 4] = [
    [1,0,0,0],
    [1,0,0,0],
    [1,0,0,0],
    [1,0,0,0],
];


pub enum Pieces {
    NormalL,
    ReverseL,
    Cube,
    Tee,
    Diag,
    ReverseDiag,
    Straight,
}
impl Pieces {
    pub fn random() -> Pieces {
        use Pieces::*;
        let mut rand = rand::thread_rng();
        match rand.gen_range(0..7) {
            0 => NormalL,
            1 => ReverseL,
            2 => Cube,
            3 => Tee,
            4 => Diag,
            5 => ReverseDiag,
            6 => Straight,
            _ => NormalL,
        }
    }

    fn get(&self) -> [[i8; 4]; 4] {
        use Pieces::*;
        match &self {
            NormalL => NORMALL,
            ReverseL => REVERSEL,
            Cube => CUBE,
            Tee => TEE,
            Diag => DIAG,
            ReverseDiag => REVERSEDIAG,
            Straight => STRAIGHT,
        }
    }

    fn get_origin(&self) -> Pos {
        use Pieces::*;
        match &self {
            NormalL => Pos(3,0),
            ReverseL => Pos(3,3),
            Cube => Pos(1,1),
            Tee => Pos(3,1),
            Diag => Pos(2,1),
            ReverseDiag => Pos(2,2),
            Straight => Pos(3,0),
        }
    }
}

pub struct Piece {
    area: [[i8; 4]; 4], // A static 3x3 area
    position: Pos,
    origin: Pos,
    alive: bool,
}
impl Piece {
    pub fn new(p: Pieces, pos: Pos) -> Piece {
        let origin = p.get_origin();
        Piece{
            area: p.get(),
            position: pos + origin,
            origin: origin,
            alive: true,
        }
    }
    
    // Return a random piece
    // out of the enum Pieces
    pub fn random(pos: Pos) -> Piece {
        let piece = Pieces::random();
        let origin = piece.get_origin();
        dbg!(pos - origin);
        Piece{
            area: piece.get(),
            position: pos - origin,
            origin: origin,
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

            // Detect blocks and kill only if Dir is down
            if grid.get_grid_pos(new_pos) == 1 && *dir == Dir::Down {
                self.kill();
                return Err(())
            } else if grid.get_grid_pos(new_pos) == 1 {
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

    pub fn get_area(&self) -> [[i8; 4]; 4] {
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
        let mut p = Piece::new(Pieces::Cube, Pos(0,0));
        let d = Dir::Down;
        p.apply_dir(&d);

        if p.get_pos() != Pos(2,1) {
            panic!("{:?} did not move down properly", p.get_pos())
        }
    }

    #[test]
    fn hit_test() {
        let mut p = Piece::new(Pieces::Cube, Pos(0,0));
        let d = Dir::Down;

        let mut tetris = Tetris::new();

        if p.is_alive() != false {
            panic!("Piece did not die, is_alive == {}", p.is_alive())
        }

    }
    #[test]
    fn hit_nothing_test() {
        let mut p = Piece::new(Pieces::Cube, Pos(0,0));
        let d = Dir::Down;

        let tetris = Tetris::new();

        p.r#move(d, &tetris);

        if p.is_alive() == false {
            panic!("Piece died, is_alive == {}", p.is_alive())
        }
    }
}