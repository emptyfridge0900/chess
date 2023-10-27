use std::{
    borrow::BorrowMut,
    cell::{Ref, RefCell},
    rc::Rc,
};

use chess::{
    board::{Board, Record},
    error,
    piece::{Bishop, King, Knight, Pawn, Piece, Queen, Rook},
    player::Player,
    Color, Point, Props, Square, Type,
};
use regex::Regex;
fn main() {
    let mut manager = ChessManager::new();
    manager.settting();
    manager.start();
}


struct ChessManager {
    board: Rc<Board>,
    is_running: bool,
    player1: Player,
    player2: Player,
}
impl ChessManager {
    fn new() -> ChessManager {
        let board = Rc::new(Board::new());
        ChessManager {
            board: Rc::clone(&board),
            is_running: false,
            player1: Player {
                board: Rc::clone(&board),
                color: Color::White,
            },
            player2: Player {
                board: Rc::clone(&board),
                color: Color::Black,
            },
        }
    }

    fn piece(&self, name: Type, color: Color) -> Option<Box<dyn Piece>> {
        match name {
            Type::King => Some(Box::new(King::new(color, Rc::clone(&self.board)))),
            Type::Queen => Some(Box::new(Queen::new(color, Rc::clone(&self.board)))),
            Type::Rook => Some(Box::new(Rook::new(color, Rc::clone(&self.board)))),
            Type::Bishop => Some(Box::new(Bishop::new(color, Rc::clone(&self.board)))),
            Type::Knight => Some(Box::new(Knight::new(color, Rc::clone(&self.board)))),
            Type::Pawn => Some(Box::new(Pawn::new(color, Rc::clone(&self.board)))),
        }
    }

