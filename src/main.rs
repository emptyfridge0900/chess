use std::{cell::RefCell, rc::Rc, borrow::BorrowMut};

use chess::{Type, Square, Color, Point, Props, piece::{Piece, Rook, Knight, Bishop, King, Queen, Pawn}, error, player::Player, board::Board};
use regex::Regex;
fn main() {

    let mut manager = ChessManager::new();
    manager.start();

}

struct ChessManager{
    board:Rc<Board>,
    isRunning:bool,
    player1:Player,
    player2:Player
}
impl ChessManager{
    fn new()->ChessManager{
        let board = Rc::new(Board::new());
        ChessManager{
            board:Rc::clone(&board),
            isRunning:false,
            player1:Player{board:Rc::clone(&board),color:Color::White},
            player2:Player{board:Rc::clone(&board),color:Color::Black}
        }
    }
    fn start(&mut self){
        self.isRunning = true;
        self.draw_board();
        

        while self.isRunning{
            self.player1.turn();
            self.draw_board();
            self.player2.turn();
            self.draw_board();
            
        }
    }
    fn stop(&mut self){
        self.isRunning = false;
    }
    fn en_passant(&self){

    }
    fn castling(&self){

    }
    fn pomotion(&self){

    }

    fn draw_board(&self){
        self.board.draw();
    }
}
