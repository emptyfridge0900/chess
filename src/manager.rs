use std::{cell::RefCell, error, rc::Rc};

use crate::{board::{Board, Notation, Attack}, error::InvalidateInutError, piece::{Bishop, King, Knight, Pawn, Piece, Queen, Rook}, Color, Point, Square, Type};


pub struct ChessManager {
    board: Rc<Board>,
    pub white_record: RefCell<Vec<String>>,
    pub black_record: RefCell<Vec<String>>,
}
impl ChessManager {
    pub fn new() -> ChessManager {
        let board = Rc::new(Board::new());
        ChessManager {
            board: Rc::clone(&board),
            white_record: RefCell::new(vec![]),
            black_record: RefCell::new(vec![]),
        }
    }

    pub fn piece(&self, name: Type, color: Color) -> Option<Box<dyn Piece>> {
        match name {
            Type::King => Some(Box::new(King::new(color, Rc::clone(&self.board)))),
            Type::Queen => Some(Box::new(Queen::new(color, Rc::clone(&self.board)))),
            Type::Rook => Some(Box::new(Rook::new(color, Rc::clone(&self.board)))),
            Type::Bishop => Some(Box::new(Bishop::new(color, Rc::clone(&self.board)))),
            Type::Knight => Some(Box::new(Knight::new(color, Rc::clone(&self.board)))),
            Type::Pawn => Some(Box::new(Pawn::new(color, Rc::clone(&self.board)))),
        }
    }

    pub fn settting(&self) {
        for (rank, row) in self.board.squares.iter().enumerate() {
            for (file, square) in row.iter().enumerate() {
                if rank == 0 || rank == 1 || rank == 6 || rank == 7 {
                    let p = match rank {
                        1 | 6 => self.piece(
                            Type::Pawn,
                            if rank == 1 {
                                Color::Black
                            } else {
                                Color::White
                            },
                        ),
                        0 | 7 => match file {
                            0 | 7 => self.piece(
                                Type::Rook,
                                if rank == 0 {
                                    Color::Black
                                } else {
                                    Color::White
                                },
                            ),
                            1 | 6 => self.piece(
                                Type::Knight,
                                if rank == 0 {
                                    Color::Black
                                } else {
                                    Color::White
                                },
                            ),
                            2 | 5 => self.piece(
                                Type::Bishop,
                                if rank == 0 {
                                    Color::Black
                                } else {
                                    Color::White
                                },
                            ),
                            3 => self.piece(
                                Type::Queen,
                                if rank == 0 {
                                    Color::Black
                                } else {
                                    Color::White
                                },
                            ),
                            4 => self.piece(
                                Type::King,
                                if rank == 0 {
                                    Color::Black
                                } else {
                                    Color::White
                                },
                            ),
                            _ => None,
                        },
                        _ => None,
                    };
                    *square.piece.borrow_mut() = p;
                }
            }
        }
    }

    

    pub fn get_possible_moves(&self, color: Color, selection: &str) -> Result<Vec<Point>, String> {
        match self.get_square(color, selection) {
            Ok(square) => {
                let mut moves: Vec<Point> = square.moves();
                moves.extend(square.particular_moves());
                Result::Ok(moves)
            }
            Err(e) => Result::Err("hello".to_owned()),
        }
    }
    pub fn is_valid_move(&self, color: Color, selection: &str, moves: Vec<Point>) -> bool {
        match self.get_square(color, selection) {
            Ok(square) => moves.contains(&square.point),
            Err(e) => false,
        }
    }

    pub fn get_square(&self, color: Color, selection: &str) -> Result<&Square, Box<dyn error::Error>> {
        let src_square = self.board.get_square(&selection);
        match src_square {
            Ok(s) => Result::Ok(s),
            Err(e) => Err(Box::new(e)),
        }
    }
    pub fn get_piece(&self, color: Color, selection: &str) -> Result<&Square, Box<dyn error::Error>> {
        let src_square = self.board.get_square(&selection);
        match src_square {
            Ok(s) => {
                if s.piece.borrow().is_some() {
                    if s.props().color == color {
                        return Result::Ok(s);
                    } else {
                        Result::Err(Box::new(InvalidateInutError))
                    }
                } else {
                    Result::Err(Box::new(InvalidateInutError))
                }
            }
            Err(e) => Err(Box::new(e)),
        }
    }

