pub mod error;
pub mod piece;
pub mod player;
pub mod board;

use std::cell::RefCell;
use piece::Piece;


#[derive(Clone,Copy,Debug,PartialEq, Eq)]
pub enum Color {
    White,
    Black
}
impl std::fmt::Display for Color{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}",self)
    }
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


pub struct Props{
    pub id:u128,
    pub color:Color,
    pub name:Type,
}
impl Props{
    pub fn get_name(&self)-> char{
        match (self.color,self.name){
            (Color::Black,Type::King)=>'♚',
            (Color::Black,Type::Queen)=>'♛',
            (Color::Black,Type::Rook)=>'♜',
            (Color::Black,Type::Bishop)=>'♝',
            (Color::Black,Type::Knight)=>'♞',
            (Color::Black,Type::Pawn)=>'♟',
            (Color::White,Type::King)=>'♔',
            (Color::White,Type::Queen)=>'♕',
            (Color::White,Type::Rook)=>'♖',
            (Color::White,Type::Bishop)=>'♗',
            (Color::White,Type::Knight)=>'♘',
            (Color::White,Type::Pawn)=>'♙',
            _=>unreachable!()
        }
    }
}

#[derive(Clone,Copy,Debug,PartialEq, Eq)]
pub struct Point{
    pub file:char,
    pub rank:u32
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