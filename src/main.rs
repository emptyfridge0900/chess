use core::borrow;
use std::fmt::Binary;
use std::{cell::RefCell, rc::Rc, borrow::BorrowMut};
use std::ops::{Add, Sub};
fn main() {

    let mut board:[[Square; 8]; 8] =[
        [
            Square::new(Some(Box::new(Rook{color:Color::Black,name:Type::Rook,point:Point::new('a',8)})),Point::new('a',8)),
            Square::new(Some(Box::new(Knight{color:Color::Black,name:Type::Knight,point:Point::new('b',8)})),Point::new('b',8)),
            Square::new(Some(Box::new(Bishop{color:Color::Black,name:Type::Bishop,point:Point::new('c',8)})),Point::new('c',8)),
            Square::new(Some(Box::new(King{color:Color::Black,name:Type::King,point:Point::new('d',8)})),Point::new('d',8)),
            Square::new(Some(Box::new(Queen{color:Color::Black,name:Type::Queen,point:Point::new('e',8)})),Point::new('e',8)),
            Square::new(Some(Box::new(Bishop{color:Color::Black,name:Type::Bishop,point:Point::new('f',8)})),Point::new('f',8)),
            Square::new(Some(Box::new(Knight{color:Color::Black,name:Type::Knight,point:Point::new('g',8)})),Point::new('g',8)),
            Square::new(Some(Box::new(Rook{color:Color::Black,name:Type::Rook,point:Point::new('h',8)})),Point::new('h',8)),
        ],
        [
            Square::new(Some(Box::new(Pawn{color:Color::Black,name:Type::Pawn,point:Point::new('a',7)})),Point::new('a',7)),
            Square::new(Some(Box::new(Pawn{color:Color::Black,name:Type::Pawn,point:Point::new('b',7)})),Point::new('b',7)),
            Square::new(Some(Box::new(Pawn{color:Color::Black,name:Type::Pawn,point:Point::new('c',7)})),Point::new('c',7)),
            Square::new(Some(Box::new(Pawn{color:Color::Black,name:Type::Pawn,point:Point::new('d',7)})),Point::new('d',7)),
            Square::new(Some(Box::new(Pawn{color:Color::Black,name:Type::Pawn,point:Point::new('e',7)})),Point::new('e',7)),
            Square::new(Some(Box::new(Pawn{color:Color::Black,name:Type::Pawn,point:Point::new('f',7)})),Point::new('f',7)),
            Square::new(Some(Box::new(Pawn{color:Color::Black,name:Type::Pawn,point:Point::new('g',7)})),Point::new('g',7)),
            Square::new(Some(Box::new(Pawn{color:Color::Black,name:Type::Pawn,point:Point::new('h',7)})),Point::new('h',7)),
        ],
        [
            Square::new(None,Point::new('a',6)),
            Square::new(None,Point::new('b',6)),
            Square::new(None,Point::new('c',6)),
            Square::new(None,Point::new('d',6)),
            Square::new(None,Point::new('e',6)),
            Square::new(None,Point::new('f',6)),
            Square::new(None,Point::new('g',6)),
            Square::new(None,Point::new('h',6)),
        ],
        [
            Square::new(None,Point::new('a',5)),
            Square::new(None,Point::new('b',5)),
            Square::new(None,Point::new('c',5)),
            Square::new(None,Point::new('d',5)),
            Square::new(None,Point::new('e',5)),
            Square::new(None,Point::new('f',5)),
            Square::new(None,Point::new('g',5)),
            Square::new(None,Point::new('h',5)),
        ],
        [
            Square::new(None,Point::new('a',4)),
            Square::new(None,Point::new('b',4)),
            Square::new(None,Point::new('c',4)),
            Square::new(None,Point::new('d',4)),
            Square::new(None,Point::new('e',4)),
            Square::new(None,Point::new('f',4)),
            Square::new(None,Point::new('g',4)),
            Square::new(None,Point::new('h',4)),
        ],
        [
            Square::new(None,Point::new('a',3)),
            Square::new(None,Point::new('b',3)),
            Square::new(None,Point::new('c',3)),
            Square::new(None,Point::new('d',3)),
            Square::new(None,Point::new('e',3)),
            Square::new(None,Point::new('f',3)),
            Square::new(None,Point::new('g',3)),
            Square::new(None,Point::new('h',3)),
        ],
        [
            Square::new(Some(Box::new(Pawn{color:Color::White,name:Type::Pawn,point:Point::new('a',2)})),Point::new('a',2)),
            Square::new(Some(Box::new(Pawn{color:Color::White,name:Type::Pawn,point:Point::new('b',2)})),Point::new('b',2)),
            Square::new(Some(Box::new(Pawn{color:Color::White,name:Type::Pawn,point:Point::new('c',2)})),Point::new('c',2)),
            Square::new(Some(Box::new(Pawn{color:Color::White,name:Type::Pawn,point:Point::new('d',2)})),Point::new('d',2)),
            Square::new(Some(Box::new(Pawn{color:Color::White,name:Type::Pawn,point:Point::new('e',2)})),Point::new('e',2)),
            Square::new(Some(Box::new(Pawn{color:Color::White,name:Type::Pawn,point:Point::new('f',2)})),Point::new('f',2)),
            Square::new(Some(Box::new(Pawn{color:Color::White,name:Type::Pawn,point:Point::new('g',2)})),Point::new('g',2)),
            Square::new(Some(Box::new(Pawn{color:Color::White,name:Type::Pawn,point:Point::new('h',2)})),Point::new('h',2)),
        ],
        [
            Square::new(Some(Box::new(Rook{color:Color::White,name:Type::Rook,point:Point::new('a',1)})),Point::new('a',1)),
            Square::new(Some(Box::new(Knight{color:Color::White,name:Type::Knight,point:Point::new('b',1)})),Point::new('b',1)),
            Square::new(Some(Box::new(Bishop{color:Color::White,name:Type::Bishop,point:Point::new('c',1)})),Point::new('c',1)),
            Square::new(Some(Box::new(King{color:Color::White,name:Type::King,point:Point::new('d',1)})),Point::new('d',1)),
            Square::new(Some(Box::new(Queen{color:Color::White,name:Type::Queen,point:Point::new('e',1)})),Point::new('e',1)),
            Square::new(Some(Box::new(Bishop{color:Color::White,name:Type::Bishop,point:Point::new('f',1)})),Point::new('f',1)),
            Square::new(Some(Box::new(Knight{color:Color::White,name:Type::Knight,point:Point::new('g',1)})),Point::new('g',1)),
            Square::new(Some(Box::new(Rook{color:Color::White,name:Type::Rook,point:Point::new('h',1)})),Point::new('h',1)),
        ],

    ];

    let mut manager = ChessManager::new(board);
    manager.start();


}

