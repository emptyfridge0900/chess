use std::{cell::RefCell, rc::Rc, time::SystemTime, collections::HashSet};

use crate::{board::Board, Color, Point, Props, Square, Type};

pub trait Piece {
    fn get_props(&self) -> Props;
    fn moves(&self) -> Vec<Point>;
    fn particular_moves(&self) -> Vec<Point>;
    fn points_between(&self, point: Point) -> Vec<Point>;
    fn is_moved(&self) -> bool;
    fn moved(&self);
    fn square(&self) -> &Square;
    fn board(&self) -> &Board;

    fn surrounding_points(&self) -> Vec<Point> {
        vec![
            self.top_left(),
            self.top(),
            self.top_right(),
            self.left(),
            self.right(),
            self.bottom_left(),
            self.bottom(),
            self.bottom_right(),
        ]
        .iter()
        .filter_map(|x| x.clone())
        .collect()
    }

    fn top_lefts(&self) -> Vec<Point> {
        let mut vec: Vec<Point> = vec![];
        let mut next = self.top_left();
        while let Some(point) = next {
            let s = self.board().get_square(&point.notation()).unwrap();
            if s.piece.borrow().is_none() {
                vec.push(point.clone());
                next = point.top_left(self.get_props().color)
            } else {
                if s.piece.borrow().as_ref().unwrap().get_props().color != self.get_props().color {
                    vec.push(point.clone());
                }
                next = None
            }
        }
        vec
    }
    fn tops(&self) -> Vec<Point> {
        let mut vec: Vec<Point> = vec![];
        let mut next = self.top();
        while let Some(point) = next {
            let s = self.board().get_square(&point.notation()).unwrap();
            if s.piece.borrow().is_none() {
                vec.push(point.clone());
                next = point.top(self.get_props().color)
            } else {
                if s.piece.borrow().as_ref().unwrap().get_props().color != self.get_props().color {
                    vec.push(point.clone());
                }
                next = None
            }
        }
        vec
    }
    fn top_rights(&self) -> Vec<Point> {
        let mut vec: Vec<Point> = vec![];
        let mut next = self.top_right();
        while let Some(point) = next {
            let s = self.board().get_square(&point.notation()).unwrap();
            if s.piece.borrow().is_none() {
                vec.push(point.clone());
                next = point.top_right(self.get_props().color)
            } else {
                if s.piece.borrow().as_ref().unwrap().get_props().color != self.get_props().color {
                    vec.push(point.clone());
                }
                next = None
            }
        }
        vec
    }
    fn bottom_lefts(&self) -> Vec<Point> {
        let mut vec: Vec<Point> = vec![];
        let mut next = self.bottom_left();
        while let Some(point) = next {
            let s = self.board().get_square(&point.notation()).unwrap();
            if s.piece.borrow().is_none() {
                vec.push(point.clone());
                next = point.bottom_left(self.get_props().color)
            } else {
                vec.push(point.clone());
                next = None
            }
        }
        vec
    }
    fn bottoms(&self) -> Vec<Point> {
        let mut vec: Vec<Point> = vec![];
        let mut next = self.bottom();
        while let Some(point) = next {
            let s = self.board().get_square(&point.notation()).unwrap();
            if s.piece.borrow().is_none() {
                vec.push(point.clone());
                next = point.bottom(self.get_props().color)
            } else {
                if s.piece.borrow().as_ref().unwrap().get_props().color != self.get_props().color {
                    vec.push(point.clone());
                }
                next = None
            }
        }
        vec
    }
    fn bottom_rights(&self) -> Vec<Point> {
        let mut vec: Vec<Point> = vec![];
        let mut next = self.bottom_right();
        while let Some(point) = next {
            let s = self.board().get_square(&point.notation()).unwrap();
            if s.piece.borrow().is_none() {
                vec.push(point.clone());
                next = point.bottom_right(self.get_props().color)
            } else {
                if s.piece.borrow().as_ref().unwrap().get_props().color != self.get_props().color {
                    vec.push(point.clone());
                }
                next = None
            }
        }
        vec
    }
    fn lefts(&self) -> Vec<Point> {
        let mut vec: Vec<Point> = vec![];
        let mut next = self.left();
        while let Some(point) = next {
            let s = self.board().get_square(&point.notation()).unwrap();
            if s.piece.borrow().is_none() {
                vec.push(point.clone());
                next = point.left(self.get_props().color)
            } else {
                if s.piece.borrow().as_ref().unwrap().get_props().color != self.get_props().color {
                    vec.push(point.clone());
                }
                next = None
            }
        }
        vec
    }
    fn rights(&self) -> Vec<Point> {
        let mut vec: Vec<Point> = vec![];
        let mut next = self.right();
        while let Some(point) = next {
            let s = self.board().get_square(&point.notation()).unwrap();
            if s.piece.borrow().is_none() {
                vec.push(point.clone());
                next = point.right(self.get_props().color)
            } else {
                if s.piece.borrow().as_ref().unwrap().get_props().color != self.get_props().color {
                    vec.push(point.clone());
                }
                next = None
            }
        }
        vec
    }

