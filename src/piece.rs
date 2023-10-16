
use std::{cell::RefCell, rc::Rc, time::SystemTime};

use crate::{Type, Point, Props, Color, board::Board};


pub trait Piece{
    fn get_props(&self)->Props;
    fn moves(&self)->Vec<Point>;
    fn points_between(&self,point:Point,point2:Point)->Vec<Point>;
    fn is_moved(&self)->bool;
    fn moved(&self);

    fn surrounding_points(&self,point:Point)->Vec<Point>{
        vec![
            self.top_left(point),
            self.top(point),
            self.top_right(point),
            self.left(point),
            self.right(point),
            self.bottom_left(point),
            self.bottom(point),
            self.bottom_right(point),
        ].iter()
        .filter_map(|x|x.clone())
        .collect()
    }

    fn top_lefts(&self,point:Point)->Vec<Point>{
        let mut vec:Vec<Point> =vec![];
        let mut next = self.top_left(point);
        while let Some(point) = next{
            vec.push(point.clone());
            next = self.top_left(point);
        } 
        vec
    }
    fn tops(&self,point:Point)->Vec<Point>{
        let mut vec:Vec<Point> =vec![];
        let mut next = self.top(point);
        while let Some(point) = next{
            vec.push(point.clone());
            next = self.top(point);
        }
        
        vec
    }
    fn top_rights(&self,point:Point)->Vec<Point>{
        let mut vec:Vec<Point> =vec![];
        let mut next = self.top_right(point);
        while let Some(point) = next{
            vec.push(point.clone());
            next = self.top_right(point);
        }
        vec
    }
    fn lefts(&self,point:Point)->Vec<Point>{
        let mut vec:Vec<Point> =vec![];
        let mut next = self.left(point);
        while let Some(point) = next{
            vec.push(point.clone());
            next = self.left(point);
        }

        vec
    }
    fn rights(&self,point:Point)->Vec<Point>{
        let mut vec:Vec<Point> =vec![];
        let mut next = self.right(point);
        while let Some(point) = next{
            vec.push(point.clone());
            next=self.right(point);
        }
        vec
    }
    fn bottom_lefts(&self,point:Point)->Vec<Point>{
        let mut vec:Vec<Point>=vec![];
        let mut next = self.bottom_left(point);
        while let Some(point) = next{
            vec.push(point.clone());
            next = self.bottom_left(point);
        }
        vec
    }
    fn bottoms(&self,point:Point)->Vec<Point>{
        let mut vec:Vec<Point> =vec![];
        let mut next = self.bottom(point);
        while let Some(point) = next{
            vec.push(point.clone());
            next = self.bottom(point);
        }
        
        vec
    }
    fn bottom_rights(&self,point:Point)->Vec<Point>{
        let mut vec:Vec<Point> =vec![];
        let mut next = self.bottom_right(point);
        while let Some(point) = next{
            vec.push(point.clone());
            next = self.bottom_right(point);
        }
        vec
    }

    fn top_right(&self,point:Point)->Option<Point>{
        let right = if self.get_props().color==Color::White{
            (point.file as u8 +1) as char
        }else{
            (point.file as u8 -1) as char
        };
        let top =if self.get_props().color==Color::White{
            point.rank + 1
        }else{
            point.rank - 1
        };
        if right>='a' && right<='h' && top>=1 && top<=8{
            return Some(Point::new(right, top));
        }
        None
    }
    fn top_left(&self,point:Point)->Option<Point>{
        let left = if self.get_props().color==Color::White{
            (point.file as u8 -1) as char
        }else{
            (point.file as u8 +1) as char
        };
        let top =if self.get_props().color==Color::White{
            point.rank + 1
        }else{
            point.rank - 1
        };
        if left>='a' && left<='h' && top>=1 && top<=8{
            return Some(Point::new(left, top));
        }
        None
    }
    fn bottom_right(&self,point:Point)->Option<Point>{
        let right = if self.get_props().color==Color::White{
            (point.file as u8 +1) as char
        }else{
            (point.file as u8 -1) as char
        };
        let bottom = if self.get_props().color==Color::White{
            point.rank -1
        } else{
            point.rank +1
        };
        if right>='a' && right<='h' && bottom>=1 && bottom<=8{
            return Some(Point::new(right, bottom));
        }
        None
    }
    fn bottom_left(&self,point:Point)->Option<Point>{
        let left = if self.get_props().color==Color::White{
            (point.file as u8 -1) as char
        }else{
            (point.file as u8 +1) as char
        };
        let bottom = if self.get_props().color==Color::White{
            point.rank -1
        } else{
            point.rank +1
        };
        if left>='a' && left<='h' && bottom>=1 && bottom<=8{
            return Some(Point::new(left, bottom));
        }
        None
    }
    fn left(&self,point:Point)->Option<Point>{
        let left = if self.get_props().color==Color::White{
            (point.file as u8 -1) as char
        }else{
            (point.file as u8 +1) as char
        };
        if left>='a' && left<='h'{
            return Some(Point::new(left, point.rank));
        }
        None
    }
    fn right(&self,point:Point)->Option<Point>{
        let right = if self.get_props().color==Color::White{
            (point.file as u8 +1) as char
        }else{
            (point.file as u8 -1) as char
        };
        if right>='a' && right<='h'{
            return Some(Point::new(right, point.rank ));
        }
        None
    }
    fn top(&self,point:Point)->Option<Point>{
        let top =if self.get_props().color==Color::White{
            point.rank + 1
        }else{
            point.rank - 1
        };
        if top>=1 && top<=8 {
            return Some(Point::new(point.file,top));
        }
        None
    }
    fn bottom(&self,point:Point)->Option<Point>{
        let bottom = if self.get_props().color==Color::White{
            point.rank -1
        } else{
            point.rank +1
        };
        if bottom>=1 && bottom<=8{
            return Some(Point::new(point.file,bottom));
        }
        None
    }
    
