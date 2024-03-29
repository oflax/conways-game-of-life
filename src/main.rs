#[macro_use]
extern crate crossterm;

use crossterm::cursor;
use crossterm::event::{read, Event, KeyCode, KeyEvent};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType};

use colored::Colorize;

use std::io;
use std::process;
use std::io::Write;

fn main() {
    let mut gr_size = String::new().to_string();
    let mut stdout = io::stdout();
    execute!(stdout, Clear(ClearType::All), cursor::MoveTo(0, 0)).unwrap();

    io::stdin().read_line(&mut gr_size);

    gr_size.pop();
    
    let coor = gr_size.split("x").collect::<Vec<&str>>();
    
    let x = coor[0].parse().unwrap();
    let y = coor[1].parse().unwrap();
    
    let mut vec: Vec<Vec<i8>> = Vec::new();
    let mut xdata: Vec<i8> = Vec::new();
    
    for _i in 0..y {
        for _j in 0..x {
            xdata.push(0);
        }
        vec.push(xdata.clone());
        xdata.clear();
    }
    
    let mut posx = 0;
    let mut posy = 0;
    
    let mut gen = 0;

    execute!(stdout, Clear(ClearType::All), cursor::MoveTo(0, 0)).unwrap();
    display(vec.clone(), 0, 0);
    println!("\nNesil: {}", gen);

    loop {
        enable_raw_mode().unwrap();
        execute!(stdout, cursor::MoveTo(0,0)).unwrap();
        execute!(stdout, cursor::Hide).unwrap();

        match read().unwrap() {
            //Event::Key(event) => println!("{:?}", event),
            Event::Key(KeyEvent {
                code: KeyCode::Right,
                ..
            }) => if posx < vec.len()-1 { posx+=1 },
            Event::Key(KeyEvent {
                code: KeyCode::Left,
                ..
            }) => if posx > 0 { posx-=1 },
            Event::Key(KeyEvent {
                code: KeyCode::Up,
                ..
            }) => if posy > 0 { posy-=1 },
            Event::Key(KeyEvent {
                code: KeyCode::Down,
                ..
            }) => if posy < vec.len()-1 { posy+=1 },
            Event::Key(KeyEvent {
                code: KeyCode::Char('a'),
                ..
            }) => if vec[posy][posx] == 0 { vec[posy][posx] = 1 } else { vec[posy][posx] = 0 },
            Event::Key(KeyEvent {
                code: KeyCode::Char(' '),
                ..
            }) => {
                    execute!(stdout, Clear(ClearType::All), cursor::Hide).unwrap();
                    disable_raw_mode().unwrap();
                    vec = simulate(vec);
                    gen += 1
            },
            Event::Key(KeyEvent {
                code: KeyCode::Char('q'),
                ..
            }) => process::exit(1),
            _ => (),
        }
        disable_raw_mode().unwrap();
        display(vec.clone(), posx, posy);
        println!("\nNesil: {}", gen);
    }
}

fn simulate(vec: Vec<Vec<i8>>) -> Vec<Vec<i8>> {
    let mut next = vec.clone();
    for (i, row) in vec.iter().enumerate() {
        for (j, node) in row.iter().enumerate() {
            let (mut u, mut ul, mut ur, mut l, mut r, mut b, mut bl, mut br) =
                (0, 0, 0, 0, 0, 0, 0, 0);
            if i > 0 {
                u = vec[i - 1][j];
                if j > 0 {
                    ul = vec[i - 1][j - 1]
                }
                if j < row.len() - 1 {
                    ur = vec[i - 1][j + 1]
                }
            }
            if i < vec.len() - 1 {
                b = vec[i + 1][j];
                if j > 0 {
                    bl = vec[i + 1][j - 1]
                }
                if j < row.len() - 1 {
                    br = vec[i + 1][j + 1]
                }
            }
            if j > 0 {
                l = vec[i][j - 1]
            }
            if j < row.len() - 1 {
                r = vec[i][j + 1]
            }
            let neighbours = u + ul + ur + l + r + b + bl + br;

            let node: i8 = *node;
            if (node == 1 && (neighbours == 2 || neighbours == 3)) || node == 0 && neighbours == 3  {
                next[i][j] = 1;
            } else {
                next[i][j] = 0;
            }
        }
    }
    next
}

fn display(vec: Vec<Vec<i8>>, px: usize, py: usize) {
    for (y, row) in vec.iter().enumerate() {
        for (x, item) in row.iter().enumerate() {
            let sym = if x == px && y == py {
                // item.to_string().yellow()
                if *item == 1 { "⬢".yellow() } else { "⬡".yellow() }
            } else if *item == 1 {
                "⬢".blue()
            } else {
                "⬡".red()
            };
            print!("{} ", sym);
        }
        println!();
    }
}
