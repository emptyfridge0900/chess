use std::{cell::{RefCell, Ref}, rc::Rc, borrow::BorrowMut};

use chess::{Type, Square, Color, Point, Props, piece::{Piece, Rook, Knight, Bishop, King, Queen, Pawn}, error, player::Player, board::Board};
use regex::Regex;
fn main() {

    let mut manager = ChessManager::new();
    manager.settting();
    manager.start();

}

struct Record{
    name:Type,
    src:Point,
    des:Point
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

    fn piece(&self, name:Type,color:Color)->Option<Box<dyn Piece>>{
        match name{
            Type::King=>Some(Box::new(King::new(color,Rc::clone(&self.board)))),
            Type::Queen=>Some(Box::new(Queen::new(color,Rc::clone(&self.board)))),
            Type::Rook=>Some(Box::new(Rook::new(color,Rc::clone(&self.board)))),
            Type::Bishop=>Some(Box::new(Bishop::new(color,Rc::clone(&self.board)))),
            Type::Knight=>Some(Box::new(Knight::new(color,Rc::clone(&self.board)))),
            Type::Pawn=>Some(Box::new(Pawn::new(color,Rc::clone(&self.board)))),
        }
    }

    fn settting(&self){
        for (rank,row) in self.board.squares.iter().enumerate(){
            for (file,square) in row.iter().enumerate(){
                if rank==0 || rank == 1|| rank==6||rank==7 {
                    let p =match rank{
                        1|6 => self.piece(Type::Pawn,if rank==1{Color::Black}else{Color::White}),
                        0|7 => match file {
                            0|7=>self.piece(Type::Rook, if rank==0{Color::Black}else{Color::White}),
                            //1|6=>self.piece(Type::Knight, if rank==0{Color::Black}else{Color::White}),
                            //2|5=>self.piece(Type::Bishop, if rank==0{Color::Black}else{Color::White}),
                            //3=>self.piece(Type::Queen, if rank==0{Color::Black}else{Color::White}),
                            4=>self.piece(Type::King, if rank==0{Color::Black}else{Color::White}),
                            _=>None
                        },
                        _=>None,
                    };
                    *square.piece.borrow_mut()=p;
                }
            }
        }
    }

    fn convert(&self,color:Color,name:Type,src:Point,des:Point)->String{
        let n = match name{
            Type::King=>'K',
            Type::Queen=>'Q',
            Type::Rook=>'R',
            Type::Bishop=>'B',
            Type::Knight=>'N',
            Type::Pawn=>' ',
        };

        let pieces = self.board.get_pieces_by_color(color);
        let notation = pieces.iter()
        .filter(|x|{
            x.piece.borrow().as_ref().unwrap().get_props().name == name
        })
        .map(|x|{
            let m = x.piece.borrow();
            let p = m.as_ref().unwrap();
            if p.get_props().name==name && x.point!=src{//same piece but different point
                let points = p.moves();
                if points.contains(&des) //&& !self.is_blocked(x.point, des)
                {
                    Some(x.point)
                } else {
                    None
                }
            } else{
                None
            }
        })
        .map(|x|{
            let notation =if x.is_some(){
                let p = x.unwrap();
                let mut  r = String::new();
                if src.rank == p.rank{
                    r.push_str(&src.file.to_string());
                }
                if src.file == p.file{
                    r.push_str(&src.rank.to_string());
                }
                r
            }else{
                String::new()
            };
            notation
        })
        .reduce(|acc,e|{
            if acc.len()>e.len(){acc}else{e}
        });



        let mut result = n.to_string();
        if notation.is_some(){
            result.push_str(&notation.unwrap());
        }
        result.push_str(&des.notation());
        result
    }

