extern crate piston_window;
extern crate rand;

use piston_window::*;
use rand::Rng;

use std::{thread, time::Duration};

const WINDOW_WH: u32 = 1200;
const BOARD_WH: usize = 100;
const SLEEP_DUR: u64 = 30;
const RAND_THRESHOLD: f64 = 0.4;

fn update(board: &mut Vec<Vec<bool>>) {

    let mut changes: Vec<(usize, usize)> = Vec::new();

    for y in 1..BOARD_WH {
        for x in 1..BOARD_WH {
            let neighbors: u8 = {
                let mut n: u8 = 0;
                let dirs: &[(i32, i32)] = &[(-1, 0),
                                            (-1, -1),
                                            (0, 1),
                                            (1, 0),
                                            (1, 1),
                                            (1, -1),
                                            (-1, 1),
                                            (0, -1)];
                for &dir in dirs {
                    if board[(y as i32 + dir.0) as usize][(x as i32 + dir.1) as usize] {
                        n += 1; 
                    }
                }
            n
            };

            let cell = board[y][x];

            if cell {
                if neighbors < 2 || neighbors > 3 {
                    changes.push((y, x));
                }
            } else {
                if neighbors == 3 {
                    changes.push((y, x));
                }
            }
        }
    }

    for &c in &changes {
        board[c.0][c.1] = !board[c.0][c.1]; 
    }

}

fn random_board() -> Vec<Vec<bool>> {

    let mut b: Vec<Vec<bool>> = vec![vec![false; BOARD_WH + 2 as usize]; BOARD_WH + 2 as usize];
    let mut rng = rand::thread_rng();

    for y in 1..BOARD_WH {
        for x in 1..BOARD_WH {
            let r: f64 = rng.gen();
            if r >= RAND_THRESHOLD {
                b[y][x] = true;
            }
        }
    }

    b

}

fn main() {

    let mut window: PistonWindow = WindowSettings::new("Game of Life - rust edisön", [WINDOW_WH, WINDOW_WH])
        .exit_on_esc(true)
        .build()
        .unwrap();

    let cell_sz = WINDOW_WH as usize / BOARD_WH;

    let sleep_dur = Duration::from_millis(SLEEP_DUR);

    let mut board = random_board();

    /* Glider code
    board[1][2] = true;
    board[2][3] = true;
    board[3][1] = true;
    board[3][2] = true;
    board[3][3] = true;
    */

    while let Some(event) = window.next() {
        window.draw_2d(&event, |ctx, gfx, _| {

            clear([1., 1., 1., 1.,], gfx);

            for b in 1..BOARD_WH {
                line([0.5, 0.5, 0.5, 1.], 1., [0., (b * cell_sz) as f64, WINDOW_WH as f64, (b * cell_sz) as f64], ctx.transform, gfx);
                line([0.5, 0.5, 0.5, 1.], 1., [(b * cell_sz) as f64, 0., (b * cell_sz) as f64, WINDOW_WH as f64], ctx.transform, gfx);
            }

            for y in 1..BOARD_WH {
                for x in 1..BOARD_WH {
                    if board[y][x] {
                        rectangle([0., 0., 0., 1.], [(x * cell_sz + 1) as f64, (y * cell_sz + 1) as f64, (cell_sz - 1) as f64, (cell_sz - 1) as f64], ctx.transform, gfx);
                    }
                }
            }

        });
        update(&mut board);
        thread::sleep(sleep_dur);
    }
}
