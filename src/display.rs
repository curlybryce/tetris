use crate::tetris::piece::{Piece, Pos};

pub fn display(grid: &Vec<Vec<i8>>, piece: &Piece) {
    let piece_pos_vec = piece.get_bits_pos();

    let mut yc = 0;
    for y in grid {
        let mut xc = 0;
        for mut x in y {
            for bit in &piece_pos_vec {
                if bit == &Pos(yc, xc) {
                    x = &1;
                }
            }
            xc += 1;
            print!("{} ", x);
        }
        print!("\n");
        yc += 1;
    }
}