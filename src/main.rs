use std::{cell::{RefCell, Ref}, rc::Rc, borrow::BorrowMut};

use chess::{Type, Square, Color, Point, Props, piece::{Piece, Rook, Knight, Bishop, King, Queen, Pawn}, error, player::Player, board::Board};
use regex::Regex;
fn main() {

    let mut manager = ChessManager::new();
    manager.start();

}

struct ChessManager{
    board:Rc<Board>,
    is_running:bool,
    player1:Player,
    player2:Player
}
impl ChessManager{
    fn new()->ChessManager{
        let board = Rc::new(Board::new());
        ChessManager{
            board:Rc::clone(&board),
            is_running:false,
            player1:Player{board:Rc::clone(&board),color:Color::White},
            player2:Player{board:Rc::clone(&board),color:Color::Black}
        }
    }
    fn start(&mut self){
        self.is_running = true;
        //self.draw_board(Color::White);
        

        while self.is_running{
            let mut valid=false;

            while !valid{
                self.draw_board(Color::White);
                let (square,square2) = self.player1.select();
                valid = self.pre_check(square,square2);
                if !valid{
                    continue;
                }

                if self.is_castling(square, square2){
                    self.castling(square, square2);
                    continue;
                }else if self.is_en_passant(square, square2){
                    self.en_passant();
                    continue;
                } else {
                    let mut piece = self.board.takes(square.point);
                    piece= self.board.replace(square2.point, piece);
                    //promotion
                    if self.is_promotion(square2){
                        self.promotion(square2)
                    }

                    self.post_check(square,square2);
                }

                

            }


        }
    }
    fn stop(&mut self){
        self.is_running = false;
    }
    fn pre_check(&self,square:&Square,square2:&Square)-> bool {
        self.is_in_range(square,square2) && 
        !self.is_blocked(square.point,square2.point)
    }
    fn post_check(&self,square:&Square,square2:&Square){
        //check

        //check mate

    }
    fn is_promotion(&self,square:&Square)->bool{
        if square.piece.borrow().as_ref().unwrap().get_props().name==Type::Pawn{

            if square.point.rank==8 && square.piece.borrow().as_ref().unwrap().get_props().color==Color::White{
                return true;
            }
            if square.point.rank==1 && square.piece.borrow().as_ref().unwrap().get_props().color==Color::Black{
                return true;
            }
        }
        false
    }
    fn promotion(&self,square:&Square){
        let color = square.piece.borrow().as_ref().unwrap().get_props().color;
        let mut pick = String::new();
        std::io::stdin().read_line(&mut pick).expect("failed to readline");
        let piece:Option<Box<dyn Piece>> =match &*pick.trim(){
            "queen"=>Some(Box::new(Queen::new(color,Type::Queen))),
            "rook"=>Some(Box::new(Rook::new(color,Type::Rook))),
            "bishop"=>Some(Box::new(Bishop::new(color,Type::Bishop))),
            "knight"=>Some(Box::new(Knight::new(color,Type::Knight))),
            _=>None
        };
        *square.piece.borrow_mut()=piece
    }
    fn is_en_passant(&self,square:&Square,square2:&Square)->bool{
        if square.piece.borrow().as_ref().is_some(){
            if square.point.rank==5 && square.piece.borrow().as_ref().unwrap().get_props().color==Color::White{

            }

            if square.point.rank==4 && square.piece.borrow().as_ref().unwrap().get_props().color==Color::Black{

            }
        } 

        false
    }
    fn en_passant(&self){
        
    }
    fn is_castling(&self,square:&Square,square2:&Square)->bool{
        square.piece.borrow().is_some() 
        && square.piece.borrow().as_ref().unwrap().get_props().name==Type::King
        && square2.piece.borrow().is_none()
    }
    fn castling(&self,square:&Square,square2:&Square)->bool{
        let is_king_moved = square.piece.borrow().as_ref().unwrap().is_moved();
        let rook= if square2.point.file>'e'{
            if square.piece.borrow().as_ref().unwrap().get_props().color==Color::White{
                //h1
                self.board.get_square("h1").unwrap()
            } else{
                //h8
                self.board.get_square("h8").unwrap()
            }
        } else{
            if square.piece.borrow().as_ref().unwrap().get_props().color==Color::White{
                //a1
                self.board.get_square("a1").unwrap()
            } else{
                //a8
                self.board.get_square("a8").unwrap()
            }
        };

        let is_rook_moved = if rook.piece.borrow().as_ref().is_some(){
            rook.piece.borrow().as_ref().unwrap().is_moved()
        }else{
            true
        };


        let squares =self.board.get_pieces_by_color(
            if square.piece.borrow().as_ref().unwrap().get_props().color==Color::White{
                Color::Black
            }else{
                Color::White
            }
        );
        
        let mut is_king_under_check= false;
        for s in squares{
            for t in square.piece.borrow().as_ref().unwrap().points_between(square.point, square2.point){
                let points = s.piece.borrow().as_ref().unwrap().moves(s.point);
                if points.contains(&t) && !self.is_blocked(s.point,t){
                    is_king_under_check = true;
                    break;
                }
            }
        }
        
        let is_piece_between = self.is_blocked(rook.point, square.point);

        if !is_king_moved && !is_rook_moved && !is_king_under_check && !is_piece_between{
            let mut piece = self.board.takes(square.point);
            piece= self.board.replace(square2.point, piece);

            let file = 
            if rook.point.file=='a'{'d'}else{'f'};
            let p = Point::new(file,rook.point.rank);
            piece = self.board.takes(rook.point);
            piece= self.board.replace(p, piece);

            return true;        
        }

        false
    }


    fn pomotion(&self)->bool{
        false
    }

    
    
    fn is_in_range(&self,square:&Square,square2:&Square)->bool{
        let points = square.piece.borrow().as_ref().unwrap().moves(square.point);
        if points.contains(&square2.point){
            return true;
        } else{
            println!("{} is not in range!",square2.point.notation());
            return false;
        }
    }
    fn is_blocked(&self,point:Point,point2:Point)->bool{
        if self.board.is_blocked(point, point2){
            println!("is blocked");
            return true;
        } else{
            return false;
        }
    }


    fn draw_board(&self,color:Color){
        self.board.draw(color);
    }
}
