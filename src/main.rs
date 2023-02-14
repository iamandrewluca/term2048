#![allow(unused)]

use crossterm::event::{read, Event, KeyCode};
use crossterm::{cursor, event, execute, style, terminal};
use std::io::stdout;

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

enum Action {
    Exit,
    Navigate(Direction),
    Ignore,
}

fn main() {
    terminal::enable_raw_mode().unwrap();
    let mut grid = [
        [0, 2, 64, 8],
        [32, 14, 0, 0],
        [0, 0, 0, 1024],
        [0, 2048, 0, 0],
    ];

    let mut x = 0;
    let y = 0;

    let mut out = stdout();
    let clear_screen = terminal::Clear(terminal::ClearType::All);
    let goto_corner = cursor::MoveTo(0, 0);

    loop {
        execute!(out, clear_screen, goto_corner).unwrap();

        draw_board(grid);

        let action = get_action();

        match action {
            Action::Exit => break,
            Action::Navigate(direction) => {}
            Action::Ignore => {}
        }
    }
}

fn mutate_grid(grid: [[u16; 4]; 4], direction: Direction) {}

fn draw_board(grid: [[u16; 4]; 4]) {
    for line in grid {
        for cell in line {
            print!("{cell: <6}")
        }
        execute!(stdout(), cursor::MoveToNextLine(2)).unwrap();
    }
}

fn get_action() -> Action {
    match read().unwrap() {
        Event::Key(e) => match e.code {
            KeyCode::Char('q') => Action::Exit,
            KeyCode::Char('Q') => Action::Exit,
            KeyCode::Esc => Action::Exit,
            KeyCode::Left => Action::Navigate(Direction::Left),
            KeyCode::Right => Action::Navigate(Direction::Right),
            KeyCode::Up => Action::Navigate(Direction::Up),
            KeyCode::Down => Action::Navigate(Direction::Down),
            _ => Action::Ignore,
        },
        _ => Action::Ignore,
    }
}