    fn top_top_left(&self,point:Point)->Option<Point>{
        let next = self.top(point);
        if let Some(point)=next{
            return self.top_left(point);
        }
        None
    }
    fn top_top_right(&self,point:Point)->Option<Point>{
        let next = self.top(point);
        if let Some(point)=next{
            return self.top_right(point);
        }
        None
    }
    fn right_top_right(&self,point:Point)->Option<Point>{
        let next = self.right(point);
        if let Some(point)=next{
            return self.top_right(point);
        }
        None
    }
    fn right_bottom_right(&self,point:Point)->Option<Point>{
        let next = self.right(point);
        if let Some(point)=next{
            return self.bottom_right(point);
        }
        None
    }
    fn bottom_bottom_right(&self,point:Point)->Option<Point>{
        let next = self.bottom(point);
        if let Some(point)=next{
            return self.bottom_right(point);
        }
        None
    }
    fn bottom_bottom_left(&self,point:Point)->Option<Point>{
        let next = self.bottom(point);
        if let Some(point)=next{
            return self.bottom_left(point);
        }
        None
    }
    fn left_bottom_left(&self,point:Point)->Option<Point>{
        let next = self.left(point);
        if let Some(point)=next{
            return self.bottom_left(point);
        }
        None
    }
    fn left_top_left(&self,point:Point)->Option<Point>{
        let next = self.left(point);
        if let Some(point)=next{
            return self.top_left(point);
        }
        None
    }
    
}

pub struct King{
    id:u128,
    board:Rc<Board>,
    moved:RefCell<bool>,
    color:Color,
    name:Type,
}
impl King{
    pub fn new(color:Color, board:Rc<Board>)->King{
        King{
            id:SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_nanos(),
            moved:RefCell::new(false),
            color,
            name:Type::King,
            board:board,
        }
    }
}

impl King{
    fn side_x2(&self)->Vec<Point>{
        if self.get_props().color==Color::White{
            vec![Point::new('c',1),Point::new('g',1)]
        }else{
            vec![Point::new('c',8),Point::new('g',8)]
        }
    }
}
impl Piece for King{

    fn get_props(&self)->Props{
        Props { 
            id:self.id,
            color:self.color, 
            name: self.name, 
        }
    }

    fn moves(&self)->Vec<Point> {
        let s = self.board.squares.iter()
        .flat_map(|x|x.iter())
        .find(|x|x.piece.borrow().is_some() && x.piece.borrow().as_ref().unwrap().get_props().id==self.id)
        .unwrap();

        let mut vec = self.surrounding_points(s.point);
        if !*self.moved.borrow(){
            vec.extend(self.side_x2());
        }
        vec
    }

    fn points_between(&self,point:Point,point2:Point) -> Vec<Point>{
        vec![]
    }

    fn is_moved(&self)->bool {
        *self.moved.borrow()
    }

    fn moved(&self) {
        if !*self.moved.borrow(){
            *self.moved.borrow_mut()=true;
        }
    }
    
}

