use std::io::{self,  BufRead};
use rand::seq::SliceRandom;

const GRID_SIZE: usize = 15;

struct Grid {
    grid: Vec<Vec<char>>,
    dir: Orientation,
}

fn main() {
    println!("Hello, world!");
    let initial_grid = vec![vec!['.'; GRID_SIZE]; GRID_SIZE]; // Initial empty grid
    let mut grid_stack = vec![Grid{grid: initial_grid, dir: Orientation::None}]; // Stack of grids starts with the initial grid
    let mut words = read_and_clean_words();
    words.sort_by(|a, b| b.len().cmp(&a.len()));
    let words = words;
    place_words(&mut grid_stack, &words, 0);
    let grid = grid_stack.last().unwrap();
    print_grid(&grid.grid);
}

#[derive(PartialEq)]
#[derive(Clone)]
enum Orientation {
    Horizontal,
    Vertical,
    None,
}

fn try_place_word(grid: &Vec<Vec<char>>, word: &str, row: usize, col: usize, orientation: &Orientation) -> bool {

    match orientation {
        Orientation::Horizontal => {
            if col + word.len() > grid[0].len() { return false; }
            for (i, c) in word.chars().enumerate() {
                if grid[row][col + i] != '.' && grid[row][col + i] != c {
                    return false; // Clash with already placed word
                }
            }
        },
        Orientation::Vertical => {
            if row + word.len() > grid.len() { return false; }
            for (i, c) in word.chars().enumerate() {
                if grid[row + i][col] != '.' && grid[row + i][col] != c {
                    return false; // Clash with already placed word
                }
            }
        },
        Orientation::None => {},
    }
    true
}

fn place_words(grid_stack: &mut Vec<Grid>, words: &Vec<String>, index: usize) -> bool {
    if index == words.len() {
        return true; // All words placed
    }

    let word = &words[index];
    let last = grid_stack.last().unwrap();
    let current_grid  = last.grid.clone(); // Work with the latest grid state
    let current_orientation = last.dir.clone(); // Work with the latest orientation state
    let mut rnd: Vec<usize> = (0..current_grid.len()).collect();
    rnd.shuffle(&mut rand::thread_rng());

    let orients = if current_orientation != Orientation::Horizontal {
        [Orientation::Vertical, Orientation::Horizontal]
    } else {
        [Orientation::Horizontal, Orientation::Vertical]
    };
    for orientation in orients {
    // fill each row of random grid with numbers between 0 and len()-1, using each number exactly once
        for row in 0..current_grid.len() {
            for col in 0..current_grid[0].len() {
            
                if try_place_word(&current_grid, word, rnd[row], rnd[col], &orientation) {
                    let mut grid_attempt: Vec<Vec<char>> = current_grid.clone(); // Clone the current grid for this attempt
                    match orientation {
                        Orientation::Horizontal => {
                            for (i, c) in word.chars().enumerate() {
                                grid_attempt[rnd[row]][rnd[col] + i] = c;
                            }
                        },
                        Orientation::Vertical => {
                            for (i, c) in word.chars().enumerate() {
                                grid_attempt[rnd[row] + i][rnd[col]] = c;
                            }
                        },
                        Orientation::None => {},
                    }
                    grid_stack.push(Grid{grid: grid_attempt, dir: orientation}); // Push the successful attempt onto the stack
                    if place_words(grid_stack, words, index + 1) {
                        return true; // Successfully placed all words
                    } else {
                        grid_stack.pop(); // Backtrack: Remove the last grid state
                    }
                }
            }
        }
    }
    false
}

fn print_grid(grid: &Vec<Vec<char>>) {
    for row in grid.iter() {
        for &cell in row.iter() {
            print!("{} ", cell);
        }
        println!();
    }
}

fn read_and_clean_words() -> Vec<String> {
    let stdin = io::stdin();
    let mut words: Vec<String> = Vec::new();

    println!("Enter words (Press ENTER on a blank line to finish):");

    for line in stdin.lock().lines() {
        let line = line.expect("Failed to read line");
        if line.is_empty() {
            break;
        }
        let cleaned_line = line.replace(" ", ""); // Remove spaces from the line
        words.push(cleaned_line);
    }

    words
}