    pub fn process(&self, color: Color, square: &Square, square2: &Square) -> Notation {

        let pieces = self.board.get_pieces_by_color(color);
        let notation = pieces
            .into_iter()
            .filter(|x| {
                let m = x.piece.borrow();
                let p = m.as_ref().unwrap();
                if x.point!=square.point && p.get_props().name == square.piece.borrow().as_ref().unwrap().get_props().name{
                    //same piece but different point
                    let mut points = p.moves();
                    points.extend(p.particular_moves());
                    points.contains(&square2.point) 
                }else{
                    false
                }

            })
            .map(|x| {
                    let p = x.point;
                    let mut r = String::new();
                    if square.point.rank == p.rank {
                        r.push_str(&square.point.file.to_string());
                    }
                    if square.point.file == p.file {
                        r.push_str(&square.point.rank.to_string());
                    }
                    r
            })
            .reduce(|acc, e| if acc.len() > e.len() { acc } else { e });
        
        let mut capture=false;
        if self.is_castling(square, square2) {
            self.castling(square, square2);
        } else if self.is_en_passant(square, square2) {
            self.en_passant(square, square2);
        } else {
            let mut piece = self.board.takes(square.point);
            piece = self.board.replace(square2.point, piece);
            if piece.is_some(){
                capture=true;
            }
            if self.is_promotion(square2) {
                self.promotion(square2)
            }
        }
        let attack = if self.is_check_mate(color.opposite()) {
            Attack::CheckMate
        }else if self.is_under_check(color.opposite()){
                Attack::Check
        }else{
            Attack::None
        };

        let record = Notation::new(
            if color == Color::White {
                self.white_record.borrow().len()+1
            } else {
                self.black_record.borrow().len()+1
            },
            color,
            square2.piece.borrow().as_ref().unwrap().get_props().name,
            if notation.is_some(){notation.unwrap()}else{String::new()},
            square.point,
            square2.point,
            capture,
            attack
        );
        self.board.add_record(record.clone());
        record
    }

    pub fn is_under_check(&self, color: Color) -> bool {
        let squares = self.board.get_pieces_by_color(color);
        let king = squares
            .iter()
            .find(|x| x.props().name == Type::King)
            .unwrap();

        let opponent = self.board.get_pieces_by_color(if color == Color::White {
            Color::Black
        } else {
            Color::White
        });

        let mut is_king_under_check = false;
        for s in opponent {
            let opponent_moves = s.moves();
            if opponent_moves.contains(&king.point) {
                is_king_under_check = true;
                break;
            }
        }
        is_king_under_check
    }

