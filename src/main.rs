use std::io::{stdout, Write};
use crossterm::{
    execute,
    cursor::MoveTo,
    terminal::{Clear, ClearType, enable_raw_mode, disable_raw_mode},
    event::{self, Event, KeyCode},
};


fn clear_and_print(message: &String) {
    let mut out = stdout();
    execute!(out, Clear(ClearType::All), MoveTo(0, 0)).unwrap();
    write!(out, "{}", message).unwrap();
    out.flush().unwrap();
}


fn generate_board_string(board: &Vec<Vec<Option<char>>>, cursor: (usize, usize)) -> String {
    let mut message = String::new();
    for (row_i, row) in board.iter().enumerate() {
        for (col_i, cell) in row.iter().enumerate() {
            let symbol = match cell {
                Some('B') => "B",
                Some('W') => "W",
                None => ".",
                _ => "?",
            };

            if (row_i, col_i) == cursor {
                message.push_str(&format!("\x1b[31m{}\x1b[0m ", symbol));
            } else {
                message.push_str(&format!("{} ", symbol));
            }
        }
        message.push_str("\r\n");
    }
    message
}

fn move_cursor(cursor: (usize, usize), direction: &str) -> (usize, usize) {
    let (row, col) = cursor;
    match direction {
        "up" => (row.saturating_sub(1), col),
        "down" => (if row < 7 { row + 1 } else { row }, col),
        "left" => (row, col.saturating_sub(1)),
        "right" => (row, if col < 7 { col + 1 } else { col }),
        _ => cursor, 
    }
}

fn flip_stones(
    board: &mut Vec<Vec<Option<char>>>,
    cursor: (usize, usize),
    player_color: char
) {
    let opponent = if player_color == 'B' { 'W' } else { 'B' };

    let directions = [
        (-1, 0), (1, 0),
        (0, -1), (0, 1),
        (-1, -1), (-1, 1),
        (1, -1), (1, 1),
    ];

    let (row, col) = cursor;
    let size = board.len();

    for (dr, dc) in directions.iter() {
        let mut r = row as isize + dr;
        let mut c = col as isize + dc;
        let mut to_flip: Vec<(usize, usize)> = Vec::new();

        //相手の石が続く間、記録
        while r > 0 && r< size as isize && c >= 0 && c < size as isize {
            match board[r as usize][c as usize] {
                Some(stone) if stone == opponent => {
                    to_flip.push((r as usize, c as usize));
                }
                Some(stone) if stone == player_color => {
                    for (fr, fc) in to_flip {
                        board[fr][fc] = Some(player_color);
                    }
                    break;
                }
                _ => break,
            }
            r += dr;
            c += dc;
        }
    }
}

fn set_stone(cursor: (usize, usize), board: &mut Vec<Vec<Option<char>>>, turn_black: &mut bool) {
    let (row, col) = cursor;
    if board[row][col].is_none() {
        if *turn_black {
            board[row][col] = Some('B');
        } else {
            board[row][col] = Some('W');
        }
        let player_color = if *turn_black { 'B' } else { 'W' };
        flip_stones(board, cursor, player_color);
        *turn_black = !*turn_black;
    }
}


fn main() {

    enable_raw_mode().unwrap();
    let mut board: Vec<Vec<Option<char>>> = vec![vec![None; 8]; 8];

    board[3][3] = Some('W');
    board[3][4] = Some('B');
    board[4][3] = Some('B');
    board[4][4] = Some('W');

    let mut cursor: (usize, usize) = (4,4);
    
    let mut turn_black = true;

    let initBoard = generate_board_string(&board, cursor);
    clear_and_print(&initBoard);

    loop {
        let board_str = generate_board_string(&board, cursor);
        clear_and_print(&board_str);

        if let Event::Key(key_event) = event::read().unwrap() {
            match key_event.code {
                KeyCode::Char('i') => cursor = move_cursor(cursor, "up"),
                KeyCode::Char('k') => cursor = move_cursor(cursor, "down"),
                KeyCode::Char('j') => cursor = move_cursor(cursor, "left"),
                KeyCode::Char('l') => cursor = move_cursor(cursor, "right"),
                KeyCode::Enter => set_stone(cursor, &mut board, &mut turn_black),
                KeyCode::Char('q') => break, // qで終了
                _ => {}

            }
        }
    }
    disable_raw_mode().unwrap();
}

