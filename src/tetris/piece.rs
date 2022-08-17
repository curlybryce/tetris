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

const NONE: Pos = Pos(0,0);
const DOWN: Pos = Pos(1,0);
const LEFT: Pos = Pos(0,-1);
const RIGHT: Pos = Pos(0,1);

#[derive(PartialEq)]
pub enum Rotate {
    Left,
    Right
}

#[derive(PartialEq)]
pub enum Dir {
    None,
    Down,
    Left,
    Right,
}
impl Dir {
    fn get(&self) -> Pos {
        match self {
            Dir::None => NONE,
            Dir::Down => DOWN,
            Dir::Left => LEFT,
            Dir::Right => RIGHT,
        }
    }
}

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

    fn get(&self) -> Vec<Vec<i8>> {
        let l: Vec<Vec<i8>> = vec![
            // Outer most ring: top, right, bottom, left
            vec![0,0,1,0,0, 0,0,0, 0,0,0,0,0, 0,0,0],
            // Inner rign: top, right, bottom, left
            vec![0,1,0, 0, 0,0,0, 1],
            // Center ring.  Is used as the center of rotation
            vec![1],
        ];
        let r_l: Vec<Vec<i8>> = vec![
            vec![0,0,1,0,0, 0,0,0, 0,0,0,0,0, 0,0,0],
            vec![0,1,0, 1, 0,0,0, 0],
            vec![1],
        ];
        let cube: Vec<Vec<i8>> = vec![
            vec![0,0,0,0,0, 0,0,0, 0,0,0,0,0, 0,0,0],
            vec![0,1,1, 1, 0,0,0, 0],
            vec![1],
        ];
        let tee: Vec<Vec<i8>> = vec![
            vec![0,0,0,0,0, 0,0,0, 0,0,0,0,0, 0,0,0],
            vec![0,1,0, 1, 0,0,0, 1],
            vec![1],
        ];
        let diag: Vec<Vec<i8>> = vec![
            vec![0,0,0,0,0, 0,0,0, 0,0,0,0,0, 0,0,0],
            vec![1,1,0, 1, 0,0,0, 0],
            vec![1],
        ];
        let r_diag: Vec<Vec<i8>> = vec![
            vec![0,0,0,0,0, 0,0,0, 0,0,0,0,0, 0,0,0],
            vec![0,1,1, 0, 0,0,0, 1],
            vec![1],
        ];
        let straight: Vec<Vec<i8>> = vec![
            vec![0,0,1,0,0, 0,0,0, 0,0,0,0,0, 0,0,0],
            vec![0,1,0, 0, 0,1,0, 0],
            vec![1],
        ];

        use Pieces::*;

        match &self {
            NormalL => l,
            ReverseL => r_l,
            Cube => cube,
            Tee => tee,
            Diag => diag,
            ReverseDiag => r_diag,
            Straight => straight,
        }
    }
}

pub struct Piece {
    area: Vec<Vec<i8>>, // A static 3x3 area
    position: Pos,
    alive: bool,
}
impl Piece {
    pub fn new(p: Pieces, pos: Pos) -> Piece {
        Piece{
            area: p.get(),
            position: pos,
            alive: true,
        }
    }
    
    // Return a random piece
    // out of the enum Pieces
    pub fn random(pos: Pos) -> Piece {
        let piece = Pieces::random();
        Piece{
            area: piece.get(),
            position: pos,
            alive: true,
        }
    }

    // Get area as a 2d array
    fn get_area(&self) -> [[i8; 5]; 5] {
        let mut array = [[0; 5]; 5];
        
        for sections in &self.area {
            if sections.len() == 16 {
                // Top
                array[0][0] = *sections.get(0).expect("Invalid");
                array[0][1] = *sections.get(1).expect("Invalid");
                array[0][2] = *sections.get(2).expect("Invalid");
                array[0][3] = *sections.get(3).expect("Invalid");
                array[0][4] = *sections.get(4).expect("Invalid");
                // Right
                array[1][4] = *sections.get(5).expect("Invalid");
                array[2][4] = *sections.get(6).expect("Invalid");
                array[3][4] = *sections.get(7).expect("Invalid");
                // Bottom
                array[4][4] = *sections.get(8).expect("Invalid");
                array[4][3] = *sections.get(9).expect("Invalid");
                array[4][2] = *sections.get(10).expect("Invalid");
                array[4][1] = *sections.get(11).expect("Invalid");
                array[4][0] = *sections.get(12).expect("Invalid");
                // Left
                array[3][0] = *sections.get(13).expect("Invalid");
                array[2][0] = *sections.get(14).expect("Invalid");
                array[1][0] = *sections.get(15).expect("Invalid");            }
            if sections.len() == 8 {
                // Top
                array[1][1] = *sections.get(0).expect("Invalid");
                array[1][2] = *sections.get(1).expect("Invalid");
                array[1][3] = *sections.get(2).expect("Invalid");
                // Right
                array[2][3] = *sections.get(3).expect("Invalid");
                // Bottom
                array[3][3] = *sections.get(4).expect("Invalid");
                array[3][2] = *sections.get(5).expect("Invalid");
                array[3][1] = *sections.get(6).expect("Invalid");
                // Left
                array[2][1] = *sections.get(7).expect("Invalid");
            }
            if sections.len() == 1 {
                array[2][2] = *sections.get(0).expect("Invalid");
            }
        }

        return array
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
                    piece_pos_vec.push(self.get_pos() + Pos(y, x))
                }
            }
            y += 1;
        }

        return piece_pos_vec
    }

    pub fn rotate(&mut self, r: Rotate, tetris: &Tetris) {
        let mut area = self.area.clone();

        for section in &self.area {
            let len = section.len();
            let mut range: Vec<usize> = (0..len).collect();
            if r == Rotate::Left {
                let mut new = vec![];
                for bit in range {
                    new.insert(0, bit);
                }
                range = new;
            }

            for c in range {
                let mut loop_num = 0;
                let mut loop_c = 4;

                // Set to the inner loop
                if len == 8 {
                    loop_num = 1;
                    loop_c = 2;
                } else if len == 1 {
                    continue
                }

                // Right
                if c < loop_c && r == Rotate::Right {
                    let x = area[loop_num].pop().expect("Out of Range");
                    area[loop_num].insert(0, x);

                // Left
                } else if c < loop_c && r == Rotate::Left {
                    let x = *area[loop_num].get(0).expect("Out of Range");
                    area[loop_num].remove(0);
                    area[loop_num].push(x);
                }
            }
        }

        let old_area = self.area.clone();
        self.area = area;

        match self.hit_detect(&Dir::None, tetris) {
            Ok(()) => (),
            Err(()) => self.area = old_area,
        }

    }

    pub fn get_pos(&self) -> Pos {
        self.position
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

        if p.get_pos() != Pos(1,0) {
            panic!("{:?} did not move down properly", p.get_pos())
        }
    }

    #[test]
    fn hit_test() {
        let mut p = Piece::new(Pieces::Cube, Pos(30,0));
        let d = Dir::Down;

        let tetris = Tetris::new();

        p.r#move(d, &tetris);

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