    pub fn is_check_mate(&self, color: Color) -> bool {
        let squares = self.board.get_pieces_by_color(color);
        let king = squares
            .iter()
            .find(|x| x.piece.borrow().as_ref().unwrap().get_props().name == Type::King)
            .unwrap();
        let moves = king.piece.borrow().as_ref().unwrap().moves();

        let enimies = self.board.get_pieces_by_color(color.opposite());

        let mut is_king_under_check = false;
        'outer: for s in enimies {
            let points = s.moves();
            for m in moves.iter() {
                if !points.contains(m) {
                    is_king_under_check = true;
                    break 'outer;
                }
            }
        }
        is_king_under_check
    }

    pub fn draw_board(&self, color: Color) {
        self.board.draw(color);
    }

    fn is_promotion(&self, square: &Square) -> bool {
        if square.props().name == Type::Pawn {
            if square.point.rank == 8 && square.props().color == Color::White {
                return true;
            }
            if square.point.rank == 1 && square.props().color == Color::Black {
                return true;
            }
        }
        false
    }

    fn promotion(&self, square: &Square) {
        println!("queen, rook, bishop, or knight");
        let color = square.piece.borrow().as_ref().unwrap().get_props().color;
        let mut pick = String::new();
        std::io::stdin()
            .read_line(&mut pick)
            .expect("failed to readline");
        let piece: Option<Box<dyn Piece>> = match &*pick.trim() {
            "queen" => Some(Box::new(Queen::new(color, Rc::clone(&self.board)))),
            "rook" => Some(Box::new(Rook::new(color, Rc::clone(&self.board)))),
            "bishop" => Some(Box::new(Bishop::new(color, Rc::clone(&self.board)))),
            "knight" => Some(Box::new(Knight::new(color, Rc::clone(&self.board)))),
            _ => None,
        };
        *square.piece.borrow_mut() = piece
    }

    fn is_en_passant(&self, square: &Square, square2: &Square) -> bool {
        if square.piece.borrow().is_some() {
            let props = square.props();
            if square.point.rank == 5 && props.color == Color::White && props.name == Type::Pawn {
                if let Some(top_left) = square.point.top_left(props.color) {
                    if top_left == square2.point && square2.is_none() {
                        return true;
                    }
                }
                if let Some(top_right) = square.point.top_right(props.color) {
                    if top_right == square2.point && square2.is_none() {
                        return true;
                    }
                }
            }

            if square.point.rank == 4 && props.color == Color::Black && props.name == Type::Pawn {
                if let Some(top_left) = square.point.top_left(props.color) {
                    if top_left == square2.point && square2.is_none() {
                        return true;
                    }
                }
                if let Some(top_right) = square.point.top_right(props.color) {
                    if top_right == square2.point && square2.is_none() {
                        return true;
                    }
                }
            }
        }

        false
    }

    fn en_passant(&self, square: &Square, square2: &Square) {
        let p = Point::new(square2.point.file, square.point.rank);
        self.board.takes(p);

        let mut piece = self.board.takes(square.point);
        piece = self.board.replace(square2.point, piece);
    }


    fn is_castling(&self, square: &Square, square2: &Square) -> bool {
        square.is_some()
            && square.props().name == Type::King
            && square2.is_none()
            && (square.point.file as u8).abs_diff(square2.point.file as u8) == 2
    }

    fn castling(&self, square: &Square, square2: &Square) {
        let rook = if square2.point.file > 'e' {
            if square.props().color == Color::White {
                self.board.get_square("h1").unwrap()
            } else {
                self.board.get_square("h8").unwrap()
            }
        } else {
            if square.props().color == Color::White {
                self.board.get_square("a1").unwrap()
            } else {
                self.board.get_square("a8").unwrap()
            }
        };

        let mut piece = self.board.takes(square.point);
        piece = self.board.replace(square2.point, piece);

        let file = if rook.point.file == 'a' { 'd' } else { 'f' };
        let p = Point::new(file, rook.point.rank);
        piece = self.board.takes(rook.point);
        piece = self.board.replace(p, piece);
    }

    fn replay(&self, white_notations: Vec<String>, black_notations: Vec<String>) {}

    //number.piece (src) (capture) dst (check)
    //1.b4
    //3.Kc3
    //R1a3
    //Qh4e1
    pub fn interpret(&self, color:Color, notation:String)->Notation{
        let split =notation.split('.').collect::<Vec<_>>();
        let mut capture=false;
        let mov= usize::from_str_radix(split[0], 10).unwrap();
        let mut n=split[1];

        //name check
        let x= n.chars().nth(0).unwrap();
        let name =
            if 'K'==x{
                n=&n[1..];
                Type::King
            }else if 'Q'==x{
                n=&n[1..];
                Type::Queen
            }else if 'R'==x{
                n=&n[1..];
                Type::Rook
            }else if 'B'==x{
                n=&n[1..];
                Type::Bishop
            }else if 'N'==x{
                n=&n[1..];
                Type::Knight
            }else{
                Type::Pawn
            };


        //checkmate check
        let last = n.chars().last().unwrap();
        let attack =if last=='+'{
            n=&n[..n.len()];
            Attack::Check
        }else if last=='#'{
            n=&n[..n.len()];
            Attack::CheckMate
        }else{
            Attack::None
        };

        //destination point check
        let dst =&n[n.len()-2..];
        n=&n[..n.len()-2];

        //capture check
        if n.chars().last().is_some(){
            if n.chars().last().unwrap()=='x'{
                capture=true;                
                n=&n[..n.len()-1];
            }
        };

        let destination= Point::new(dst.as_bytes()[0] as char, (dst.as_bytes()[1]-48) as u32);
        //source point check

        println!("{:?}",name);
        let candidates= self.board.get_pieces_by_color(color)
        .into_iter()
        .filter(|x|x.props().name==name)
        .filter(|x|{
            let mut moves = x.piece.borrow().as_ref().unwrap().moves();
            print!("{:?}",moves);
            println!("{:?}",destination);
            moves.extend(x.piece.borrow().as_ref().unwrap().particular_moves());
            moves.contains(&destination)
        })
        .map(|x|x.point)
        .collect::<Vec<_>>();


        if n.len()==0{
            println!("{:?}",*candidates.first().unwrap());
            return Notation::new(mov,Color::White,name,n.to_string(),*candidates.first().unwrap(),destination,capture,attack);
        }
        if n.len()==1{
            if n.chars().last().unwrap().is_numeric(){
                let rank = n.chars().last().unwrap().to_digit(10).unwrap();
                let c =candidates.iter().filter(|x|x.rank==rank).collect::<Vec<_>>().first().unwrap().clone();
                return Notation::new(mov,Color::White,name,n.to_string(),*c,destination,capture,attack);
            }
            let file= n.chars().last().unwrap();
            let c = candidates.iter().filter(|x|x.file==file).collect::<Vec<_>>().first().unwrap().clone();
            return Notation::new(mov,Color::White,name,n.to_string(),*c,destination,capture,attack);
        } else if n.len()==2{
            let file = n.as_bytes()[0] as char;
            let rank = (n.as_bytes()[1]-48) as u32;
            let c = candidates.iter().filter(|x|x.file==file && x.rank==rank).collect::<Vec<_>>().first().unwrap().clone();
            return Notation::new(mov,Color::White,name,n.to_string(),*c,destination,capture,attack);
        }else{
            println!("{}",n.len());
            unreachable!()
        }
        //return Notation::new(mov,Color::White,name,n.to_string(),src,destination,capture,under);
    }
}
