use std::rc::Rc;

use crate::{Square, Color, board::Board};

pub struct Player{
    pub board:Rc<Board>,
    pub color:Color
}

impl Player{

    pub fn select_piece(&self)->&Square{
        loop{
            println!("Select piece");
            let mut select = String::new();
            std::io::stdin().read_line(&mut select).expect("failed to readline");
            let src_square =self.board.get_square(&select);
            match(src_square){
                Ok(s)=>{
                    if s.piece.borrow().is_some() {
                        if s.props().color == self.color{
                            return s;
                        }else{
                            println!("{} is not your piece",s.point.to_string());
                        }
                    }else{
                        continue;
                    }
                },
                Err(e)=>{
                    println!("{}",e);
                    continue;
                }
            }
        }
    }

    pub fn select_moving_point(&self)->&Square{
        loop{
            println!("Select point");
            let mut target = String::new();
            std::io::stdin().read_line(&mut target).expect("failed to readline");
            let dest_square =self.board.get_square(&target);
            match(dest_square){
                Ok(s)=>{
                    if s.piece.borrow().is_some() && s.props().color == self.color{
                        println!("{} is your piece",s.point.to_string());
                        continue;
                    }
                    return s;
                },
                Err(e)=>{
                    println!("{}",e);
                    continue;
                }
            }
        }
    }

}