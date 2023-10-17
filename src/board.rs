use std::{cell::RefCell, borrow::BorrowMut};

use crate::{Square, piece::{Piece, Rook, Knight, Bishop, King, Queen, Pawn}, Color, Point, Type, Props, error};


pub struct Board{
    pub squares:[[Square; 8]; 8]
}
impl Board{
    pub fn new()->Board{
    Board{ 
        squares : [
        [
            Square::new(RefCell::new(None),Point::new('a',8)),
            Square::new(RefCell::new(None),Point::new('b',8)),
            Square::new(RefCell::new(None),Point::new('c',8)),
            Square::new(RefCell::new(None),Point::new('d',8)),
            Square::new(RefCell::new(None),Point::new('e',8)),
            Square::new(RefCell::new(None),Point::new('f',8)),
            Square::new(RefCell::new(None),Point::new('g',8)),
            Square::new(RefCell::new(None),Point::new('h',8)),
        ],
        [
            Square::new(RefCell::new(None),Point::new('a',7)),
            Square::new(RefCell::new(None),Point::new('b',7)),
            Square::new(RefCell::new(None),Point::new('c',7)),
            Square::new(RefCell::new(None),Point::new('d',7)),
            Square::new(RefCell::new(None),Point::new('e',7)),
            Square::new(RefCell::new(None),Point::new('f',7)),
            Square::new(RefCell::new(None),Point::new('g',7)),
            Square::new(RefCell::new(None),Point::new('h',7)),
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
            Square::new(RefCell::new(None),Point::new('a',2)),
            Square::new(RefCell::new(None),Point::new('b',2)),
            Square::new(RefCell::new(None),Point::new('c',2)),
            Square::new(RefCell::new(None),Point::new('d',2)),
            Square::new(RefCell::new(None),Point::new('e',2)),
            Square::new(RefCell::new(None),Point::new('f',2)),
            Square::new(RefCell::new(None),Point::new('g',2)),
            Square::new(RefCell::new(None),Point::new('h',2)),
        ],
        [
            Square::new(RefCell::new(None),Point::new('a',1)),
            Square::new(RefCell::new(None),Point::new('b',1)),
            Square::new(RefCell::new(None),Point::new('c',1)),
            Square::new(RefCell::new(None),Point::new('d',1)),
            Square::new(RefCell::new(None),Point::new('e',1)),
            Square::new(RefCell::new(None),Point::new('f',1)),
            Square::new(RefCell::new(None),Point::new('g',1)),
            Square::new(RefCell::new(None),Point::new('h',1)),
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
                    let p = piece.unwrap();
                    p.moved();
                    return square.piece.borrow_mut().replace(p);
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

    pub fn get_pieces_by_color(&self, color:Color)->Vec<&Square>{
        let mut vec=vec![];
        for row in self.squares.iter(){
            for square in row{
                let piece= square.piece.borrow();

                if piece.is_some() && piece.as_ref().unwrap().get_props().color == color{
                    vec.push(square);
                }
            }
        }
        vec
    }

    pub fn points_between(&self,point:Point,point2:Point)->Vec<Point>{
        let s = self.get_square(&point.notation());
        match s {
            Ok(square)=>{
                let piece= square.piece.borrow();
                if piece.is_some(){
                    piece.as_ref().unwrap().points_between(point2)
                }else{
                    vec![]
                }
            },
            Err(err)=>vec![]
        }
    }




    pub fn draw(&self,color:Color){
        if color==Color::White{
            for row in self.squares.iter(){
                print!("{}",row[0].point.rank);
                for square in row.iter(){
                    let piece= square.piece.borrow();
                    
                    if piece.is_some(){
                        let props = piece.as_ref().unwrap().get_props();
                        print!(" {} ",props.get_name());
                    } else{
                        print!(" {} ",' ');
                    }
                }
                println!();
            }
            println!("  a  b  c  d  e  f  g  h");
        } else{
            for row in self.squares.iter().rev(){
                print!("{}",row[0].point.rank);
                for square in row.iter().rev(){
                    let piece= square.piece.borrow();
                    
                    if piece.is_some(){
                        let props = piece.as_ref().unwrap().get_props();
                        print!(" {} ",props.get_name());
                    } else{
                        print!(" {} ",' ');
                    }
                }
                println!();
            }
            println!("  h  g  f  e  d  c  b  a");
        }
    }
    
}