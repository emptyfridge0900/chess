use core::borrow;
use std::{cell::RefCell, rc::Rc, borrow::BorrowMut};
use std::ops::{Add, Sub};
fn main() {

    let p = Point::new('a',1);
    let mut board:[[Option<Square>; 8]; 8] =[
        [
            Some(Square::new(Some(Box::new(Queen{color:Color::Black,name:Type::King,point:p.clone()})),p.clone())),
            None,None,None,None,None,None,None
        ],
        [None,None,None,None,None,None,None,None],
        [None,None,None,None,None,None,None,None],
        [None,None,None,None,None,None,None,None],
        [None,None,None,None,None,None,None,None],
        [None,None,None,None,None,None,None,None],
        [None,None,None,None,None,None,None,None],
        [None,None,None,None,None,None,None,None],

    ];
    for n in &mut board{
        let con = n[0].take();
        let pi =con.unwrap().piece;
        println!("{:?}",pi.unwrap().moves());
    }


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
    rank:u32,
}
impl Point{
    fn new(file:char,rank:u32)->Point{
        Point{ file,rank }
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



struct ChessManager{
    board:[[Square; 8]; 8],
    piece:Vec<Box<dyn Piece>>
}
impl ChessManager{

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
// struct Rook{

// }
// impl Piece for Rook{
    
// }
// struct Bishop{

// }
// impl Piece for Bishop{
    
// }
// struct Knight{

// }
// impl Piece for Knight{
    
// }
// struct Pawn{

// }
// impl Piece for Pawn{
    
// }