#[derive(Clone,Debug)]
enum Color {
    White,
    Black
}
#[derive(Clone,Debug)]
enum Type{
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn
}
#[derive(Clone,Debug)]
struct Point{
    file:char,
    rank:u32
}
impl Point{
    fn new(file:char,rank:u32)->Point{
        Point{ file,rank }
    }
    fn notation(&self)->String{
        format!("{}{}",self.file,self.rank)
    }
}

struct Square{
    piece:Option<Box<dyn Piece>>,
    point:Point
}
impl Square{
    fn new(piece:Option<Box<dyn Piece>>,point:Point)->Square{
        Square { piece, point }
    }
}
struct Player{

}
impl Player{
    fn select_piece(&self)->String{
        let mut guess = String::new();
        std::io::stdin().read_line(&mut guess).expect("failed to readline");
        guess
    }
    fn move_piece(&self,input:&str){
        
    }
}



struct ChessManager{
    board:[[Square; 8]; 8],
    isRunning:bool,
    player1:Player,
    player2:Player
}
impl ChessManager{
    fn new(board:[[Square; 8]; 8])->ChessManager{
        ChessManager{
            board,
            isRunning:false,
            player1:Player{},
            player2:Player{}
        }
    }
    fn start(&mut self){
        self.isRunning = true;
        while self.isRunning{
            let select = self.player1.select_piece();
            let mut selected_square:Option<Box<dyn Piece>> =None;
            self.player1.move_piece(&select);
            for row in self.board.iter_mut(){
                for square in row.iter_mut(){
                    if select.trim()==square.point.notation(){
                        if square.piece.is_some(){
                            
                            let hmm = square.piece.take().unwrap();
                            selected_square = Some(hmm);
                            println!("{:?}",selected_square.as_ref().unwrap().get_type());
                        }
                    }
                }
            }

            println!("your target");
            let mut target= String::new();
            std::io::stdin().read_line(&mut target).expect("failed to readline");

            for row in self.board.iter_mut(){
                for square in row.iter_mut(){
                    if target.trim()==square.point.notation(){
                        if square.piece.is_none(){
                            square.piece = selected_square.take();
                            println!("{:?}",square.piece.as_ref().unwrap().get_type());
                        }else {
                            //square.piece.replace(selected_square.unwrap());
                            square.piece = selected_square.take();
                            println!("{:?}",square.piece.as_ref().unwrap().get_type());
                        }
                    }
                }
            }


        }
    }
    fn stop(&mut self){
        self.isRunning = false;
    }
}


