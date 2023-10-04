use std::{cell::RefCell, rc::Rc, borrow::BorrowMut};

fn main() {

    let mut board:[[Option<Square>; 8]; 8] =[
        [
            Some(Square::new(Color::Black,Type::Rook)),
            Some(Square::new(Color::Black,Type::Knight)),
            Some(Square::new(Color::Black,Type::Bishop)),
            Some(Square::new(Color::Black,Type::King)),
            Some(Square::new(Color::Black,Type::Queen)),
            Some(Square::new(Color::Black,Type::Bishop)),
            Some(Square::new(Color::Black,Type::Knight)),
            Some(Square::new(Color::Black,Type::Rook))
        ],
        [
            Some(Square::new(Color::Black,Type::Pawn)),
            Some(Square::new(Color::Black,Type::Pawn)),
            Some(Square::new(Color::Black,Type::Pawn)),
            Some(Square::new(Color::Black,Type::Pawn)),
            Some(Square::new(Color::Black,Type::Pawn)),
            Some(Square::new(Color::Black,Type::Pawn)),
            Some(Square::new(Color::Black,Type::Pawn)),
            Some(Square::new(Color::Black,Type::Pawn))
        ],
        [None,None,None,None,None,None,None,None],
        [None,None,None,None,None,None,None,None],
        [None,None,None,None,None,None,None,None],
        [None,None,None,None,None,None,None,None],
        [
            Some(Square::new(Color::White,Type::Pawn)),
            Some(Square::new(Color::White,Type::Pawn)),
            Some(Square::new(Color::White,Type::Pawn)),
            Some(Square::new(Color::White,Type::Pawn)),
            Some(Square::new(Color::White,Type::Pawn)),
            Some(Square::new(Color::White,Type::Pawn)),
            Some(Square::new(Color::White,Type::Pawn)),
            Some(Square::new(Color::White,Type::Pawn))
        ],
        [
            Some(Square::new(Color::White,Type::Rook)),
            Some(Square::new(Color::White,Type::Knight)),
            Some(Square::new(Color::White,Type::Bishop)),
            Some(Square::new(Color::White,Type::King)),
            Some(Square::new(Color::White,Type::Queen)),
            Some(Square::new(Color::White,Type::Bishop)),
            Some(Square::new(Color::White,Type::Knight)),
            Some(Square::new(Color::White,Type::Rook))
        ]
    ];
    


}


enum Color {
    White,
    Black
}
enum Type{
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn
}
struct Square{
    color:Color,
    piece:Type
}

impl Square{
    fn new(color:Color,piece:Type)->Square{
        Square { 
            color: color, 
            piece: piece 
        }
    }
}


// struct Chess{
//     board:Board
// }
// impl Chess {
//     fn start(){

//     }
//     fn draw_board(){

//     }

    
// }




trait Piece{
    // fn get_color(&self)->Color;
    // fn set_board(&mut self,board:&Rc< [[Option<Box<dyn Piece>>; 8]; 8] >);
    fn get_type(&self);
    // fn can_move(&self,rank:char,file:u32)->bool;
    // fn set_squre(&self,rank:char,file:u32);
    // fn get_squre(&self)->String;
    // fn attack(&self,piece: Box<dyn Piece>);
    // fn can_attack(&self,piece: Box<dyn Piece>) -> bool;
    
}

struct King{
    board: Option<[[Option<Square>;8];8]>,
    rank:char,
    file:u32,
}
impl Piece for King{
    // fn can_move(&self)->bool {
    //     //todo!()
    //     true
    // }
    fn get_type(&self) {
        
    }

}
// struct Queen{

// }
// impl Piece for Queen{
    
// }
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