pub struct Queen{
    id:u128,
    board:Rc<Board>,
    moved:RefCell<bool>,
    color:Color,
    name:Type,
}
impl Queen{
    pub fn new(color:Color,board:Rc<Board>)->Queen{
        Queen{
            id:SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_nanos(),
            moved:RefCell::new(false),
            color,
            name:Type::Queen,
            board
        }
    }
}
impl Piece for Queen{

    fn get_props(&self)->Props{
        Props { 
            id:self.id,
            color:self.color, 
            name: self.name, 
        }
    }

    // fn available_move(&self)->Vec<Option<Point>> {
    //     todo!()
    // }


    fn moves(&self)->Vec<Point> {
        let s = self.board.squares.iter()
        .flat_map(|x|x.iter())
        .find(|x|x.piece.borrow().is_some() && x.piece.borrow().as_ref().unwrap().get_props().id==self.id)
        .unwrap();
    
        let mut vec = self.top_lefts(s.point);
        vec.extend(self.tops(s.point));
        vec.extend(self.top_rights(s.point));
        vec.extend(self.lefts(s.point));
        vec.extend(self.rights(s.point));
        vec.extend(self.bottom_lefts(s.point));
        vec.extend(self.bottoms(s.point));
        vec.extend(self.bottom_rights(s.point));
        vec
    }

    fn points_between(&self,point:Point,point2:Point)->Vec<Point> {
        let points = if self.tops(point).contains(&point2){
            self.tops(point)
        }else if self.top_lefts(point).contains(&point2){
            self.top_lefts(point)
        }else if self.top_rights(point).contains(&point2){
            self.top_rights(point)
        }else if self.rights(point).contains(&point2){
            self.rights(point)
        }else if self.bottom_rights(point).contains(&point2){
            self.bottom_rights(point)
        }else if self.bottoms(point).contains(&point2){
            self.bottoms(point)
        }else if self.bottom_lefts(point).contains(&point2){
            self.bottom_lefts(point)
        }else if self.lefts(point).contains(&point2){
            self.lefts(point)
        } else{
            vec![]
        };
        points.into_iter().take_while(|x| x!=&point2).collect()
    }

    fn is_moved(&self)->bool {
        *self.moved.borrow()
    }
    fn moved(&self) {
        if !*self.moved.borrow(){
            *self.moved.borrow_mut()=true;
        }
    }
    
}
pub struct Rook{
    id:u128,
    board:Rc<Board>,
    moved:RefCell<bool>,
    color:Color,
    name:Type,
}
impl Rook{
    pub fn new(color:Color, board:Rc<Board>)->Rook{
        Rook{
            id:SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_nanos(),
            moved:RefCell::new(false),
            color,
            name:Type::Rook,
            board
        }
    }
}
impl Piece for Rook{
    fn get_props(&self)->Props{
        Props { 
            id:self.id,
            color:self.color, 
            name: self.name, 
        }
    }

    fn moves(&self)->Vec<Point> {
        let s = self.board.squares.iter()
        .flat_map(|x|x.iter())
        .find(|x|x.piece.borrow().is_some() && x.piece.borrow().as_ref().unwrap().get_props().id==self.id)
        .unwrap();

        let mut vec = self.tops(s.point);
        vec.extend(self.lefts(s.point));
        vec.extend(self.rights(s.point));
        vec.extend(self.bottoms(s.point));
        vec
    }

    fn points_between(&self,point:Point,point2:Point)->Vec<Point> {
        let points = if self.tops(point).contains(&point2){
            self.tops(point)
        }else if self.rights(point).contains(&point2){
            self.rights(point)
        }else if self.bottoms(point).contains(&point2){
            self.bottoms(point)
        }else if self.lefts(point).contains(&point2){
            self.lefts(point)
        } else{
            vec![]
        };
        points.into_iter().take_while(|x| x!=&point2).collect()
    }

    fn is_moved(&self)->bool {
        *self.moved.borrow()
    }
    fn moved(&self) {
        if !*self.moved.borrow(){
            *self.moved.borrow_mut()=true;
        }
    }

}


pub struct Bishop{
    id:u128,
    board:Rc<Board>,
    moved:RefCell<bool>,
    color:Color,
    name:Type,
}
impl Bishop{
    pub fn new(color:Color,board:Rc<Board>)->Bishop{
        Bishop{
            id:SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_nanos(),
            moved:RefCell::new(false),
            color,
            name:Type::Bishop,
            board
        }
    }
}
impl Piece for Bishop{

    fn get_props(&self)->Props{
        Props { 
            id:self.id,
            color:self.color, 
            name: self.name, 
        }
    }

