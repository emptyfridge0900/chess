use std::{rc::Rc, cell::RefCell, borrow::BorrowMut};

use crate::{Square, piece::{Piece, Rook, Knight, Bishop, King, Queen, Pawn}, Color, Point, Type, board::Board, Props};

pub struct Player{
    pub board:Rc<Board>,
    pub color:Color
}

impl Player{

    fn move_piece(&self){
        
    }
    fn captures(&self,a:Point,b:Point){

        let piece = self.board.takes(a);
        let test =self.board.replace(b, piece);

    }


    fn select(&self)->(Option<Props>,Option<Props>){
        println!("Select piece");
        let mut select = String::new();
        std::io::stdin().read_line(&mut select).expect("failed to readline");

        println!("Destination");
        let mut target= String::new();
        std::io::stdin().read_line(&mut target).expect("failed to readline");

        let mut selected_props:Option<Props> =self.board.get_prop(&select);
        let mut target_props:Option<Props> = self.board.get_prop(&target);

        (selected_props,target_props)
    }


    pub fn turn(&self){

        let (selected_props,target_props)=self.select();

        if selected_props.is_some() && target_props.is_some(){
            let p = selected_props.as_ref().unwrap();
            let q = target_props.as_ref().unwrap();

            if p.color==self.color && q.color!=self.color{
                self.captures(p.point, q.point);
            }
        } else if selected_props.is_some() {

        } else {

        }
        // for row in self.board.iter(){
        //     for square in row{
        //         if target.trim()==square.point.notation(){
        //             if square.piece.borrow().is_none(){
        //                 *square.piece.borrow_mut() = selected_piece.take();
        //                 println!("{:?}",square.piece.borrow().as_ref().unwrap().get_props().color);
        //             }else {
        //                 //square.piece.replace(selected_square.unwrap());
        //                 *square.piece.borrow_mut() = selected_piece.take();
        //                 println!("{:?}",square.piece.borrow().as_ref().unwrap().get_props().name);
        //             }
        //         }
        //     }
        // }

    }
}