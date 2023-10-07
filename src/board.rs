use std::{cell::RefCell, borrow::BorrowMut};

use crate::{Square, piece::{Piece, Rook, Knight, Bishop, King, Queen, Pawn}, Color, Point, Type, Props, error};


pub struct Board{
    squares:[[Square; 8]; 8]
}
impl Board{
    pub fn new()->Board{
       Board{ 
        squares : [
        [
            Square::new(RefCell::new(Some(Box::new(Rook::new(Color::Black,Type::Rook)))),Point::new('a',8)),
            Square::new(RefCell::new(Some(Box::new(Knight::new(Color::Black,Type::Knight)))),Point::new('b',8)),
            Square::new(RefCell::new(Some(Box::new(Bishop::new(Color::Black,Type::Bishop)))),Point::new('c',8)),
            Square::new(RefCell::new(Some(Box::new(Queen::new(Color::Black,Type::Queen)))),Point::new('d',8)),
            Square::new(RefCell::new(Some(Box::new(King::new(Color::Black,Type::King)))),Point::new('e',8)),
            Square::new(RefCell::new(Some(Box::new(Bishop::new(Color::Black,Type::Bishop)))),Point::new('f',8)),
            Square::new(RefCell::new(Some(Box::new(Knight::new(Color::Black,Type::Knight)))),Point::new('g',8)),
            Square::new(RefCell::new(Some(Box::new(Rook::new(Color::Black,Type::Rook)))),Point::new('h',8)),
        ],
        [
            Square::new(RefCell::new(Some(Box::new(Pawn::new(Color::Black,Type::Pawn)))),Point::new('a',7)),
            Square::new(RefCell::new(Some(Box::new(Pawn::new(Color::Black,Type::Pawn)))),Point::new('b',7)),
            Square::new(RefCell::new(Some(Box::new(Pawn::new(Color::Black,Type::Pawn)))),Point::new('c',7)),
            Square::new(RefCell::new(Some(Box::new(Pawn::new(Color::Black,Type::Pawn)))),Point::new('d',7)),
            Square::new(RefCell::new(Some(Box::new(Pawn::new(Color::Black,Type::Pawn)))),Point::new('e',7)),
            Square::new(RefCell::new(Some(Box::new(Pawn::new(Color::Black,Type::Pawn)))),Point::new('f',7)),
            Square::new(RefCell::new(Some(Box::new(Pawn::new(Color::Black,Type::Pawn)))),Point::new('g',7)),
            Square::new(RefCell::new(Some(Box::new(Pawn::new(Color::Black,Type::Pawn)))),Point::new('h',7)),
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
            Square::new(RefCell::new(Some(Box::new(Pawn::new(Color::White,Type::Pawn)))),Point::new('a',2)),
            Square::new(RefCell::new(Some(Box::new(Pawn::new(Color::White,Type::Pawn)))),Point::new('b',2)),
            Square::new(RefCell::new(Some(Box::new(Pawn::new(Color::White,Type::Pawn)))),Point::new('c',2)),
            Square::new(RefCell::new(Some(Box::new(Pawn::new(Color::White,Type::Pawn)))),Point::new('d',2)),
            Square::new(RefCell::new(Some(Box::new(Pawn::new(Color::White,Type::Pawn)))),Point::new('e',2)),
            Square::new(RefCell::new(Some(Box::new(Pawn::new(Color::White,Type::Pawn)))),Point::new('f',2)),
            Square::new(RefCell::new(Some(Box::new(Pawn::new(Color::White,Type::Pawn)))),Point::new('g',2)),
            Square::new(RefCell::new(Some(Box::new(Pawn::new(Color::White,Type::Pawn)))),Point::new('h',2)),
        ],
        [
            Square::new(RefCell::new(Some(Box::new(Rook::new(Color::White,Type::Rook)))),Point::new('a',1)),
            Square::new(RefCell::new(Some(Box::new(Knight::new(Color::White,Type::Knight)))),Point::new('b',1)),
            Square::new(RefCell::new(Some(Box::new(Bishop::new(Color::White,Type::Bishop)))),Point::new('c',1)),
            Square::new(RefCell::new(Some(Box::new(Queen::new(Color::White,Type::Queen)))),Point::new('d',1)),
            Square::new(RefCell::new(Some(Box::new(King::new(Color::White,Type::King)))),Point::new('e',1)),
            Square::new(RefCell::new(Some(Box::new(Bishop::new(Color::White,Type::Bishop)))),Point::new('f',1)),
            Square::new(RefCell::new(Some(Box::new(Knight::new(Color::White,Type::Knight)))),Point::new('g',1)),
            Square::new(RefCell::new(Some(Box::new(Rook::new(Color::White,Type::Rook)))),Point::new('h',1)),
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

    pub fn get_square(&self, select:&str)->Result<&Square,error::InvalidateInutError>{
        for row in self.squares.iter(){
            for square in row{
                if select.trim()==square.point.notation(){
                    return Ok(&square);
                }
            }
        }
        Err(error::InvalidateInutError)
    }

    pub fn get_pieces(&self, color:Color)->Vec<&Square>{
        let mut vec=vec![];
        for row in self.squares.iter(){
            for square in row{
                let piece= square.piece.borrow();

                if piece.as_ref().is_some(){
                    if piece.as_ref().unwrap().get_props().color == color{
                        vec.push(square);
                    }
                }
            }
        }
        vec
    }

    pub fn under_move(&self, point:Point, point2:Point)-> bool{
        for row in self.squares.iter(){
            for square in row{
                let piece= square.piece.borrow();

                if piece.as_ref().is_some() && square.point == point{
                    let x = piece.as_ref().unwrap().tops(point);




                    let moves = piece.as_ref().unwrap().moves(point);
                    if moves.contains(&point2){
                        return true;
                    }else{
                        return false;
                    }

                    
                }
            }
        }

        false
    }

    pub fn is_blocked(&self,point:Point, point2:Point)->bool{

        false
    }


    pub fn draw(&self,color:Color){
        if color==Color::White{
            for row in self.squares.iter(){
                for square in row.iter(){
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
        } else{
            for row in self.squares.iter().rev(){
                for square in row.iter().rev(){
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
    
}