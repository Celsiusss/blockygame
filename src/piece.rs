use rand::{Rng, prelude::Distribution, distributions};
use sdl2::pixels::Color;


use crate::{grid::Position, shapes::{ShapeRotations, Ls, Js, Ss, Zs, Is, Os, Ts}};

// pub struct Shape<'a> {
//     pub shape: [[Option<&'a Block>; 4]; 4]
// }

#[derive(Clone, Copy)]
pub enum PieceType {
    L, J, S, Z, I, O, T
}

impl Distribution<PieceType> for distributions::Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> PieceType {
        match rng.gen_range(0..7) {
            0 => PieceType::L,
            1 => PieceType::J,
            2 => PieceType::S,
            3 => PieceType::Z,
            4 => PieceType::I,
            5 => PieceType::O,
            6 => PieceType::T,
            _ => panic!("this should never happen")
        }
    }
}

#[derive(Clone, Copy)]
pub struct ActivePiece {
    pub shape: Shape,
    pub position: Position,
    pub piece_type: PieceType,
    pub rotation: u8
}

impl ActivePiece {
    pub fn new(piece_type: PieceType) -> Self {
        ActivePiece {
            shape: *get_default_piece(piece_type),
            position: Position(0, 0),
            piece_type,
            rotation: 0
        }
    }
}


#[derive(Clone, Copy)]
pub struct Shape {
    pub shape: [[Option<Block>; 4]; 4],
    pub bounds: (usize, usize),
    pub pos: Position
}

#[derive(Clone, Copy, Debug)]
pub struct Block {
    pub color: crate::shapes::Color,
}

#[derive(Clone, Copy, Debug)]
pub struct BlockData {
    pub block: Block,
    pub placed: bool,
    pub ghost: bool
}


pub static PIECES: [&ShapeRotations; 7] = [&Ls, &Js, &Ss, &Zs, &Is, &Os, &Ts];

pub fn get_random_piece() -> (&'static Shape, PieceType) {
    let piece_type: PieceType = rand::random();
    return (get_default_piece(piece_type), piece_type);
}

pub fn get_default_piece(piece_type: PieceType) -> &'static Shape {
    get_piece_roation(piece_type, 0)
}

pub fn get_piece_roation(piece_type: PieceType, rotation: usize) -> &'static Shape {
    match piece_type {
        PieceType::L => &Ls[rotation],
        PieceType::J => &Js[rotation],
        PieceType::S => &Ss[rotation],
        PieceType::Z => &Zs[rotation],
        PieceType::I => &Is[rotation],
        PieceType::O => &Os[rotation],
        PieceType::T => &Ts[rotation],
    }
}