    fn start(&mut self){
        

        self.is_running = true;
        //self.draw_board(Color::White);

        let mut white_record:Vec<String> = vec![];
        let mut black_record:Vec<String> = vec![];
        

        while self.is_running{
            let mut valid=false;

            while !valid{
                self.draw_board(Color::White);
                let (square,square2) = self.player1.select();
                println!("{:?}",square.piece.borrow().as_ref().unwrap().moves());
                valid = self.pre_check(square,square2);
                if !valid{
                    continue;
                }

                if self.is_castling(square, square2){
                    self.castling(square, square2);
                    let n =self.convert(Color::White, square.piece.borrow().as_ref().unwrap().get_props().name, square.point, square2.point);
                    white_record.push(n);
                    continue;
                }else if self.is_en_passant(square, square2){
                    self.en_passant();
                    continue;
                } else {
                    let n =self.convert(Color::White, square.piece.borrow().as_ref().unwrap().get_props().name, square.point, square2.point);
                    white_record.push(n);


                    let mut piece = self.board.takes(square.point);
                    piece= self.board.replace(square2.point, piece);
                    //promotion
                    if self.is_promotion(square2){
                        self.promotion(square2)
                    }
                    self.post_check(square,square2);
                }
            }


            for record in white_record.iter(){
                println!("{:?}",record);
            }

            // let mut valid=false;

            // while !valid{
            //     self.draw_board(Color::Black);
            //     let (square,square2) = self.player2.select();
            //     valid = self.pre_check(square,square2);
            //     if !valid{
            //         continue;
            //     }

            //     if self.is_castling(square, square2){
            //         self.castling(square, square2);
            //         let n =self.convert(Color::White, square.piece.borrow().as_ref().unwrap().get_props().name, square.point, square2.point);
            //         white_record.push(n);
            //         continue;
            //     }else if self.is_en_passant(square, square2){
            //         self.en_passant();
            //         continue;
            //     } else {

            //         let n =self.convert(Color::White, square.piece.borrow().as_ref().unwrap().get_props().name, square.point, square2.point);
            //         white_record.push(n);

            //         let mut piece = self.board.takes(square.point);
            //         piece= self.board.replace(square2.point, piece);
            //         //promotion
            //         if self.is_promotion(square2){
            //             self.promotion(square2)
            //         }
            //         self.post_check(square,square2);
            //     }

            // }
            for record in std::iter::zip(white_record.iter(),black_record.iter()){
                println!("{}|{}",  record.0, record.1);
            }

        }
    }
    fn stop(&mut self){
        self.is_running = false;
    }
    fn pre_check(&self,square:&Square,square2:&Square)-> bool {
        self.is_in_range(square,square2) 
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
            "queen"=>Some(Box::new(Queen::new(color,Rc::clone(&self.board)))),
            "rook"=>Some(Box::new(Rook::new(color,Rc::clone(&self.board)))),
            "bishop"=>Some(Box::new(Bishop::new(color,Rc::clone(&self.board)))),
            "knight"=>Some(Box::new(Knight::new(color,Rc::clone(&self.board)))),
            _=>None
        };
        *square.piece.borrow_mut()=piece
    }
    fn is_en_passant(&self,square:&Square,square2:&Square)->bool{
        if square.piece.borrow().is_some(){
            if square.point.rank==5 && square.piece.borrow().as_ref().unwrap().get_props().color==Color::White{

            }

            if square.point.rank==4 && square.piece.borrow().as_ref().unwrap().get_props().color==Color::Black{

            }
        } 

        false
    }
    fn en_passant(&self){
        
    }

    fn is_under_check(&self,color:Color)->bool{

        let squares = self.board.get_pieces_by_color(color);
        let king =squares
        .iter()
        .find(|x|x.piece.borrow().as_ref().unwrap().get_props().name==Type::King)
        .unwrap();

        let enimies = self.board.get_pieces_by_color(if color==Color::White{Color::Black}else{Color::White});
        
        let mut is_king_under_check= false;
        for s in enimies{
            let points = s.piece.borrow().as_ref().unwrap().moves();
            if points.contains(&king.point) {
                is_king_under_check = true;
                break;
            }
        }
        is_king_under_check
    }
    
    fn is_check_mate(&self,color:Color)->bool{
        let squares = self.board.get_pieces_by_color(color);
        let king =squares
        .iter()
        .find(|x|x.piece.borrow().as_ref().unwrap().get_props().name==Type::King)
        .unwrap();
        let moves =king.piece.borrow().as_ref().unwrap().moves();
        
        let enimies = self.board.get_pieces_by_color(if color==Color::White{Color::Black}else{Color::White});

        let mut is_king_under_check= false;
        'outer: for s in enimies{
            let points = s.piece.borrow().as_ref().unwrap().moves();
            for m in moves.iter(){
                if !points.contains(m) {
                    is_king_under_check = true;
                    break 'outer;
                }
            }
        }
        is_king_under_check
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

        let is_rook_moved = if rook.piece.borrow().is_some(){
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
            for t in square.piece.borrow().as_ref().unwrap().points_between(square2.point){
                let points = s.piece.borrow().as_ref().unwrap().moves();
                if points.contains(&t) {
                    is_king_under_check = true;
                    break;
                }
            }
        }
        

        if !is_king_moved && !is_rook_moved && !is_king_under_check {
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
        let points = square.piece.borrow().as_ref().unwrap().moves();
        if points.contains(&square2.point){
            return true;
        } else{
            println!("{} is not in range!",square2.point.notation());
            return false;
        }
    }


    fn draw_board(&self,color:Color){
        self.board.draw(color);
    }
}
