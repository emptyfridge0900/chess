pub mod error;
pub mod piece;
pub mod player;
pub mod board;
pub mod manager;

use std::{cell::RefCell, ops::{Add, Shl}};
use piece::Piece;


#[derive(Clone,Copy,Debug,PartialEq, Eq)]
pub enum Color {
    White,
    Black
}
impl Color{
    pub fn opposite(self)->Color{
        if self==Color::White{
            Color::Black
        } else{
            Color::White
        }
    }
    pub fn toggle(&mut self){
        if *self==Color::White{
            *self=Color::Black;
        } else{
            *self=Color::White;
        }
    }
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

#[derive(Clone,Copy,Debug,PartialEq, Eq,Hash)]
pub struct Point{
    pub file:char,
    pub rank:u32
}

impl Point{
    pub fn new(file:char,rank:u32)->Point{
        Point{ file,rank }
    }
    pub fn to_string(&self)->String{
        format!("{}{}",self.file,self.rank)
    }
    pub fn top(&self,color:Color)->Option<Point>{
        let rank = if color==Color::White{
            self.rank+1
        }else{
            self.rank-1
        };

        if rank>=1 && rank<=8{
            Some(Point{file:self.file,rank})
        } else{
            None
        }
    }
    pub fn top_left(&self,color:Color)->Option<Point>{
        let rank = if color==Color::White{
            self.rank+1
        }else{
            self.rank-1
        };
        let file = if color == Color::White{
            (self.file as u8 -1) as char
        }else{
            (self.file as u8 +1) as char
        };

        if file>='a' && file<='h' && rank>=1 && rank<=8{
            Some(Point::new(file, rank))
        }else{
            None
        }

    }
    pub fn top_right(&self,color:Color)->Option<Point>{
        let rank = if color==Color::White{
            self.rank+1
        }else{
            self.rank-1
        };
        let file = if color == Color::White{
            (self.file as u8 +1) as char
        }else{
            (self.file as u8 -1) as char
        };
        if file>='a' && file<='h' && rank>=1 && rank<=8{
            Some(Point::new(file, rank))
        }else{
            None
        }

    }
    pub fn right(&self,color:Color)->Option<Point>{
        let file = if color == Color::White{
            (self.file as u8 +1) as char
        }else{
            (self.file as u8 -1) as char
        };

        if file>='a' && file<='h'{
            Some(Point{file,rank:self.rank})
        }else{
            None
        }
    }
    pub fn left(&self,color:Color)->Option<Point>{
        let file = if color == Color::White{
            (self.file as u8 -1) as char
        }else{
            (self.file as u8 +1) as char
        };

        if file>='a' && file<='h'{
            Some(Point{file,rank:self.rank})
        }else{
            None
        }
    }
    pub fn bottom(&self,color:Color)->Option<Point>{
        let rank = if color==Color::White{
            self.rank-1
        }else{
            self.rank+1
        };
        
        if rank>=1 && rank<=8{
            Some(Point{file:self.file,rank})
        } else{
            None
        }
    }
    pub fn bottom_left(&self,color:Color)->Option<Point>{
        let rank = if color==Color::White{
            self.rank-1
        }else{
            self.rank+1
        };
        let file = if color == Color::White{
            (self.file as u8 -1) as char
        }else{
            (self.file as u8 +1) as char
        };
        if file>='a' && file<='h' && rank>=1 && rank<=8{
            Some(Point::new(file, rank))
        }else{
            None
        }

    }
    pub fn bottom_right(&self,color:Color)->Option<Point>{
        let rank = if color==Color::White{
            self.rank-1
        }else{
            self.rank+1
        };
        let file = if color == Color::White{
            (self.file as u8 +1) as char
        }else{
            (self.file as u8 -1) as char
        };
        if file>='a' && file<='h' && rank>=1 && rank<=8{
            Some(Point::new(file, rank))
        }else{
            None
        }

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
    pub fn moves(&self)->Vec<Point>{
        self.piece.borrow().as_ref().unwrap().moves()
    }
    pub fn particular_moves(&self)->Vec<Point>{
        self.piece.borrow().as_ref().unwrap().particular_moves()
    }
    pub fn is_some(&self)->bool{
        self.piece.borrow().as_ref().is_some()
    }
    pub fn is_none(&self)->bool{
        self.piece.borrow().as_ref().is_none()
    }
    pub fn props(&self)->Props{
        self.piece.borrow().as_ref().unwrap().get_props()
    }
}


pub enum MoveType{
    Nomal=0,
    Promotion=1<<14,
    EnPassant=2<<14,
    Castling=3<<14
}

pub struct Move{
    pub data:u16

}
impl Move{
    pub fn new(){

    }
    pub fn new1(&self,from:Squre,to:Squre){
        let x = (from << 6) + to;
    }
}

pub enum Squre{
    SqA1, SqB1, SqC1, SqD1, SqE1, SqF1, SqG1, SqH1,
    SqA2, SqB2, SqC2, SqD2, SqE2, SqF2, SqG2, SqH2,
    SqA3, SqB3, SqC3, SqD3, SqE3, SqF3, SqG3, SqH3,
    SqA4, SqB4, SqC4, SqD4, SqE4, SqF4, SqG4, SqH4,
    SqA5, SqB5, SqC5, SqD5, SqE5, SqF5, SqG5, SqH5,
    SqA6, SqB6, SqC6, SqD6, SqE6, SqF6, SqG6, SqH6,
    SqA7, SqB7, SqC7, SqD7, SqE7, SqF7, SqG7, SqH7,
    SqA8, SqB8, SqC8, SqD8, SqE8, SqF8, SqG8, SqH8,
    SqNone,

    //SQUARE_ZERO = 0,
    //SQUARE_NB   = 64
}

impl Shl<i32> for Squre{
    type Output=Self;
    fn shl(self, rhs:i32) -> Self::Output{
        let lhs=self;
        let i =lhs as i32;
        let x =i<<rhs;
        let y:Squre=x;
    }
}
impl Add<Squre> for Squre{
    type Output=Self;

    fn add(self, rhs: Squre) -> Self::Output {
        self+rhs
    }
    
}