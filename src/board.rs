use std::cell::RefCell;

use crate::{Square, piece::{Piece, Rook, Knight, Bishop, King, Queen, Pawn}, Color, Point, Type, Props};


pub struct Board{
    squares:[[Square; 8]; 8]
}
impl Board{
    pub fn new()->Board{
       Board{ 
        squares : [
        [
            Square::new(RefCell::new(Some(Box::new(Rook{color:Color::Black,name:Type::Rook,point:Point::new('a',8)}))),Point::new('a',8)),
            Square::new(RefCell::new(Some(Box::new(Knight{color:Color::Black,name:Type::Knight,point:Point::new('b',8)}))),Point::new('b',8)),
            Square::new(RefCell::new(Some(Box::new(Bishop{color:Color::Black,name:Type::Bishop,point:Point::new('c',8)}))),Point::new('c',8)),
            Square::new(RefCell::new(Some(Box::new(King{color:Color::Black,name:Type::King,point:Point::new('d',8)}))),Point::new('d',8)),
            Square::new(RefCell::new(Some(Box::new(Queen{color:Color::Black,name:Type::Queen,point:Point::new('e',8)}))),Point::new('e',8)),
            Square::new(RefCell::new(Some(Box::new(Bishop{color:Color::Black,name:Type::Bishop,point:Point::new('f',8)}))),Point::new('f',8)),
            Square::new(RefCell::new(Some(Box::new(Knight{color:Color::Black,name:Type::Knight,point:Point::new('g',8)}))),Point::new('g',8)),
            Square::new(RefCell::new(Some(Box::new(Rook{color:Color::Black,name:Type::Rook,point:Point::new('h',8)}))),Point::new('h',8)),
        ],
        [
            Square::new(RefCell::new(Some(Box::new(Pawn{color:Color::Black,name:Type::Pawn,point:Point::new('a',7)}))),Point::new('a',7)),
            Square::new(RefCell::new(Some(Box::new(Pawn{color:Color::Black,name:Type::Pawn,point:Point::new('b',7)}))),Point::new('b',7)),
            Square::new(RefCell::new(Some(Box::new(Pawn{color:Color::Black,name:Type::Pawn,point:Point::new('c',7)}))),Point::new('c',7)),
            Square::new(RefCell::new(Some(Box::new(Pawn{color:Color::Black,name:Type::Pawn,point:Point::new('d',7)}))),Point::new('d',7)),
            Square::new(RefCell::new(Some(Box::new(Pawn{color:Color::Black,name:Type::Pawn,point:Point::new('e',7)}))),Point::new('e',7)),
            Square::new(RefCell::new(Some(Box::new(Pawn{color:Color::Black,name:Type::Pawn,point:Point::new('f',7)}))),Point::new('f',7)),
            Square::new(RefCell::new(Some(Box::new(Pawn{color:Color::Black,name:Type::Pawn,point:Point::new('g',7)}))),Point::new('g',7)),
            Square::new(RefCell::new(Some(Box::new(Pawn{color:Color::Black,name:Type::Pawn,point:Point::new('h',7)}))),Point::new('h',7)),
        ],
        [
            Square::new(RefCell::new(None),Point::new('a',6)),
            Square::new(RefCell::new(None),Point::new('b',6)),
            Square::new(RefCell::new(None),Point::new('c',6)),
            Square::new(RefCell::new(None),Point::new('d',6)),
            Square::new(RefCell::new(None),Point::new('e',6)),
            Square::new(RefCell::new(None),Point::new('f',6)),
            Square::new(RefCell::new(None),Point::new('g',6)),
            Square::new(RefCell::new(None),Point::new('h',6)),
        ],
        [
            Square::new(RefCell::new(None),Point::new('a',5)),
            Square::new(RefCell::new(None),Point::new('b',5)),
            Square::new(RefCell::new(None),Point::new('c',5)),
            Square::new(RefCell::new(None),Point::new('d',5)),
            Square::new(RefCell::new(None),Point::new('e',5)),
            Square::new(RefCell::new(None),Point::new('f',5)),
            Square::new(RefCell::new(None),Point::new('g',5)),
            Square::new(RefCell::new(None),Point::new('h',5)),
        ],
        [
            Square::new(RefCell::new(None),Point::new('a',4)),
            Square::new(RefCell::new(None),Point::new('b',4)),
            Square::new(RefCell::new(None),Point::new('c',4)),
            Square::new(RefCell::new(None),Point::new('d',4)),
            Square::new(RefCell::new(None),Point::new('e',4)),
            Square::new(RefCell::new(None),Point::new('f',4)),
            Square::new(RefCell::new(None),Point::new('g',4)),
            Square::new(RefCell::new(None),Point::new('h',4)),
        ],
        [
            Square::new(RefCell::new(None),Point::new('a',3)),
            Square::new(RefCell::new(None),Point::new('b',3)),
            Square::new(RefCell::new(None),Point::new('c',3)),
            Square::new(RefCell::new(None),Point::new('d',3)),
            Square::new(RefCell::new(None),Point::new('e',3)),
            Square::new(RefCell::new(None),Point::new('f',3)),
            Square::new(RefCell::new(None),Point::new('g',3)),
            Square::new(RefCell::new(None),Point::new('h',3)),
        ],
        [
            Square::new(RefCell::new(Some(Box::new(Pawn{color:Color::White,name:Type::Pawn,point:Point::new('a',2)}))),Point::new('a',2)),
            Square::new(RefCell::new(Some(Box::new(Pawn{color:Color::White,name:Type::Pawn,point:Point::new('b',2)}))),Point::new('b',2)),
            Square::new(RefCell::new(Some(Box::new(Pawn{color:Color::White,name:Type::Pawn,point:Point::new('c',2)}))),Point::new('c',2)),
            Square::new(RefCell::new(Some(Box::new(Pawn{color:Color::White,name:Type::Pawn,point:Point::new('d',2)}))),Point::new('d',2)),
            Square::new(RefCell::new(Some(Box::new(Pawn{color:Color::White,name:Type::Pawn,point:Point::new('e',2)}))),Point::new('e',2)),
            Square::new(RefCell::new(Some(Box::new(Pawn{color:Color::White,name:Type::Pawn,point:Point::new('f',2)}))),Point::new('f',2)),
            Square::new(RefCell::new(Some(Box::new(Pawn{color:Color::White,name:Type::Pawn,point:Point::new('g',2)}))),Point::new('g',2)),
            Square::new(RefCell::new(Some(Box::new(Pawn{color:Color::White,name:Type::Pawn,point:Point::new('h',2)}))),Point::new('h',2)),
        ],
        [
            Square::new(RefCell::new(Some(Box::new(Rook{color:Color::White,name:Type::Rook,point:Point::new('a',1)}))),Point::new('a',1)),
            Square::new(RefCell::new(Some(Box::new(Knight{color:Color::White,name:Type::Knight,point:Point::new('b',1)}))),Point::new('b',1)),
            Square::new(RefCell::new(Some(Box::new(Bishop{color:Color::White,name:Type::Bishop,point:Point::new('c',1)}))),Point::new('c',1)),
            Square::new(RefCell::new(Some(Box::new(King{color:Color::White,name:Type::King,point:Point::new('d',1)}))),Point::new('d',1)),
            Square::new(RefCell::new(Some(Box::new(Queen{color:Color::White,name:Type::Queen,point:Point::new('e',1)}))),Point::new('e',1)),
            Square::new(RefCell::new(Some(Box::new(Bishop{color:Color::White,name:Type::Bishop,point:Point::new('f',1)}))),Point::new('f',1)),
            Square::new(RefCell::new(Some(Box::new(Knight{color:Color::White,name:Type::Knight,point:Point::new('g',1)}))),Point::new('g',1)),
            Square::new(RefCell::new(Some(Box::new(Rook{color:Color::White,name:Type::Rook,point:Point::new('h',1)}))),Point::new('h',1)),
        ],

    ]
       }
    }

