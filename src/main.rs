#![allow(unused)]

use crossterm::event::{read, Event, KeyCode};
use crossterm::{cursor, event, execute, style, terminal};
use std::{io, process};

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
    terminal::enable_raw_mode();

    let mut grid = [
        [0, 2, 64, 8],
        [32, 14, 0, 0],
        [0, 0, 0, 1024],
        [0, 2048, 0, 0],
    ];

    let mut out = io::stdout();

    execute!(out, cursor::Hide);

    // Clear screen
    execute!(
        out,
        terminal::Clear(terminal::ClearType::All),
        cursor::MoveTo(0, 0)
    );

    // Drawborders
    execute!(out, style::Print(BORDERS.join("\n\r")),);

    loop {
        draw_grid(grid);

        let action = get_action();

        match action {
            Action::Exit => {
                execute!(
                    io::stdout(),
                    cursor::MoveTo(2, 18),
                    style::Print("Good bye!")
                );
                break;
            }
            Action::Navigate(direction) => {}
            Action::Ignore => {}
        }
    }

    execute!(out, cursor::Show);
    terminal::disable_raw_mode();
    process::exit(0);
}

fn mutate_grid(grid: [[u16; 4]; 4], direction: Direction) {}

fn draw_grid(grid: [[u16; 4]; 4]) {
    for i in 0..grid.len() {
        let line = grid[i];
        for j in 0..line.len() {
            let cell = line[j];
            let x = 4 + j as u16 * 10;
            let y = 2 + i as u16 * 4;
            execute!(io::stdout(), cursor::MoveTo(x, y), style::Print(cell));
        }
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

// Yes, but it looks good
const BORDERS: &'static [&'static str] = &[
    "[][][][][][][][][][][][][][][][][][][][][]",
    "[]        []        []        []        []",
    "[]        []        []        []        []",
    "[]        []        []        []        []",
    "[][][][][][][][][][][][][][][][][][][][][]",
    "[]        []        []        []        []",
    "[]        []        []        []        []",
    "[]        []        []        []        []",
    "[][][][][][][][][][][][][][][][][][][][][]",
    "[]        []        []        []        []",
    "[]        []        []        []        []",
    "[]        []        []        []        []",
    "[][][][][][][][][][][][][][][][][][][][][]",
    "[]        []        []        []        []",
    "[]        []        []        []        []",
    "[]        []        []        []        []",
    "[][][][][][][][][][][][][][][][][][][][][]",
];
