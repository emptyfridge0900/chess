
use crate::{Type, Point, Props, Color};


pub trait Piece{
    fn get_props(&self)->Props;
    // fn available_move(&self)->Vec<Option<Point>>;
    // fn can_move(&self,rank:char,file:u32)->bool;
    // fn set_squre(&self,rank:char,file:u32);
    // fn get_squre(&self)->String;
    // fn attack(&self,piece: Box<dyn Piece>);
    // fn can_attack(&self,piece: Box<dyn Piece>) -> bool;
    
    fn moves(&self, point:Point)->Vec<Point>;

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
    color:Color,
    name:Type,
}
impl King{
    pub fn new(color:Color,name:Type)->King{
        King{
            color,
            name,
        }
    }
}
impl Piece for King{

    fn get_props(&self)->Props{
        Props { 
            color:self.color, 
            name: self.name, 
        }
    }


    // fn available_move(&self)->Vec<Option<Point>> {
    //     self.surrounding_points(self.point.file, self.point.rank)
    // }

    fn moves(&self, point:Point)->Vec<Point> {
        self.surrounding_points(point)
    }
    
}

pub struct Queen{
    color:Color,
    name:Type,
}
impl Queen{
    pub fn new(color:Color,name:Type)->Queen{
        Queen{
            color,
            name,
        }
    }
}
impl Piece for Queen{

    fn get_props(&self)->Props{
        Props { 
            color:self.color, 
            name: self.name, 
        }
    }

    // fn available_move(&self)->Vec<Option<Point>> {
    //     todo!()
    // }


    fn moves(&self,point:Point)->Vec<Point> {
        
        let mut vec = self.top_lefts(point);
        vec.extend(self.tops(point));
        vec.extend(self.top_rights(point));
        vec.extend(self.lefts(point));
        vec.extend(self.rights(point));
        vec.extend(self.bottom_lefts(point));
        vec.extend(self.bottoms(point));
        vec.extend(self.bottom_rights(point));
        vec
}
}
pub struct Rook{
    color:Color,
    name:Type,
}
impl Rook{
    pub fn new(color:Color,name:Type)->Rook{
        Rook{
            color,
            name,
        }
    }
}
impl Piece for Rook{
    fn get_props(&self)->Props{
        Props { 
            color:self.color, 
            name: self.name, 
        }
    }

    fn moves(&self,point:Point)->Vec<Point> {
        let mut vec = self.tops(point);
        vec.extend(self.lefts(point));
        vec.extend(self.rights(point));
        vec.extend(self.bottoms(point));
        vec
    }

}


pub struct Bishop{
    color:Color,
    name:Type,
}
impl Bishop{
    pub fn new(color:Color,name:Type)->Bishop{
        Bishop{
            color,
            name,
        }
    }
}
impl Piece for Bishop{

    fn get_props(&self)->Props{
        Props { 
            color:self.color, 
            name: self.name, 
        }
    }

    fn moves(&self,point:Point)->Vec<Point> {
        let mut  vec = self.top_lefts(point);
        vec.extend(self.top_rights(point));
        vec.extend(self.bottom_lefts(point));
        vec.extend(self.bottom_rights(point));
        vec
    }
}
pub struct Knight{
    color:Color,
    name:Type,
}
impl Knight{
    pub fn new(color:Color,name:Type)->Knight{
        Knight{
            color,
            name,
        }
    }
}
impl Piece for Knight{


    fn get_props(&self)->Props{
        Props { 
            color:self.color, 
            name: self.name, 
        }
    }

    fn moves(&self,point:Point)->Vec<Point> {
        vec![
            self.top_top_left(point),
            self.top_top_right(point),
            self.right_top_right(point),
            self.right_bottom_right(point),
            self.bottom_bottom_right(point),
            self.bottom_bottom_left(point),
            self.left_bottom_left(point),
            self.left_top_left(point),
        ].iter()
        .filter_map(|x|x.clone())
        .collect()
    }
}
pub struct Pawn{
    color:Color,
    name:Type,
}
impl Pawn{
    pub fn new(color:Color,name:Type)->Pawn{
        Pawn{
            color,
            name,
        }
    }
}
impl Piece for Pawn{



    fn get_props(&self)->Props{
        Props { 
            color:self.color, 
            name: self.name, 
        }
    }

    fn moves(&self,point:Point)->Vec<Point> {
        let mut  vec = self.top_lefts(point);
        vec.extend(self.tops(point));
        vec.extend(self.top_rights(point));
        vec
    }

}