    fn top_right(&self) -> Option<Point> {
        let point = self.square().point;
        let right = if self.get_props().color == Color::White {
            (point.file as u8 + 1) as char
        } else {
            (point.file as u8 - 1) as char
        };
        let top = if self.get_props().color == Color::White {
            point.rank + 1
        } else {
            point.rank - 1
        };
        if right >= 'a' && right <= 'h' && top >= 1 && top <= 8 {
            return Some(Point::new(right, top));
        }
        None
    }
    fn top_left(&self) -> Option<Point> {
        let point = self.square().point;
        let left = if self.get_props().color == Color::White {
            (point.file as u8 - 1) as char
        } else {
            (point.file as u8 + 1) as char
        };
        let top = if self.get_props().color == Color::White {
            point.rank + 1
        } else {
            point.rank - 1
        };
        if left >= 'a' && left <= 'h' && top >= 1 && top <= 8 {
            return Some(Point::new(left, top));
        }
        None
    }
    fn bottom_right(&self) -> Option<Point> {
        let point = self.square().point;
        let right = if self.get_props().color == Color::White {
            (point.file as u8 + 1) as char
        } else {
            (point.file as u8 - 1) as char
        };
        let bottom = if self.get_props().color == Color::White {
            point.rank - 1
        } else {
            point.rank + 1
        };
        if right >= 'a' && right <= 'h' && bottom >= 1 && bottom <= 8 {
            return Some(Point::new(right, bottom));
        }
        None
    }
    fn bottom_left(&self) -> Option<Point> {
        let point = self.square().point;
        let left = if self.get_props().color == Color::White {
            (point.file as u8 - 1) as char
        } else {
            (point.file as u8 + 1) as char
        };
        let bottom = if self.get_props().color == Color::White {
            point.rank - 1
        } else {
            point.rank + 1
        };
        if left >= 'a' && left <= 'h' && bottom >= 1 && bottom <= 8 {
            return Some(Point::new(left, bottom));
        }
        None
    }
    fn left(&self) -> Option<Point> {
        let point = self.square().point;
        let left = if self.get_props().color == Color::White {
            (point.file as u8 - 1) as char
        } else {
            (point.file as u8 + 1) as char
        };
        if left >= 'a' && left <= 'h' {
            return Some(Point::new(left, point.rank));
        }
        None
    }
    fn right(&self) -> Option<Point> {
        let point = self.square().point;
        let right = if self.get_props().color == Color::White {
            (point.file as u8 + 1) as char
        } else {
            (point.file as u8 - 1) as char
        };
        if right >= 'a' && right <= 'h' {
            return Some(Point::new(right, point.rank));
        }
        None
    }
    fn top(&self) -> Option<Point> {
        let point = self.square().point;
        let top = if self.get_props().color == Color::White {
            point.rank + 1
        } else {
            point.rank - 1
        };
        if top >= 1 && top <= 8 {
            return Some(Point::new(point.file, top));
        }
        None
    }
    fn bottom(&self) -> Option<Point> {
        let point = self.square().point;
        let bottom = if self.get_props().color == Color::White {
            point.rank - 1
        } else {
            point.rank + 1
        };
        if bottom >= 1 && bottom <= 8 {
            return Some(Point::new(point.file, bottom));
        }
        None
    }

