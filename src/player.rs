use std::{rc::Rc, cell::RefCell, borrow::BorrowMut};

use crate::{Square, piece::{Piece, Rook, Knight, Bishop, King, Queen, Pawn}, Color, Point, Type, board::Board, Props};

pub struct Player{
    pub board:Rc<Board>,
    pub color:Color
}

impl Player{

    pub fn select(&self)->(&Square,&Square){
        println!("{} player",self.color);
        loop{
            println!("Select piece");
            let mut select = String::new();
            std::io::stdin().read_line(&mut select).expect("failed to readline");

            println!("Destination");
            let mut target= String::new();
            std::io::stdin().read_line(&mut target).expect("failed to readline");

            let src_square =self.board.get_square(&select);
            let des_square = self.board.get_square(&target);
            println!("{:?}",src_square.as_ref().unwrap().point);
            match (src_square,des_square){
                (Ok(a), Ok(b))=>{
                    if a.piece.borrow().is_some() && a.piece.borrow().as_ref().unwrap().get_props().color == self.color 
                    {
                        if b.piece.borrow().is_none() || b.piece.borrow().as_ref().unwrap().get_props().color != self.color{
                            return (a,b);
                        }else{
                            println!("{} is not your piece",b.point.notation());
                        }
                    }else{
                        println!("{:?}",a.piece.borrow().is_some());
                        println!("{} is not your piece",a.point.notation());
                    }
                    continue;
                },
                (Ok(_), Err(_))=> {println!("retrying"); continue;},
                (Err(_), Ok(_)) => {println!("retrying"); continue;},
                (Err(_), Err(_)) => {println!("retrying"); continue;},
            };
        }
    }

}