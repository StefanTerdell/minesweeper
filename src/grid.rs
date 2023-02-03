use console::Term;
use rand::random;
use std::cmp;

#[derive(PartialEq)]
pub enum GameState {
    Playing,
    Won,
    Lost,
}

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

pub fn create_grid(size_x: usize, size_y: usize, incidence: f32) -> Grid {
    let mut grid: Grid = Vec::new();

    for _ in 0..size_y {
        let mut column = Vec::new();

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

            column.push(cell)
        }

        grid.push(column);
    }

    count_mines(&mut grid);

    grid
}

pub fn count_mines(grid: &mut Grid) {
    for y in 0..grid.len() {
        let max_y = grid.len() - 1;

        for x in 0..grid[y].len() {
            let max_x = grid[y].len() - 1;

            if grid[y][x].content == CellContent::Mine {
                continue;
            }

            let mut count = 0;

            for ry in if y == 0 { 0 } else { y - 1 }..=cmp::min(y + 1, max_y) {
                for rx in if x == 0 { 0 } else { x - 1 }..=cmp::min(x + 1, max_x) {
                    if grid[ry][rx].content == CellContent::Mine {
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
    let mut no_invisible = true;

    let mut mines = 0;
    let mut flags = 0;

    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            let cell = &grid[y][x];
            no_invisible = no_invisible && cell.state != CellState::Invisible;

            if cell.state == CellState::Flagged {
                flags += 1;
            }

            if cell.content == CellContent::Mine {
                mines += 1;
            }
        }
    }

    no_invisible && mines == flags
}

// Returns true if you revealed a mine
pub fn reveal(grid: &mut Grid, x: usize, y: usize) -> bool {
    if grid[y][x].state != CellState::Invisible {
        return false;
    }

    grid[y][x].state = CellState::Visible;

    if grid[y][x].content == CellContent::Mine {
        return true;
    }

    if grid[y][x].content == CellContent::Empty {
        let max_y = grid.len() - 1;
        for ry in if y == 0 { 0 } else { y - 1 }..=cmp::min(y + 1, max_y) {
            let max_x = grid[y].len() - 1;
            for rx in if x == 0 { 0 } else { x - 1 }..=cmp::min(x + 1, max_x) {
                reveal(grid, rx, ry);
            }
        }
    }

    false
}
