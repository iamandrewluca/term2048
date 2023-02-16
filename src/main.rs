#![allow(unused)]

use crossterm::event::{read, Event, KeyCode};
use crossterm::{cursor, event, execute, style, terminal};
use std::{fmt, io, process};

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
        // or, alternatively:
        // fmt::Debug::fmt(self, f)
    }
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
        [32, 14, 0, 128],
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

    let msg_position = cursor::MoveTo(2, 18);

    loop {
        draw_grid(grid);

        let action = get_action();

        match action {
            Action::Exit => break,
            Action::Navigate(direction) => message(direction.to_string()),
            Action::Ignore => {}
        }
    }

		message("Good bye!");
    execute!(out, cursor::Show);
    terminal::disable_raw_mode();
    process::exit(0);
}

fn mutate_grid(grid: [[u16; 4]; 4], direction: Direction) {}

fn message<S: Into<String>>(msg: S) {
    execute!(
        io::stdout(),
        cursor::MoveTo(2, 18),
        terminal::Clear(terminal::ClearType::CurrentLine),
        style::Print(msg.into())
    );
}

fn draw_grid(grid: [[u16; 4]; 4]) {
    for i in 0..grid.len() {
        let line = grid[i];
        for j in 0..line.len() {
            let cell = line[j];
            let cell_len = cell.to_string().len() as u16;
            let x = 4 + j as u16 * 10 + (4 - cell_len) / 2;
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
