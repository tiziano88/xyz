extern crate rand;
extern crate termion;

use std::thread;
use std::time;
use rand::thread_rng;
use rand::Rng;

const max: u32 = 128;

#[derive(Default,Clone)]
struct Grid {
    grid: [[u32; 32]; 32],
}

trait Modifier {
    fn modify(&mut self, &mut Grid);
}

struct Blob {
    x: i32,
    y: i32,
}

impl Modifier for Blob {
    fn modify(&mut self, grid: &mut Grid) {
        for x in 0..grid.grid.len() {
            for y in 0..grid.grid[x].len() {
                let dx = self.x - x as i32;
                let dy = self.y - y as i32;
                let distance = dx * dx + dy * dy;
                if distance > 0 {
                    grid.grid[x][y] += 10 / distance as u32;
                    if grid.grid[x][y] > max {
                        grid.grid[x][y] = 0;
                    }
                }
            }
        }
    }
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
        let mut modifiers = vec![
            Blob{x:1, y:2},
            Blob{x:15, y:10},
        ];
        for m in modifiers.iter_mut() {
            m.modify(self);
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
                let x = out(*cell as u32);
                s.push_str(&x);
            }
            s.push('\n');
        }
        s
    }
}

fn out(i: u32) -> String {
    let norm = if i > max { max } else { i };
    let alpha: Vec<char> = vec![' ', '.', ',', '\'', ':', ';', '-', '+', 'o', 'O', '#', '$', '@'];

    let blue = &termion::color::Blue;
    let green = &termion::color::Green;
    let yellow = &termion::color::Yellow;
    let red = &termion::color::Red;
    let colors: Vec<&termion::color::Color> = vec![blue, green, yellow, red];
    let alpha_index = (norm * (alpha.len() as u32 - 1) / max) as usize;
    let color_index = (norm * (colors.len() as u32 - 1) / max) as usize; // XXX
    format!("{}{}",
            termion::color::Fg(colors[color_index]),
            alpha[alpha_index])
}

fn main() {
    // let alpha: Vec<char> =
    // r##"$@B%8&WM#*oahkbdpqwmZO0QLCJUYXzcvunxrjft/\|()1{}[]?-_+~<>i!lI;:,"^`'. '"##
    // to_string()
    // as_mut_vec();
    let interval = time::Duration::from_millis(100);
    let mut rng = thread_rng();
    let mut grid = Grid::new();
    println!("{}", termion::clear::All);
    loop {
        thread::sleep(interval);
        let mut s = "".to_string();
        grid.step();
        println!("{}", termion::cursor::Hide);
        println!("{}", termion::cursor::Goto(1, 1));
        // println!("{}", termion::cursor::Goto(30, 30));
        println!("{}", grid.show());
        // println!("\n\n\n\n\n\n{}", s);
        println!("{}", termion::cursor::Show);
    }
}
