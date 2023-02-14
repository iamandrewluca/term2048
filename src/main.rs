use crossterm::event::{read, Event, KeyCode};
use crossterm::execute;
use std::io::stdout;

enum Action {
    Exit,
    Navigate(crossterm::event::KeyCode),
    Ignore,
}

fn main() {
    crossterm::terminal::enable_raw_mode().unwrap();
    let grid = [[0u16; 4]; 4];

    loop {
        draw_board(grid);

        let action = get_action();

        match action {
            Action::Exit => {
                break;
            }
            Action::Navigate(a) => println!("{:?}", a),
            Action::Ignore => {}
        }
    }
}

fn draw_board(grid: [[u16; 4]; 4]) {
    for (_i, line) in grid.iter().enumerate() {
        execute!(stdout(), crossterm::cursor::MoveToNextLine(1),).unwrap();
        for (_j, cell) in line.iter().enumerate() {
            execute!(
                stdout(),
                crossterm::style::Print(cell),
                crossterm::cursor::MoveRight(2),
            )
            .unwrap();
        }
    }
}

fn get_action() -> Action {
    match read().unwrap() {
        Event::Key(e) => match e.code {
            KeyCode::Esc => Action::Exit,
            KeyCode::Left => Action::Navigate(KeyCode::Left),
            KeyCode::Right => Action::Navigate(KeyCode::Right),
            KeyCode::Up => Action::Navigate(KeyCode::Up),
            KeyCode::Down => Action::Navigate(KeyCode::Down),
            _ => Action::Ignore,
        },
        _ => Action::Ignore,
    }
}