    fn top_top_left(&self) -> Option<Point> {
        let next = self.top();
        if let Some(point) = next {
            return point.top_left(self.get_props().color);
        }
        None
    }
    fn top_top_right(&self) -> Option<Point> {
        let next = self.top();
        if let Some(point) = next {
            return point.top_right(self.get_props().color);
        }
        None
    }
    fn right_top_right(&self) -> Option<Point> {
        let next = self.right();
        if let Some(point) = next {
            return point.top_right(self.get_props().color);
        }
        None
    }
    fn right_bottom_right(&self) -> Option<Point> {
        let next = self.right();
        if let Some(point) = next {
            return point.bottom_right(self.get_props().color);
        }
        None
    }
    fn bottom_bottom_right(&self) -> Option<Point> {
        let next = self.bottom();
        if let Some(point) = next {
            return point.bottom_right(self.get_props().color);
        }
        None
    }
    fn bottom_bottom_left(&self) -> Option<Point> {
        let next = self.bottom();
        if let Some(point) = next {
            return point.bottom_left(self.get_props().color);
        }
        None
    }
    fn left_bottom_left(&self) -> Option<Point> {
        let next = self.left();
        if let Some(point) = next {
            return point.bottom_left(self.get_props().color);
        }
        None
    }
    fn left_top_left(&self) -> Option<Point> {
        let next = self.left();
        if let Some(point) = next {
            return point.top_left(self.get_props().color);
        }
        None
    }
}

pub struct King {
    id: u128,
    board: Rc<Board>,
    moved: RefCell<bool>,
    color: Color,
    name: Type,
}
impl King {
    pub fn new(color: Color, board: Rc<Board>) -> King {
        King {
            id: SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_nanos(),
            moved: RefCell::new(false),
            color,
            name: Type::King,
            board: board,
        }
    }
}

impl King {

    fn is_safe(&self,point:&str)->bool{
        let mut is_king_under_check = false;
        let rook = self.board.get_square(point).unwrap();
        if rook.piece.borrow().is_some() && !rook.piece.borrow().as_ref().unwrap().is_moved() {
            let opponent = self.board.get_pieces_by_color(if self.color==Color::White{Color::Black}else{Color::White});
            let rook_moves = rook
                .piece
                .borrow()
                .as_ref()
                .unwrap()
                .moves();
            let king_moves = self.moves();
            let rook_set: HashSet<_> = rook_moves.iter().copied().collect();   
            let reachable = king_moves.iter().any(|item| rook_set.contains(item));
            if !reachable{
                return false;
            }
            let points = if (rook.point.file =='a' && self.color==Color::White)
            || (rook.point.file =='h' && self.color==Color::Black)
            {
                vec![
                    self.square().point,
                    self.square().point.left(self.color).unwrap(),
                    self.square().point.left(self.color).unwrap().left(self.color).unwrap(),
                ]
            }else{
                vec![
                    self.square().point,
                    self.square().point.right(self.color).unwrap(),
                    self.square().point.right(self.color).unwrap().right(self.color).unwrap(),
                ]                
            };


              
            'outer: for s in opponent {
                let moves = s.piece.borrow().as_ref().unwrap().moves();
                for m in points.iter() {
                    if !moves.contains(m) {
                        is_king_under_check = true;
                        break 'outer;
                    }
                }
            }
        }
        is_king_under_check
    }

}
impl Piece for King {
    fn get_props(&self) -> Props {
        Props {
            id: self.id,
            color: self.color,
            name: self.name,
        }
    }

    fn square(&self) -> &Square {
        self.board
            .squares
            .iter()
            .flat_map(|x| x.iter())
            .find(|x| {
                x.piece.borrow().is_some()
                    && x.piece.borrow().as_ref().unwrap().get_props().id == self.id
            })
            .unwrap()
    }