trait Piece{
    fn get_color(&self)->Color;
    // fn set_board(&mut self,board:&Rc< [[Option<Box<dyn Piece>>; 8]; 8] >);
    fn get_type(&self)->Type;
    // fn available_move(&self)->Vec<Option<Point>>;
    // fn can_move(&self,rank:char,file:u32)->bool;
    // fn set_squre(&self,rank:char,file:u32);
    // fn get_squre(&self)->String;
    // fn attack(&self,piece: Box<dyn Piece>);
    // fn can_attack(&self,piece: Box<dyn Piece>) -> bool;
    fn moves(&self)->Vec<Point>;
    fn surrounding_points(&self,file:char,rank:u32)->Vec<Option<Point>>{
        vec![
            self.top_left(file, rank),
            self.top(file, rank),
            self.top_right(file, rank),
            self.left(file, rank),
            self.right(file, rank),
            self.bottom_left(file, rank),
            self.bottom(file, rank),
            self.bottom_right(file, rank),
        ]
    }
    fn top_right(&self,file:char,rank:u32)->Option<Point>{
        let right = (file as u8 +1) as char;
        let top = rank + 1;
        if right>='a' && right<='h' && top>=1 && top<=8{
            return Some(Point::new(right, top));
        }
        None
    }
    fn top_left(&self,file:char,rank:u32)->Option<Point>{
        let left = (file as u8 -1) as char;
        let top = rank + 1;
        if left>='a' && left<='h' && top>=1 && top<=8{
            return Some(Point::new(left, top));
        }
        None
    }
    fn bottom_right(&self,file:char,rank:u32)->Option<Point>{
        let right = (file as u8 +1) as char;
        let bottom = rank-1;
        if right>='a' && right<='h' && bottom>=1 && bottom<=8{
            return Some(Point::new(right, bottom));
        }
        None
    }
    fn bottom_left(&self,file:char,rank:u32)->Option<Point>{
        let left = (file as u8 -1) as char;
        let bottom = rank-1;
        if left>='a' && left<='h' && bottom>=1 && bottom<=8{
            return Some(Point::new(left, bottom));
        }
        None
    }
    fn left(&self,file:char,rank:u32)->Option<Point>{
        let left = (file as u8 -1) as char;
        if left>='a' && left<='h'{
            return Some(Point::new(left, rank));
        }
        None
    }
    fn right(&self,file:char,rank:u32)->Option<Point>{
        let right = (file as u8 +1) as char;
        if right>='a' && right<='h'{
            return Some(Point::new(right, rank ));
        }
        None
    }
    fn top(&self,file:char,rank:u32)->Option<Point>{
        let top = rank + 1;
        if top>=1 && top<=8 {
            return Some(Point::new(file,top));
        }
        None
    }
    fn bottom(&self,file:char,rank:u32)->Option<Point>{
        let bottom = rank-1;
        if bottom>=1 && bottom<=8{
            return Some(Point::new(file,bottom));
        }
        None
    }
    
}

struct King{
    color:Color,
    name:Type,
    point:Point

}
impl Piece for King{

    fn get_color(&self)->Color {
        self.color.clone()
    }
    fn get_type(&self)->Type {
       self.name.clone()
    }

    // fn available_move(&self)->Vec<Option<Point>> {
    //     self.surrounding_points(self.point.file, self.point.rank)
    // }

    fn moves(&self)->Vec<Point> {
        self.surrounding_points(self.point.file, self.point.rank).iter()
        .filter_map(|x|x.clone())
        .collect()
    }
    
}
struct Queen{
    color:Color,
    name:Type,
    point:Point
}
impl Piece for Queen{
    fn get_color(&self)->Color {
        self.color.clone()
    }

    fn get_type(&self)->Type {
        self.name.clone()
    }

    // fn available_move(&self)->Vec<Option<Point>> {
    //     todo!()
    // }

    fn moves(&self)->Vec<Point> {
        let mut vec:Vec<Point>=vec![];

        let mut next = self.right(self.point.file, self.point.rank);
        while let Some(point) = next{
            vec.push(point.clone());
            next=self.right(point.file,point.rank);
        }
        next = self.top_right(self.point.file, self.point.rank);
        while let Some(point) = next{
            vec.push(point.clone());
            next = self.top_right(point.file,point.rank);
        }
        next = self.top(self.point.file, self.point.rank);
        while let Some(point) = next{
            vec.push(point.clone());
            next = self.top(point.file,point.rank);
        }
        next = self.top_left(self.point.file, self.point.rank);
        while let Some(point) = next{
            vec.push(point.clone());
            next = self.top_left(point.file,point.rank);
        }
        next = self.left(self.point.file, self.point.rank);
        while let Some(point) = next{
            vec.push(point.clone());
            next = self.left(point.file,point.rank);
        }
        next = self.bottom_left(self.point.file, self.point.rank);
        while let Some(point) = next{
            vec.push(point.clone());
            next = self.bottom_left(point.file,point.rank);
        }
        next = self.bottom(self.point.file, self.point.rank);
        while let Some(point) = next{
            vec.push(point.clone());
            next = self.bottom(point.file,point.rank);
        }
        next = self.bottom_right(self.point.file, self.point.rank);
        while let Some(point) = next{
            vec.push(point.clone());
            next = self.bottom_right(point.file,point.rank);
        }

        vec

    }
}
struct Rook{
color:Color,
    name:Type,
    point:Point
}
impl Piece for Rook{
    fn get_color(&self)->Color {
        self.color.clone()
    }

