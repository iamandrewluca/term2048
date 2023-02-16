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
    }
}

enum Action {
    Exit,
    Navigate(Direction),
    Ignore,
}

struct Grid {
    size: usize,
    data: Vec<Vec<usize>>,
}

impl Grid {
    fn new(size: usize) -> Grid {
        Grid {
            size,
            data: vec![vec![0; size]; size],
        }
    }

    fn from(data: Vec<Vec<usize>>) -> Grid {
        Grid {
            size: data.len(),
            data,
        }
    }
}

fn main() {
    terminal::enable_raw_mode();

    let grid = Grid::from(vec![
        vec![0, 2, 64, 8],
        vec![32, 16, 0, 128],
        vec![2, 2, 0, 1024],
        vec![0, 2048, 0, 0],
    ]);

    let mut out = io::stdout();

    execute!(out, cursor::Hide);

    // Clear screen
    execute!(
        out,
        terminal::Clear(terminal::ClearType::All),
        cursor::MoveTo(0, 0)
    );

    // Drawborders
    execute!(out, style::Print(BORDERS.join("\n\r")));

    let msg_position = cursor::MoveTo(2, 18);

    loop {
        draw_grid(&grid);

        let action = get_action();

        match action {
            Action::Exit => break,
            Action::Navigate(direction) => {
                // move_grid(grid, direction);
                // collapse_grid(grid, direction);
                // move_grid(grid, direction);
                // generate_on_grid(grid);
                // message(direction.to_string());
            }
            Action::Ignore => {}
        }
    }

    message("Good bye!");
    execute!(out, cursor::Show);
    terminal::disable_raw_mode();
    process::exit(0);
}

fn move_grid(grid: Grid, direction: Direction) {
    let range = match direction {
        Direction::Left => 4..0,
        Direction::Up => 4..0,
        Direction::Right => 0..4,
        Direction::Down => 0..4,
    };
}

// fn collapse_grid(grid: [[u16; 4]; 4], direction: Direction) {}
// fn generate_on_grid(grid: [[u16; 4]; 4]) {}

fn message<S: Into<String>>(msg: S) {
    execute!(
        io::stdout(),
        cursor::MoveTo(2, 18),
        terminal::Clear(terminal::ClearType::CurrentLine),
        style::Print(msg.into())
    );
}

fn draw_grid(grid: &Grid) {
    for i in 0..grid.data.len() {
        let line = grid.data.get(i).unwrap();
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