    fn moves(&self) -> Vec<Point> {
        let mut vec: Vec<Point> = vec![
            self.top_left(),
            self.top(),
            self.top_right(),
            self.left(),
            self.right(),
            self.bottom_left(),
            self.bottom(),
            self.bottom_right(),
        ]
        .iter()
        .filter_map(|x| {
            if x.is_some() {
                let s = self.board.get_square(&x.unwrap().notation()).unwrap();
                if s.piece.borrow().is_some()
                    && s.piece.borrow().as_ref().unwrap().get_props().color != self.color
                {
                    x.clone()
                } else if s.piece.borrow().is_none() {
                    x.clone()
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect();
        vec
    }

    fn particular_moves(&self) -> Vec<Point> {
        let mut vec: Vec<Point> = vec![];
        if !*self.moved.borrow() {
            if self.color == Color::White {
                if self.is_safe("a1"){
                    vec.push(self.square().point.left(self.color).unwrap().left(self.color).unwrap());
                }
                if self.is_safe("h1"){
                    vec.push(self.square().point.right(self.color).unwrap().right(self.color).unwrap());                    
                }
            } else {
                if self.is_safe("h8"){
                    vec.push(self.square().point.left(self.color).unwrap().left(self.color).unwrap());
                }
                if self.is_safe("a8"){
                    vec.push(self.square().point.right(self.color).unwrap().right(self.color).unwrap());                    
                }
            }
        }
        vec
    }

    fn points_between(&self, point2: Point) -> Vec<Point> {
        vec![]
    }

    fn is_moved(&self) -> bool {
        *self.moved.borrow()
    }

    fn moved(&self) {
        if !*self.moved.borrow() {
            *self.moved.borrow_mut() = true;
        }
    }

    fn board(&self) -> &Board {
        &self.board
    }

    
}

pub struct Queen {
    id: u128,
    board: Rc<Board>,
    moved: RefCell<bool>,
    color: Color,
    name: Type,
}
impl Queen {
    pub fn new(color: Color, board: Rc<Board>) -> Queen {
        Queen {
            id: SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_nanos(),
            moved: RefCell::new(false),
            color,
            name: Type::Queen,
            board,
        }
    }
}
impl Piece for Queen {
    fn get_props(&self) -> Props {
        Props {
            id: self.id,
            color: self.color,
            name: self.name,
        }
    }

    fn square(&self) -> &Square {
        self.board
            .squares
            .iter()
            .flat_map(|x| x.iter())
            .find(|x| {
                x.piece.borrow().is_some()
                    && x.piece.borrow().as_ref().unwrap().get_props().id == self.id
            })
            .unwrap()
    }

    fn moves(&self) -> Vec<Point> {
        let mut vec = self.top_lefts();
        vec.extend(self.tops());
        vec.extend(self.top_rights());
        vec.extend(self.lefts());
        vec.extend(self.rights());
        vec.extend(self.bottom_lefts());
        vec.extend(self.bottoms());
        vec.extend(self.bottom_rights());
        vec
    }

    fn points_between(&self, point2: Point) -> Vec<Point> {
        let points = if self.tops().contains(&point2) {
            self.tops()
        } else if self.top_lefts().contains(&point2) {
            self.top_lefts()
        } else if self.top_rights().contains(&point2) {
            self.top_rights()
        } else if self.rights().contains(&point2) {
            self.rights()
        } else if self.bottom_rights().contains(&point2) {
            self.bottom_rights()
        } else if self.bottoms().contains(&point2) {
            self.bottoms()
        } else if self.bottom_lefts().contains(&point2) {
            self.bottom_lefts()
        } else if self.lefts().contains(&point2) {
            self.lefts()
        } else {
            vec![]
        };
        points.into_iter().take_while(|x| x != &point2).collect()
    }
    fn particular_moves(&self) -> Vec<Point> {
        vec![]
    }

    fn is_moved(&self) -> bool {
        *self.moved.borrow()
    }
    fn moved(&self) {
        if !*self.moved.borrow() {
            *self.moved.borrow_mut() = true;
        }
    }
    fn board(&self) -> &Board {
        &self.board
    }

    
}
pub struct Rook {
    id: u128,
    board: Rc<Board>,
    moved: RefCell<bool>,
    color: Color,
    name: Type,
}
impl Rook {
    pub fn new(color: Color, board: Rc<Board>) -> Rook {
        Rook {
            id: SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_nanos(),
            moved: RefCell::new(false),
            color,
            name: Type::Rook,
            board,
        }
    }
}
impl Piece for Rook {
    fn get_props(&self) -> Props {
        Props {
            id: self.id,
            color: self.color,
            name: self.name,
        }
    }

    fn square(&self) -> &Square {
        self.board
            .squares
            .iter()
            .flat_map(|x| x.iter())
            .find(|x| {
                x.piece.borrow().is_some()
                    && x.piece.borrow().as_ref().unwrap().get_props().id == self.id
            })
            .unwrap()
    }

    fn moves(&self) -> Vec<Point> {
        let mut vec = self.tops();
        vec.extend(self.lefts());
        vec.extend(self.rights());
        vec.extend(self.bottoms());
        vec
    }

    fn particular_moves(&self) -> Vec<Point> {
        vec![]
    }

    fn points_between(&self, point2: Point) -> Vec<Point> {
        let points = if self.tops().contains(&point2) {
            self.tops()
        } else if self.rights().contains(&point2) {
            self.rights()
        } else if self.bottoms().contains(&point2) {
            self.bottoms()
        } else if self.lefts().contains(&point2) {
            self.lefts()
        } else {
            vec![]
        };
        points.into_iter().take_while(|x| x != &point2).collect()
    }

    fn is_moved(&self) -> bool {
        *self.moved.borrow()
    }
    fn moved(&self) {
        if !*self.moved.borrow() {
            *self.moved.borrow_mut() = true;
        }
    }
    fn board(&self) -> &Board {
        &self.board
    }
}

pub struct Bishop {
    id: u128,
    board: Rc<Board>,
    moved: RefCell<bool>,
    color: Color,
    name: Type,
}
impl Bishop {
    pub fn new(color: Color, board: Rc<Board>) -> Bishop {
        Bishop {
            id: SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_nanos(),
            moved: RefCell::new(false),
            color,
            name: Type::Bishop,
            board,
        }
    }
}
impl Piece for Bishop {
    fn get_props(&self) -> Props {
        Props {
            id: self.id,
            color: self.color,
            name: self.name,
        }
    }

    fn square(&self) -> &Square {
        self.board
            .squares
            .iter()
            .flat_map(|x| x.iter())
            .find(|x| {
                x.piece.borrow().is_some()
                    && x.piece.borrow().as_ref().unwrap().get_props().id == self.id
            })
            .unwrap()
    }

    fn moves(&self) -> Vec<Point> {
        let mut vec = self.top_lefts();
        vec.extend(self.top_rights());
        vec.extend(self.bottom_lefts());
        vec.extend(self.bottom_rights());
        vec
    }

    fn particular_moves(&self) -> Vec<Point> {
        vec![]
    }

    fn points_between(&self, point2: Point) -> Vec<Point> {
        let point = self.square().point;
        let points = if self.top_lefts().contains(&point2) {
            self.top_lefts()
        } else if self.top_rights().contains(&point2) {
            self.top_rights()
        } else if self.bottom_rights().contains(&point2) {
            self.bottom_rights()
        } else if self.bottom_lefts().contains(&point2) {
            self.bottom_lefts()
        } else {
            vec![]
        };
        points.into_iter().take_while(|x| x != &point2).collect()
    }

    fn is_moved(&self) -> bool {
        *self.moved.borrow()
    }
    fn moved(&self) {
        if !*self.moved.borrow() {
            *self.moved.borrow_mut() = true;
        }
    }
    fn board(&self) -> &Board {
        &self.board
    }
}
pub struct Knight {
    id: u128,
    board: Rc<Board>,
    moved: RefCell<bool>,
    color: Color,
    name: Type,
}
impl Knight {
    pub fn new(color: Color, board: Rc<Board>) -> Knight {
        Knight {
            id: SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_nanos(),
            moved: RefCell::new(false),
            color,
            name: Type::Knight,
            board,
        }
    }
}
impl Piece for Knight {
    fn get_props(&self) -> Props {
        Props {
            id: self.id,
            color: self.color,
            name: self.name,
        }
    }

    fn square(&self) -> &Square {
        self.board
            .squares
            .iter()
            .flat_map(|x| x.iter())
            .find(|x| {
                x.piece.borrow().is_some()
                    && x.piece.borrow().as_ref().unwrap().get_props().id == self.id
            })
            .unwrap()
    }

    fn moves(&self) -> Vec<Point> {
        //self.square().piece.borrow().as_ref().unwrap().get_props().color != self.get_props().color
        vec![
            self.top_top_left(),
            self.top_top_right(),
            self.right_top_right(),
            self.right_bottom_right(),
            self.bottom_bottom_right(),
            self.bottom_bottom_left(),
            self.left_bottom_left(),
            self.left_top_left(),
        ]
        .iter()
        .filter_map(|x| {
            if x.is_some() {
                let s = self.board.get_square(&x.unwrap().notation()).unwrap();
                if s.piece.borrow().is_some()
                    && s.piece.borrow().as_ref().unwrap().get_props().color != self.color
                {
                    x.clone()
                } else if s.piece.borrow().is_none() {
                    x.clone()
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect()
    }

    fn particular_moves(&self) -> Vec<Point> {
        vec![]
    }

    fn points_between(&self, point2: Point) -> Vec<Point> {
        vec![]
    }

    fn is_moved(&self) -> bool {
        *self.moved.borrow()
    }
    fn moved(&self) {
        if !*self.moved.borrow() {
            *self.moved.borrow_mut() = true;
        }
    }
    fn board(&self) -> &Board {
        &self.board
    }
}
pub struct Pawn {
    id: u128,
    board: Rc<Board>,
    moved: RefCell<bool>,
    color: Color,
    name: Type,
}
impl Pawn {
    pub fn new(color: Color, board: Rc<Board>) -> Pawn {
        Pawn {
            id: SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_nanos(),
            moved: RefCell::new(false),
            color,
            name: Type::Pawn,
            board,
        }
    }
}
impl Piece for Pawn {
    fn get_props(&self) -> Props {
        Props {
            id: self.id,
            color: self.color,
            name: self.name,
        }
    }

    fn square(&self) -> &Square {
        self.board
            .squares
            .iter()
            .flat_map(|x| x.iter())
            .find(|x| {
                x.piece.borrow().is_some()
                    && x.piece.borrow().as_ref().unwrap().get_props().id == self.id
            })
            .unwrap()
    }

    fn moves(&self) -> Vec<Point> {
        let mut vec: Vec<Point> = vec![];

        for p in vec![self.top_left(), self.top_right()].iter() {
            if let Some(point) = p {
                let s = self.board.get_square(&point.notation()).unwrap();
                if s.piece.borrow().is_some()
                    && s.piece.borrow().as_ref().unwrap().get_props().color
                        != self.get_props().color
                {
                    vec.push(point.clone());
                }
            }
        }
        if let Some(p) = self.top() {
            let s = self.board.get_square(&p.notation()).unwrap();
            if s.piece.borrow().is_none() {
                vec.push(p);
            }
        }

        vec
    }

    fn particular_moves(&self) -> Vec<Point> {
        let mut vec: Vec<Point> = vec![];

        if !self.is_moved() {
            let s = self.board.get_square(&self.top_x2().notation()).unwrap();
            if s.piece.borrow().is_none() {
                vec.push(self.top_x2());
            }
        }

        if self.square().point.rank == 5 && self.color == Color::White
        {
            let left = self.square().point.left(self.color);
            if let Some(side) = left{
                if let Some(last_record) = self.board.record.borrow().last(){
                    if last_record.src.rank==7 
                    && last_record.dst==side
                    && last_record.name== Type::Pawn
                    && last_record.color==Color::Black 
                    {
                        vec.push(self.square().point.top_left(self.color).unwrap());
                    }
                }
            }

            let right = self.square().point.right(self.color);
            if let Some(side) = right{
                if let Some(last_record) = self.board.record.borrow().last(){
                    if last_record.src.rank==7 
                    && last_record.dst==side
                    && last_record.name== Type::Pawn
                    && last_record.color==Color::Black 
                    {
                        vec.push(self.square().point.top_right(self.color).unwrap());
                    }
                }
            }
        }

        if self.square().point.rank == 4 && self.color == Color::Black
        {
            let left = self.square().point.left(self.color);
            if let Some(side) = left{
                if let Some(last_record) = self.board.record.borrow().last(){
                    if last_record.src.rank==2 
                    && last_record.dst==side
                    && last_record.name== Type::Pawn
                    && last_record.color==Color::White 
                    {
                        vec.push(self.square().point.top_left(self.color).unwrap());
                    }
                }
            }

            let right = self.square().point.right(self.color);
            if let Some(side) = right{
                if let Some(last_record) = self.board.record.borrow().last(){
                    if last_record.src.rank==2
                    && last_record.dst==side
                    && last_record.name== Type::Pawn
                    && last_record.color==Color::White 
                    {
                        vec.push(self.square().point.top_right(self.color).unwrap());
                    }
                }
            }            
        }

        vec
    }

    fn points_between(&self, point2: Point) -> Vec<Point> {
        vec![]
    }

    fn is_moved(&self) -> bool {
        *self.moved.borrow()
    }
    fn moved(&self) {
        if !*self.moved.borrow() {
            *self.moved.borrow_mut() = true;
        }
    }
    fn board(&self) -> &Board {
        &self.board
    }
}
impl Pawn {
    fn top_x2(&self) -> Point {
        let point = self.square().point;
        let top = if self.get_props().color == Color::White {
            point.rank + 2
        } else {
            point.rank - 2
        };
        if top >= 1 && top <= 8 {}

        return Point::new(point.file, top);
    }
}