    fn moves(&self)->Vec<Point> {
        let s = self.board.squares.iter()
        .flat_map(|x|x.iter())
        .find(|x|x.piece.borrow().is_some() && x.piece.borrow().as_ref().unwrap().get_props().id==self.id)
        .unwrap();

        let mut  vec = self.top_lefts(s.point);
        vec.extend(self.top_rights(s.point));
        vec.extend(self.bottom_lefts(s.point));
        vec.extend(self.bottom_rights(s.point));
        vec
    }

    fn points_between(&self,point:Point,point2:Point)->Vec<Point> {
        let points = if self.top_lefts(point).contains(&point2){
            self.top_lefts(point)
        }else if self.top_rights(point).contains(&point2){
            self.top_rights(point)
        }else if self.bottom_rights(point).contains(&point2){
            self.bottom_rights(point)
        }else if self.bottom_lefts(point).contains(&point2){
            self.bottom_lefts(point)
        }else{
            vec![]
        };
        points.into_iter().take_while(|x| x!=&point2).collect()
    }

    fn is_moved(&self)->bool {
        *self.moved.borrow()
    }
    fn moved(&self) {
        if !*self.moved.borrow(){
            *self.moved.borrow_mut()=true;
        }
    }
}
pub struct Knight{
    id:u128,
    board:Rc<Board>,
    moved:RefCell<bool>,
    color:Color,
    name:Type,
}
impl Knight{
    pub fn new(color:Color,board:Rc<Board>,)->Knight{
        Knight{
            id:SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_nanos(),
            moved:RefCell::new(false),
            color,
            name:Type::Knight,
            board
        }
    }
}
impl Piece for Knight{
    fn get_props(&self)->Props{
        Props { 
            id:self.id,
            color:self.color, 
            name: self.name, 
        }
    }

    fn moves(&self)->Vec<Point> {
        let s = self.board.squares.iter()
        .flat_map(|x|x.iter())
        .find(|x|x.piece.borrow().is_some() && x.piece.borrow().as_ref().unwrap().get_props().id==self.id)
        .unwrap();
        vec![
            self.top_top_left(s.point),
            self.top_top_right(s.point),
            self.right_top_right(s.point),
            self.right_bottom_right(s.point),
            self.bottom_bottom_right(s.point),
            self.bottom_bottom_left(s.point),
            self.left_bottom_left(s.point),
            self.left_top_left(s.point),
        ].iter()
        .filter_map(|x|x.clone())
        .collect()
    }

    fn points_between(&self,point:Point,point2:Point)->Vec<Point> {
        vec![]
    }

    fn is_moved(&self)->bool {
        *self.moved.borrow()
    }
    fn moved(&self) {
        if !*self.moved.borrow(){
            *self.moved.borrow_mut()=true;
        }
    }

}
pub struct Pawn{
    id:u128,
    board:Rc<Board>,
    moved:RefCell<bool>,
    color:Color,
    name:Type,
}
impl Pawn{
    pub fn new(color:Color,board:Rc<Board>,)->Pawn{
        Pawn{
            id:SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_nanos(),
            moved:RefCell::new(false),
            color,
            name:Type::Pawn,
            board
        }
    }
}
impl Piece for Pawn{

    fn get_props(&self)->Props{
        Props {
            id:self.id, 
            color:self.color, 
            name: self.name, 
        }
    }

    fn moves(&self)->Vec<Point> {

        let s = self.board.squares.iter()
        .flat_map(|x|x.iter())
        .find(|x|x.piece.borrow().is_some() && x.piece.borrow().as_ref().unwrap().get_props().id==self.id)
        .unwrap();

        let mut vec :Vec<Point> = vec![
            self.top_left(s.point),
            self.top_right(s.point),
            self.top(s.point)
        ].iter()
        .filter_map(|x|x.clone())
        .collect();

        if !*self.moved.borrow(){
            vec.push(self.top_x2(s.point));
        }
        vec 
    }

    fn points_between(&self,point:Point,point2:Point)->Vec<Point> {
        vec![]
    }

    fn is_moved(&self)->bool {
        *self.moved.borrow()
    }
    fn moved(&self) {
        if !*self.moved.borrow(){
            *self.moved.borrow_mut()=true;
        }
    }

}
impl Pawn{
    fn top_x2(&self, point:Point)->Point{
        let top =if self.get_props().color==Color::White{
            point.rank + 2
        }else{
            point.rank - 2
        };

        if top>=1 && top<=8 {
        }
        return Point::new(point.file,top);
        
    }
}