use chess::manager::ChessManager;
use chess::{Color, Point};
fn main() {
    let manager = ChessManager::new();
    manager.settting();
    let mut color = Color::White;//1.b4
    let no =manager.interpret(color, "2.Nc3".to_string());
    println!("{:?}",no);
    loop {
        manager.draw_board(color);

        if manager.is_under_check(color) {
            if manager.is_check_mate(color) {
                println!("Checkmate");
                break;
            }
            println!("you are under check");
        }

        let mut moves: Vec<Point> = vec![];
        let mut src = String::new();
        let mut dst = String::new();
        loop {
            src = String::new();
            std::io::stdin()
                .read_line(&mut src)
                .expect("failed to readline");
            moves = match manager.get_possible_moves(color, &src) {
                Ok(x) => x,
                Err(e) => continue,
            };
            break;
        }
        println!("{:?}", moves);

        loop {
            dst = String::new();
            std::io::stdin()
                .read_line(&mut dst)
                .expect("failed to readline");
            if !manager.is_valid_move(color, &dst, moves.clone()) {
                continue;
            }
            break;
        }
        let m = manager.get_square(color, &src).unwrap();
        let n = manager.get_square(color, &dst).unwrap();
        let r = manager.process(color, m, n);

        if color == Color::White {
            manager.white_record.borrow_mut().push(r.to_string());
        } else {
            manager.black_record.borrow_mut().push(r.to_string());
        }
        color.toggle();

        for record in std::iter::zip(
            manager.white_record.borrow().iter(),
            manager.black_record.borrow().iter(),
        ) {
            println!("{}|{}", record.0, record.1);
        }
    }
}