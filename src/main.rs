const ROWS: usize = 70;
const COLS: usize = 70;
use std::{thread, time};

enum Directions {
    Left,
    Right,
    Up,
    Down,
}

fn main() {
    let mut ant: (usize, usize) = (ROWS/2, COLS/2);
    let mut grid = [[false;ROWS];COLS];
    let mut generations = 0;
    let interval = time::Duration::from_millis(1);
    print!("\x1b[2J"); //clears the screen, on linux
    let mut direction: Directions = Directions::Right;
    loop {
        update_grid(&mut grid, &mut direction, &mut ant);
        print!("\x1b[0;0H"); //moves the cursor at the top left corner, on linux
        print_grid(&grid);
        print!("generations: {}", generations);
        generations = generations + 1;
        thread::sleep(interval);
    }
}

fn print_grid(grid: &[[bool; ROWS];COLS]) {
    for i in 1..ROWS {
        for j in 1..COLS {
            print!("{}", if grid[i][j] == true {
                "o"
            } else {
                " "
            });
        }
		print!("\n");
    }
}
fn update_grid(grid: &mut [[bool;ROWS];COLS],
               direction: &mut Directions,
               ant: &mut (usize, usize)) {
    match grid[ant.0][ant.1] {
        true => {
            match direction {
                Directions::Right => {
                    let n_ant = (ant.0, ant.1 - 1);
                    //flip the grid tile
                    grid[ant.0][ant.1] = !grid[ant.0][ant.1];
                    *direction = Directions::Up;
                    *ant = n_ant;
                },
                Directions::Up => {
                    let n_ant = (ant.0 - 1, ant.1);
                    grid[ant.0][ant.1] = !grid[ant.0][ant.1];
                    *direction = Directions::Left;
                    *ant = n_ant;
                },
                Directions::Left => {
                    let n_ant = (ant.0, ant.1 + 1);
                    grid[ant.0][ant.1] = !grid[ant.0][ant.1];
                    *direction = Directions::Down;
                    *ant = n_ant;
                },
                Directions::Down => {
                    let n_ant = (ant.0 + 1, ant.1);
                    grid[ant.0][ant.1] = !grid[ant.0][ant.1];
                    *direction = Directions::Right;
                    *ant = n_ant;
                },
            }
        },
        false => {
            match direction {
                Directions::Right => {
                    let n_ant = (ant.0, ant.1 + 1);
                    //flip the grid tile
                    grid[ant.0][ant.1] = !grid[ant.0][ant.1];
                    *direction = Directions::Down;
                    *ant = n_ant;
                },
                Directions::Down => {
                    let n_ant = (ant.0 - 1, ant.1);
                    grid[ant.0][ant.1] = !grid[ant.0][ant.1];
                    *direction = Directions::Left;
                    *ant = n_ant;
                },
                Directions::Left => {
                    let n_ant = (ant.0, ant.1 - 1);
                    grid[ant.0][ant.1] = !grid[ant.0][ant.1];
                    *direction = Directions::Up;
                    *ant = n_ant;
                },
                Directions::Up => {
                    let n_ant = (ant.0 + 1, ant.1);
                    grid[ant.0][ant.1] = !grid[ant.0][ant.1];
                    *direction = Directions::Right;
                    *ant = n_ant;
                },
            }
        }
    }
}


