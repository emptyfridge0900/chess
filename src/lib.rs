pub mod error;
pub mod piece;

use std::cell::RefCell;
use piece::Piece;


#[derive(Clone,Copy,Debug,PartialEq, Eq)]
pub enum Color {
    White,
    Black
}
#[derive(Clone,Copy,Debug,PartialEq, Eq)]
pub enum Type{
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn
}
#[derive(Clone,Copy,Debug,PartialEq, Eq)]
pub struct Point{
    file:char,
    rank:u32
}

pub struct Props{
    pub color:Color,
    pub name:Type,
    pub point:Point
}

impl Point{
    pub fn new(file:char,rank:u32)->Point{
        Point{ file,rank }
    }
    pub fn notation(&self)->String{
        format!("{}{}",self.file,self.rank)
    }
}

pub struct Square{
    pub piece:RefCell<Option<Box<dyn Piece>>>,
    pub point:Point
}
impl Square{
    pub fn new(piece:RefCell<Option<Box<dyn Piece>>>,point:Point)->Square{
        Square { piece, point }
    }
}