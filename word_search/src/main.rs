use std::io::{self,  BufRead};
use rand::seq::SliceRandom;
use rand::Rng;
use std::collections::HashMap;


struct Words {
    words: Vec<String>,
    map: HashMap<char, Vec<usize>>,
}

impl Words {
    fn load(mut all_words: Vec<String>) -> Words {
        all_words.sort_by(|a, b| a.len().cmp(&b.len()));
        let mut map: HashMap<char, Vec<usize>> = HashMap::new();
        for (index, word) in all_words.iter().enumerate() {
            for character in word.chars() {
                map.entry(character)
                    .and_modify(|v| v.push(index))
                    .or_insert(vec![index]);
            }
        }
        Words { words: all_words, map }
    }
}


fn map(m: char) -> u8 {
    match m {
        'a'..='z' => (m as u8) - ('a' as u8),
        _ => panic!("Invalid character"),
    }
}

const GRID_SIZE: usize = 64;

struct Grid {
    grid: Vec<Vec<char>>,
    dir: Orientation,
}

struct Matrix {
    storage: Vec<u8>,
    x: usize,
    y: usize,
}

// convolve
fn convolve(m: &Matrix, v: &Vec<u8>) -> Vec<u8> {
    let mut result = vec![0; m.storage.len() - v.len() + 1];
    for s in 0..result.len() {
        let mslice =  &m.storage[s..s+v.len()];
        let it_fits = mslice.iter().zip(v.iter()).all(|(a, b)| 
           *a == *b || (*a == 0 || *b == 0)
        );
    }
    result
}


fn main() {
    println!("Hello, world!");
    let initial_grid = vec![vec!['.'; GRID_SIZE]; GRID_SIZE]; // Initial empty grid
    let mut grid_stack = vec![Grid{grid: initial_grid, dir: Orientation::None}]; // Stack of grids starts with the initial grid
    let mut words = read_and_clean_words();
    let table = Words::load(words.clone());
    print!("{:?}",table.map.get(&'A'));
    words.sort_by(|a, b| b.len().cmp(&a.len()));
    let words = words;
    place_words(&mut grid_stack, &words, 0);
    let mut g = grid_stack.last().unwrap().grid.clone();
    replace_dots_with_random_letters(&mut g);
    print_grid(&g);
}

#[derive(PartialEq)]
#[derive(Clone)]
enum Orientation {
    Horizontal,
    Vertical,
    None,
}


fn replace_dots_with_random_letters(grid: &mut Vec<Vec<char>>) {
    let mut rng = rand::thread_rng();
    for row in grid.iter_mut() {
        for cell in row.iter_mut() {
            if *cell == '.' {
                *cell = (rng.gen::<u8>() % 26 + 65) as char;
            }
        }
    }
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
                    grid_stack.push(Grid{grid: grid_attempt, dir: orientation.clone()}); // Push the successful attempt onto the stack
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
        let mut cleaned_line = line.replace(" ", ""); // Remove spaces from the line
        cleaned_line.make_ascii_uppercase(); // Convert to lowercase
        words.push(cleaned_line);
    }

    words
}

