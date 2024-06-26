//function to get X, O or the number to enter for the position
fn board_char(board : u32, pos : u8) -> char {
    let x : usize = ((board >> (pos * 2)) & 3) as usize;
    if x == 0 {
        return (pos + 49 as u8) as char;
    }
    return " XO?".as_bytes()[x] as char;
}

fn print_board(board : u32){
    println!("{}|{}|{}", board_char(board, 0), board_char(board, 1), board_char(board, 2));
    println!("-----");
    println!("{}|{}|{}", board_char(board, 3), board_char(board, 4), board_char(board, 5));
    println!("-----");
    println!("{}|{}|{}", board_char(board, 6), board_char(board, 7), board_char(board, 8));
}

fn set_board_pos(board : u32, pos : u8, player : u8) -> u32 {
    return board ^ (1 << (pos * 2 + player));
}

fn is_board_pos_empty(board : u32, pos : u8) -> bool {
    return (board >> (pos * 2) & 3) == 0;
}

fn is_board_full(board : u32) -> bool {
    let mut r : bool = true;
    for pos in 0..9 {
        r &= !is_board_pos_empty(board, pos);
    }
    return r;
}

// 0: no win, 1: X win, 2: O win, 3: tie
fn get_board_win(board : u32) -> u8 {
    //straight lines
    for i in 0..3 {
        if (board >> (i * 6)) & 21 == 21 {
            return 1;
        }

        if board & (4161 << i * 2) == (4161 << i * 2) {
            return 1;
        }

        if (board >> (i * 6)) & 42 == 42 {
            return 2;
        }

        if board & (8322 << i * 2) == (8322 << i * 2) {
            return 2;
        }
    }
    //diagonals
    for i in 0..2 {
        if board & (65793 << i) == (65793 << i) {
            return i + 1;
        }

        if board & (4368 << i) == (4368 << i) {
           return i + 1;
        }
    }
    if is_board_full(board) {
        return 3;
    }
    return 0;
}

fn main(){

    let mut board : u32 = 0;

    let mut cplayer : u8 = 0;

    print_board(board);

    while (get_board_win(board) == 0) &! is_board_full(board){
        
        println!("{}'s turn: ", "XO".as_bytes()[cplayer as usize] as char);

        let mut input_line = String::new();

        std::io::stdin().read_line(&mut input_line)
            .expect("Failed to read line.");

        let mut input_num : u8 = match input_line.trim().parse() {
            Ok(n) => n,
            Err(_) => { println!("Invalid input."); continue; },
        };

        input_num = input_num - 1;

        if input_num > 9 { println!("Invalid number input."); continue; }
        if !is_board_pos_empty(board, input_num) { println!("That place is occupied."); continue; }

        board = set_board_pos(board, input_num, cplayer);

        print_board(board);

        cplayer = 1 - cplayer;
    }
    match get_board_win(board) {
        0 => { println!("Something went wrong."); },
        1 => { println!("X won!"); },
        2 => { println!("O won!"); },
        3 => { println!("Tie!"); },
        4.. => { println!("Something went VERY wrong."); },
    }
}