    fn settting(&self) {
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
                            1|6=>self.piece(Type::Knight, if rank==0{Color::Black}else{Color::White}),
                            2|5=>self.piece(Type::Bishop, if rank==0{Color::Black}else{Color::White}),
                            3=>self.piece(Type::Queen, if rank==0{Color::Black}else{Color::White}),
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

    fn convert(&self, record:Record) -> String {
        let n = match record.name {
            Type::King => 'K',
            Type::Queen => 'Q',
            Type::Rook => 'R',
            Type::Bishop => 'B',
            Type::Knight => 'N',
            Type::Pawn => ' ',
        };

        let pieces = self.board.get_pieces_by_color(record.color);
        let notation = pieces
            .iter()
            .filter(|x| x.piece.borrow().as_ref().unwrap().get_props().name == record.name)
            .map(|x| {
                let m = x.piece.borrow();
                let p = m.as_ref().unwrap();
                if p.get_props().name == record.name && x.point != record.src {
                    //same piece but different point
                    let points = p.moves();
                    if points.contains(&record.dst)
                    //&& !self.is_blocked(x.point, des)
                    {
                        Some(x.point)
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .map(|x| {
                let notation = if x.is_some() {
                    let p = x.unwrap();
                    let mut r = String::new();
                    if record.src.rank == p.rank {
                        r.push_str(&record.src.file.to_string());
                    }
                    if record.src.file == p.file {
                        r.push_str(&record.src.rank.to_string());
                    }
                    r
                } else {
                    String::new()
                };
                notation
            })
            .reduce(|acc, e| if acc.len() > e.len() { acc } else { e });

        let mut result = n.to_string();
        if notation.is_some() {
            result.push_str(&notation.unwrap());
        }
        result.push_str(&record.dst.notation());
        result
    }

    fn turn(&self, color:Color){
        
    }
    fn start(&mut self) {
        self.is_running = true;
        //self.draw_board(Color::White);

        let mut white_record: Vec<String> = vec![];
        let mut black_record: Vec<String> = vec![];

        while self.is_running {
            let mut valid = false;

            //while !valid {
                if self.is_under_check(Color::White){
                    if self.is_check_mate(Color::White){
                        self.stop();
                        break;
                    }
                    println!("you are under check");
                }
                self.draw_board(Color::White);
                let square = self.player1.select_piece();

                let mut moves: Vec<Point> = square.piece.borrow().as_ref().unwrap().moves();
                moves.extend(square.piece.borrow().as_ref().unwrap().particular_moves());
                println!("{:?}", moves);

                let square2 = self.player1.select_moving_point();
                if !moves.contains(&square2.point){
                    println!("Invalid!");
                    continue;
                }

                let record = Record::new(
                        Color::White,
                        square.piece.borrow().as_ref().unwrap().get_props().name,
                        square.point,
                        square2.point
                    );
                if self.is_castling(square, square2) {
                    self.castling(square, square2);
                    continue;
                } else if self.is_en_passant(square, square2) {
                    self.en_passant(square, square2);
                    continue;
                } else {
                    let mut piece = self.board.takes(square.point);
                    piece = self.board.replace(square2.point, piece);

                    //promotion
                    if self.is_promotion(square2) {
                        self.promotion(square2)
                    }
                }
                let n = self.convert(record.clone());
                white_record.push(n);
                self.board.add_record(record.clone());
            //}

            for record in white_record.iter() {
                println!("{:?}", record);
            }

            for record in std::iter::zip(white_record.iter(), black_record.iter()) {
                println!("{}|{}", record.0, record.1);
            }
        }
    }
    fn stop(&mut self) {
        self.is_running = false;
    }

    fn is_promotion(&self, square: &Square) -> bool {
        if square.piece.borrow().as_ref().unwrap().get_props().name == Type::Pawn {
            if square.point.rank == 8
                && square.piece.borrow().as_ref().unwrap().get_props().color == Color::White
            {
                return true;
            }
            if square.point.rank == 1
                && square.piece.borrow().as_ref().unwrap().get_props().color == Color::Black
            {
                return true;
            }
        }
        false
    }

    fn promotion(&self, square: &Square) {
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
            let props = square.piece.borrow().as_ref().unwrap().get_props();
            if square.point.rank == 5
            && props.color == Color::White
            && props.name == Type::Pawn
            {
                if let Some(top_left)=square.point.top_left(props.color){
                    if top_left == square2.point && square2.piece.borrow().is_none(){
                        return true;
                    }
                }
                if let Some(top_right)=square.point.top_right(props.color){
                    if top_right == square2.point && square2.piece.borrow().is_none(){
                        return true;
                    }
                }
            }

            if square.point.rank == 4
            && props.color == Color::Black
            && props.name == Type::Pawn
            {
                if let Some(top_left)=square.point.top_left(props.color){
                    if top_left == square2.point && square2.piece.borrow().is_none(){
                        return true;
                    }
                }
                if let Some(top_right)=square.point.top_right(props.color){
                    if top_right == square2.point && square2.piece.borrow().is_none(){
                        return true;
                    }
                }
            }
        }

        false
    }

    fn en_passant(&self,square: &Square, square2: &Square) {
        let p = Point::new(square2.point.file, square.point.rank);
        self.board.takes(p);

        let mut piece = self.board.takes(square.point);
        piece = self.board.replace(square2.point, piece);

    }

    fn is_under_check(&self, color: Color) -> bool {
        let squares = self.board.get_pieces_by_color(color);
        let king = squares
            .iter()
            .find(|x| x.piece.borrow().as_ref().unwrap().get_props().name == Type::King)
            .unwrap();

        let opponent = self.board.get_pieces_by_color(if color == Color::White {
            Color::Black
        } else {
            Color::White
        });

        let mut is_king_under_check = false;
        for s in opponent {
            let opponent_moves = s.piece.borrow().as_ref().unwrap().moves();
            if opponent_moves.contains(&king.point) {
                is_king_under_check = true;
                break;
            }
        }
        is_king_under_check
    }

    fn is_check_mate(&self, color: Color) -> bool {
        let squares = self.board.get_pieces_by_color(color);
        let king = squares
            .iter()
            .find(|x| x.piece.borrow().as_ref().unwrap().get_props().name == Type::King)
            .unwrap();
        let moves = king.piece.borrow().as_ref().unwrap().moves();

        let enimies = self.board.get_pieces_by_color(if color == Color::White {
            Color::Black
        } else {
            Color::White
        });

        let mut is_king_under_check = false;
        'outer: for s in enimies {
            let points = s.piece.borrow().as_ref().unwrap().moves();
            for m in moves.iter() {
                if !points.contains(m) {
                    is_king_under_check = true;
                    break 'outer;
                }
            }
        }
        is_king_under_check
    }

    fn is_castling(&self, square: &Square, square2: &Square) -> bool {
        square.piece.borrow().is_some()
            && square.piece.borrow().as_ref().unwrap().get_props().name == Type::King
            && square2.piece.borrow().is_none()
            && (square.point.file as u8).abs_diff(square2.point.file as u8) == 2
    }

    fn castling(&self, square: &Square, square2: &Square) {
        let rook = if square2.point.file > 'e' {
            if square.piece.borrow().as_ref().unwrap().get_props().color == Color::White {
                self.board.get_square("h1").unwrap()
            } else {
                self.board.get_square("h8").unwrap()
            }
        } else {
            if square.piece.borrow().as_ref().unwrap().get_props().color == Color::White {
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


    fn draw_board(&self, color: Color) {
        self.board.draw(color);
    }
}