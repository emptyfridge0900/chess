
use crate::{Type, Point, Props, Color};


pub trait Piece{
    fn get_props(&self)->Props;
    fn set_point(&mut self, point:Point);
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

pub struct King{
    pub color:Color,
    pub name:Type,
    pub point:Point
}
impl Piece for King{

    fn get_props(&self)->Props{
        Props { 
            color:self.color, 
            name: self.name, 
            point: self.point 
        }
    }

    fn set_point(&mut self, point:Point) {
        self.point=point
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

pub struct Queen{
    pub color:Color,
    pub name:Type,
    pub point:Point
}
impl Piece for Queen{

    fn set_point(&mut self, point:Point) {
        self.point=point
    }

    fn get_props(&self)->Props{
        Props { 
            color:self.color, 
            name: self.name, 
            point: self.point 
        }
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
pub struct Rook{
    pub color:Color,
    pub name:Type,
    pub point:Point
}
impl Piece for Rook{

    fn set_point(&mut self, point:Point) {
        self.point=point
    }

    fn get_props(&self)->Props{
        Props { 
            color:self.color, 
            name: self.name, 
            point: self.point 
        }
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
pub struct Bishop{
    pub color:Color,
    pub name:Type,
    pub point:Point
}
impl Piece for Bishop{

    fn set_point(&mut self, point:Point) {
        self.point=point
    }

    fn get_props(&self)->Props{
        Props { 
            color:self.color, 
            name: self.name, 
            point: self.point 
        }
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
pub struct Knight{
    pub color:Color,
    pub name:Type,
    pub point:Point
}
impl Piece for Knight{

    fn set_point(&mut self, point:Point) {
        self.point=point
    }

    fn get_props(&self)->Props{
        Props { 
            color:self.color, 
            name: self.name, 
            point: self.point 
        }
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
pub struct Pawn{
    pub color:Color,
    pub name:Type,
    pub point:Point
}
impl Piece for Pawn{

    fn set_point(&mut self, point:Point) {
        self.point=point
    }

    fn get_props(&self)->Props{
        Props { 
            color:self.color, 
            name: self.name, 
            point: self.point 
        }
    }

    fn moves(&self)->Vec<Point> {
        vec![]
    }

}