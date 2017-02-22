extern crate rand;

use std::thread;
use std::time;
use rand::thread_rng;
use rand::Rng;

const max: u32 = 128;

#[derive(Default,Clone)]
struct Grid {
    grid: [[u32; 32]; 32],
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
        let sources = vec![(2, 4), (10, 12), (9, 15)];
        for x in 0..32 {
            for y in 0..32 {
                for &(sx, sy) in sources.iter() {
                    let dx = sx - x as i32;
                    let dy = sy - y as i32;
                    let distance = dx * dx + dy * dy;
                    if distance > 0 {
                        self.grid[x][y] += 10 / distance as u32;
                        if self.grid[x][y] > max {
                            self.grid[x][y] = 0;
                        }
                    }
                }
            }
        }
        for x in 0..32 {
            for y in 0..32 {
                let mut v = self.grid[x][y];
                for i in [-1, 0, 1].iter() {
                    for j in [-1, 0, 1].iter() {
                        let x = x as isize + i;
                        let y = y as isize + j;
                        if x < 0 || x >= 32 || y < 0 || y >= 32 {
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
                let x = format!("{}", out(*cell as u32));
                s.push_str(&x);
            }
            s.push('\n');
        }
        s
    }
}

fn out(i: u32) -> char {
    let norm = if i > max { max } else { i };
    let alpha: Vec<char> = vec![' ', '.', ',', '\'', ':', ';', '-', '+', 'o', 'O', '#', '$', '@'];
    let index = (norm * (alpha.len() as u32 - 1) / max) as usize;
    alpha[index]
}

fn main() {
    // let alpha: Vec<char> =
    // r##"$@B%8&WM#*oahkbdpqwmZO0QLCJUYXzcvunxrjft/\|()1{}[]?-_+~<>i!lI;:,"^`'. '"##
    // to_string()
    // as_mut_vec();
    let interval = time::Duration::from_millis(100);
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
