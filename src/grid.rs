use console::Term;
use rand::random;
use std::{cmp, ops::RangeInclusive};

#[derive(Debug, PartialEq)]
pub enum CellContent {
    Mine,
    Empty,
    Close(i32),
}

#[derive(PartialEq)]
pub enum CellState {
    Visible,
    Invisible,
    Flagged,
}

pub struct Cell {
    pub content: CellContent,
    pub state: CellState,
}

pub type Grid = Vec<Vec<Cell>>;

fn safe_range(n: &usize, length: &usize) -> RangeInclusive<usize> {
    return if *n == 0 { 0 } else { n - 1 }..=cmp::min(n + 1, *length - 1);
}

pub fn create_grid(size_x: usize, size_y: usize, incidence: f32) -> Grid {
    let mut grid: Grid = Vec::with_capacity(size_y);

    for _ in 0..size_y {
        let mut column = Vec::with_capacity(size_x);

        for _ in 0..size_x {
            let has_mine = random::<f32>() < incidence;

            let cell = Cell {
                content: if has_mine {
                    CellContent::Mine
                } else {
                    CellContent::Empty
                },
                state: CellState::Invisible,
            };

            column.push(cell);
        }

        grid.push(column);
    }

    count_mines(&mut grid);

    grid
}

pub fn count_mines(grid: &mut Grid) {
    let max_y = grid.len();
    for y in 0..max_y {
        let max_x = grid[y].len();
        for x in 0..max_x {
            if grid[y][x].content == CellContent::Mine {
                continue;
            }

            let mut count = 0;

            for sy in safe_range(&y, &max_y) {
                for sx in safe_range(&x, &max_x) {
                    if grid[sy][sx].content == CellContent::Mine {
                        count += 1;
                    }
                }
            }

            if count > 0 {
                grid[y][x].content = CellContent::Close(count);
            }
        }
    }
}

pub fn print_grid(grid: &Grid, term: &Term) {
    term.clear_screen().unwrap();

    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            let char = match grid[y][x].state {
                CellState::Invisible => String::from("I"),
                CellState::Flagged => String::from("f"),
                CellState::Visible => match grid[y][x].content {
                    CellContent::Mine => String::from("*"),
                    CellContent::Empty => String::from(" "),
                    CellContent::Close(n) => n.to_string(),
                },
            };

            print!(" {}", char);
        }
        println!("");
    }
}

pub fn check_win(grid: &Grid) -> bool {
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            let cell = &grid[y][x];

            if cell.state == CellState::Invisible {
                return false;
            }

            if (cell.state == CellState::Flagged) != (cell.content == CellContent::Mine) {
                return false;
            }
        }
    }

    true
}

// Returns true if you revealed a mine
pub fn reveal(grid: &mut Grid, x: usize, y: usize) -> bool {
    let cell = &mut grid[y][x];

    if cell.state != CellState::Invisible {
        return false;
    }

    cell.state = CellState::Visible;

    if cell.content == CellContent::Mine {
        return true;
    }

    if cell.content == CellContent::Empty {
        let max_y = grid.len();
        for sy in safe_range(&y, &max_y) {
            let max_x = grid[y].len();
            for sx in safe_range(&x, &max_x) {
                reveal(grid, sx, sy);
            }
        }
    }

    false
}
