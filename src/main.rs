mod grid;
mod input;
use grid::*;
use input::*;

fn main() {
    let term = init_term();
    let size_x = 20;
    let size_y = 10;

    let mut grid = create_grid(size_x, size_y, 0.1f32);
    let mut state = GameState::Playing;
    let mut pos_x = 0;
    let mut pos_y = 0;

    print_grid(&grid, &term);

    loop {
        print_brackets(pos_x, pos_y, &term);

        match await_input(&term) {
            'w' => {
                if pos_y > 0 {
                    pos_y -= 1;
                }
            }
            'a' => {
                if pos_x > 0 {
                    pos_x -= 1;
                }
            }
            's' => {
                if pos_y < size_y - 1 {
                    pos_y += 1;
                }
            }
            'd' => {
                if pos_x < size_x - 1 {
                    pos_x += 1;
                }
            }
            ' ' => {
                if reveal(&mut grid, pos_x, pos_y) {
                    state = GameState::Lost;
                } else if check_win(&grid) {
                    state = GameState::Won;
                }

                print_grid(&grid, &term);
            }
            'f' => {
                if grid[pos_y][pos_x].state == CellState::Visible {
                    return;
                }

                if grid[pos_y][pos_x].state == CellState::Flagged {
                    grid[pos_y][pos_x].state = CellState::Invisible;
                } else {
                    grid[pos_y][pos_x].state = CellState::Flagged;

                    if check_win(&grid) {
                        state = GameState::Won;
                    }
                };

                print_grid(&grid, &term);
            }
            _ => break,
        }

        if state != GameState::Playing {
            if state == GameState::Lost {
                println!("You lost!");
            } else {
                println!("You won!")
            }

            await_input(&term);

            break;
        }

        clear_brackets(&term);
    }

    cleanup_term(term)
}