    fn get_type(&self)->Type {
        self.name.clone()
    }

    fn moves(&self)->Vec<Point> {
        let mut vec:Vec<Point>=vec![];

        let mut next = self.right(self.point.file, self.point.rank);
        while let Some(point) = next{
            vec.push(point.clone());
            next=self.right(point.file,point.rank);
        }
        next = self.top(self.point.file, self.point.rank);
        while let Some(point) = next{
            vec.push(point.clone());
            next = self.top(point.file,point.rank);
        }
        next = self.left(self.point.file, self.point.rank);
        while let Some(point) = next{
            vec.push(point.clone());
            next = self.left(point.file,point.rank);
        }
        next = self.bottom(self.point.file, self.point.rank);
        while let Some(point) = next{
            vec.push(point.clone());
            next = self.bottom(point.file,point.rank);
        }
        vec
    }
}
struct Bishop{
color:Color,
    name:Type,
    point:Point
}
impl Piece for Bishop{
    fn get_color(&self)->Color {
        self.color.clone()
    }

    fn get_type(&self)->Type {
        self.name.clone()
    }

    fn moves(&self)->Vec<Point> {
        let mut vec:Vec<Point>=vec![];
        let mut next = self.top_right(self.point.file, self.point.rank);
        while let Some(point) = next{
            vec.push(point.clone());
            next=self.top_right(point.file,point.rank);
        }
        next=self.top_left(self.point.file, self.point.rank);
        while let Some(point) = next{
            vec.push(point.clone());
            next=self.top_left(point.file,point.rank);
        }
        next=self.bottom_left(self.point.file, self.point.rank);
        while let Some(point) = next{
            vec.push(point.clone());
            next=self.bottom_left(point.file,point.rank);
        }
        next=self.bottom_right(self.point.file, self.point.rank);
        while let Some(point) = next{
            vec.push(point.clone());
            next=self.bottom_right(point.file,point.rank);
        }
        vec
    }
}
struct Knight{
color:Color,
    name:Type,
    point:Point
}
impl Piece for Knight{
    fn get_color(&self)->Color {
        self.color.clone()
    }

    fn get_type(&self)->Type {
        self.name.clone()
    }

    fn moves(&self)->Vec<Point> {
        let mut vec:Vec<Point>=vec![];
        let mut next = self.top(self.point.file, self.point.rank);

        if let Some(point) = next{
            next=self.top_left(point.file,point.rank);
            if let Some(p)=next{
                vec.push(p);
            }
            next=self.top_right(point.file,point.rank);
            if let Some(p)=next{
                vec.push(p);
            }
        }

        next = self.right(self.point.file, self.point.rank);

        if let Some(point) = next{
            next=self.top_right(point.file,point.rank);
            if let Some(p)=next{
                vec.push(p);
            }
            next=self.bottom_right(point.file,point.rank);
            if let Some(p)=next{
                vec.push(p);
            }
        }

        next = self.bottom(self.point.file, self.point.rank);

        if let Some(point) = next{
            next=self.bottom_left(point.file,point.rank);
            if let Some(p)=next{
                vec.push(p);
            }
            next=self.bottom_right(point.file,point.rank);
            if let Some(p)=next{
                vec.push(p);
            }
        }

        next = self.left(self.point.file, self.point.rank);

        if let Some(point) = next{
            next=self.top_left(point.file,point.rank);
            if let Some(p)=next{
                vec.push(p);
            }
            next=self.bottom_left(point.file,point.rank);
            if let Some(p)=next{
                vec.push(p);
            }
        }
        
        vec
    }
}
struct Pawn{
color:Color,
    name:Type,
    point:Point
}
impl Piece for Pawn{
    fn get_color(&self)->Color {
        self.color.clone()
    }

    fn get_type(&self)->Type {
        self.name.clone()
    }

    fn moves(&self)->Vec<Point> {
        vec![]
    }
}


