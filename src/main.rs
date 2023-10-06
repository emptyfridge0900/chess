use std::{cell::RefCell, rc::Rc};

use chess::{Type, Square, Color, Point, Props, piece::{Piece, Rook, Knight, Bishop, King, Queen, Pawn}};

fn main() {

    let mut board:Rc<[[Square; 8]; 8]> =Rc::new([
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

    ]);

    let mut manager = ChessManager::new(board);
    manager.start();


}



struct Player{
    board:Rc<[[Square; 8]; 8]>,
    color:Color
}

impl Player{

    fn move_piece(&self){
        
    }
    fn captures(&self,a:Point,b:Point){
        let mut p : Option<Box<dyn Piece>> =None;
        for row in self.board.iter(){
            for square in row{

                if a.notation()==square.point.notation(){
                    if square.piece.borrow().is_some(){
                        p = square.piece.take();
                        let prop=p.as_ref().unwrap().get_props();
                        println!("{:?} {:?}",prop.color,prop.name);
                    }
                }
            }
        }

        for row in self.board.iter(){
            for square in row{
                if b.notation()==square.point.notation(){
                    if square.piece.borrow().is_some(){
                        let mut x = p.take().unwrap();
                        x.set_point(b);
                        square.piece.replace(Some(x));
                    }
                }

            }
        }

    }


    fn select(&self)->(Option<Props>,Option<Props>){
        println!("Select piece");
        let mut select = String::new();
        std::io::stdin().read_line(&mut select).expect("failed to readline");

        println!("Destination");
        let mut target= String::new();
        std::io::stdin().read_line(&mut target).expect("failed to readline");

        let mut selected_props:Option<Props> =None;
        let mut target_props:Option<Props> = None;
        for row in self.board.iter(){
            for square in row{

                if select.trim()==square.point.notation(){
                    if square.piece.borrow().is_some(){
                        selected_props = Some(square.piece.borrow().as_ref().unwrap().get_props());

                    } else {
                        selected_props = None;
                    }
                }
                if target.trim()==square.point.notation(){
                    if square.piece.borrow().is_some(){
                        target_props = Some(square.piece.borrow().as_ref().unwrap().get_props());
                    } else {
                        target_props = None;
                    }
                }
            }
        }
        (selected_props,target_props)
    }


    fn turn(&self){

        let (selected_props,target_props)=self.select();
        

        if selected_props.is_some() && target_props.is_some(){
            let p = selected_props.as_ref().unwrap();
            let q = target_props.as_ref().unwrap();
            if p.color==self.color && q.color!=self.color{
                println!("let's capture");
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

struct ChessManager{
    board:Rc<[[Square; 8]; 8]>,
    isRunning:bool,
    player1:Player,
    player2:Player
}
impl ChessManager{
    fn new(board:Rc<[[Square; 8]; 8]>)->ChessManager{
        ChessManager{
            board:Rc::clone(&board),
            isRunning:false,
            player1:Player{board:Rc::clone(&board),color:Color::White},
            player2:Player{board:Rc::clone(&board),color:Color::Black}
        }
    }
    fn start(&mut self){
        self.isRunning = true;
        while self.isRunning{
            self.player1.turn();

        }
    }
    fn stop(&mut self){
        self.isRunning = false;
    }
}
