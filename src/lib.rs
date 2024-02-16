pub mod error;
pub mod piece;
pub mod player;
pub mod board;
pub mod manager;

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

#[derive(Clone,Copy,Debug,PartialEq, Eq,Hash)]
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