    pub fn takes(&self, point:Point)->Option<Box<dyn Piece>>{
        for row in self.squares.iter(){
            for square in row{
                if point.notation()==square.point.notation(){
                    if square.piece.borrow().is_some(){
                        return square.piece.take();
                    }
                }
            }
        }
        None
    }

    pub fn replace(&self,point:Point,piece: Option<Box<dyn Piece>>)->Option<Box<dyn Piece>>{
        for row in self.squares.iter(){
            for square in row{
                if point.notation()==square.point.notation(){
                    return square.piece.borrow_mut().replace(piece.unwrap());
                }
            }
        }
        piece
    }

    pub fn get_prop(&self, select:&str)->Option<Props>{
        for row in self.squares.iter(){
            for square in row{

                if select.trim()==square.point.notation(){
                    if square.piece.borrow().is_some(){
                        return Some(square.piece.borrow().as_ref().unwrap().get_props());

                    } else {
                        return None;
                    }
                }
            }
        }
        None
    }

    pub fn draw(&self){
        for row in self.squares.iter(){
            for square in row{
                let piece= square.piece.borrow();

                if piece.as_ref().is_some(){
                    let props = piece.as_ref().unwrap().get_props();
                    print!(" {} ",props.get_name());
                } else{
                    print!(" {} ",' ');
                }
            }
            println!();
        }
    }
    
}