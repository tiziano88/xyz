extern crate rand;

use std::thread;
use std::time;
use rand::thread_rng;
use rand::Rng;

#[derive(Default,Clone)]
struct Grid {
    grid: [[i32; 32]; 32],
}

impl Grid {
    fn new() -> Grid {
        let mut g = Grid::default();
        g
    }
    fn step(&mut self) {
        for i in 0..32 {
            self.grid[31][i] += 1;
        }
        for x in 0..32 {
            for y in 0..32 {
                let mut v = self.grid[x][y];
                for i in [-1, 0, 1].iter() {
                    for j in [-1, 0, 1].iter() {
                        let x = x as isize + i;
                        let y = y as isize + j;
                        if x < 0 || x >= 32 || y < 0 || y > 32 {
                            continue;
                        }
                        v += self.grid[x as usize][y as usize];
                    }
                }
            }
        }
    }
    fn show(&self) -> String {
        let mut s = "".to_string();
        for row in self.grid.iter() {
            for cell in row.iter() {
                let x = format!("{}", cell);
                s.push_str(&x);
            }
            s.push('\n');
        }
        s
    }
}

fn main() {
    let alpha: Vec<char> = vec!['O', 'o', 'X', 'x', '-', '_', '#'];
    let interval = time::Duration::from_millis(1_000);
    let mut rng = thread_rng();
    let mut grid = Grid::new();
    loop {
        thread::sleep(interval);
        let mut s = "".to_string();
        grid.step();
        println!("{}", grid.show());
        // println!("\n\n\n\n\n\n{}", s);
    }
}
