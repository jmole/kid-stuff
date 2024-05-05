use std::fmt::Result;
use std::io::{self,  BufRead};
use rand::seq::SliceRandom;
use rand::Rng;
use std::collections::HashMap;
use grid::*;
use std::assert_eq;
use bitflags::bitflags;




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

trait Maximizer<T: PartialOrd> {
    fn max_list(&self) -> (Vec<(usize,usize)>, T);
} 

impl Maximizer<f32> for Grid<f32> {
    fn max_list(&self) -> (Vec<(usize,usize)>, f32) {
        let mut list: Vec<(usize, usize)> = Vec::new();
        let mut max = 0.0;
        for ((row, col), val) in self.indexed_iter() {
            if list.len() == 0 {
                list.push((row,col));
                max = *val;
            } else if *val > max {
                list.clear();
                list.push((row,col));
                max = *val;
            } else if *val == max {
                list.push((row,col));
            }
        }
        if max == 0.0 {
            return (vec![], 0.0);
        }
        return (list, max);
    }
}


fn map(m: char) -> u8 {
    match m {
        'a'..='z' => (m as u8) - ('a' as u8),
        _ => panic!("Invalid character"),
    }
}

const GRID_SIZE: usize = 48;
const EMPTY: char = '.';
const VALID_DIRS: [Direction; 4] = [Direction::EE, Direction::SS, Direction::SE, Direction::NE];


bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    struct CharDirection: u8 {
        const NONE = 0;
        const N = 0b1;
        const NE = 0b10;
        const E = 0b100;
        const SE = 0b1000;
    }
}


impl Default for CharDirection {
    fn default() -> Self {
        CharDirection::NONE
    }
}

#[derive(Clone)]
#[derive(PartialEq)]
#[derive(Debug)]
#[derive(Copy)]
struct Character {
    letter: char,
    directions: CharDirection,
}

impl std::fmt::Display for Character {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.letter)
    }
}

impl Character {
    fn is_empty(&self) -> bool {
        return self.letter == EMPTY;
    }
    fn combine_with(&mut self, other: &Character) {
        if self.is_empty() {
            self.letter = other.letter;
        } else {
            assert_eq!(self.letter, other.letter);
        }
        self.directions |= other.directions;
    }
}

impl Default for Character {
    fn default() -> Self {
        Character {
            letter: EMPTY,
            directions: CharDirection::default(),
        }
    }
}

impl From<char> for Character {
    fn from(value: char) -> Self {
        Character {
            letter: value,
            directions: CharDirection::NONE,
        }
    }
}

fn gg(char_grid: Grid<char>) -> Grid<Character> {
    ggd(char_grid, CharDirection::default())
}

fn ggd(char_grid: Grid<char>, direction: CharDirection) -> Grid<Character> {
    print!("gg");
    let mut g = Grid::init(char_grid.rows(), char_grid.cols(), Character::default());
    for ((row,col), val) in char_grid.indexed_iter() {
        if *val != EMPTY {
            g[(row,col)].letter = *val;
            g[(row,col)].directions = direction;
        }
        
    }
    return g;
}

#[derive(Clone)]
struct Board {
    grid: Grid<Character>,
    dir: Orientation,
}




/** "Convolves" a word with a grid.

If the word can fit in the grid at a certain position, the corresponding position in the grid gets a score >= 1.
If the word can't fit in the grid at the specified position, the position is scored 0.

TODO: implement letter-frequency into scoring, like scrabble.
*/ 
fn convolve(grid: &Grid<Character>, word: &Grid<Character>) -> Grid<f32> {
    if word.cols() > grid.cols() || word.rows() > grid.rows() {
        return grid![[]];
    }
    let out_rows = grid.rows() - word.rows() + 1;
    let out_cols = grid.cols() - word.cols() + 1;
    let mut out: Grid<f32> = Grid::new(out_rows, out_cols);
    for ((row, col), score) in out.indexed_iter_mut() {
        *score = 1.0;
        // Iterate through word and compare each letter to the corresponding letter
        // on the grid, if it were placed at row,col
        for ((r, c), letter) in word.indexed_iter() {
            if letter.letter == EMPTY {
                continue;
            }
            if let Some(value) = grid.get(r+row,c+col) {
                if value.letter == EMPTY {
                    *score = *score * 1.0;
                } else if value.letter == letter.letter && !value.directions.intersects(letter.directions) {
                    // TODO: add letter frequency scor,ing
                    *score = *score * 2.0;
                } else {
                    *score = 0.0;
                }
            }
        }
    }
    return out;
}


fn main() {
    println!("Hello, world!");
    let initial_grid = Grid::init(GRID_SIZE, GRID_SIZE, Character::default()); // Initial empty grid
    let mut grid_stack = vec![Board{grid: initial_grid, dir: Orientation::None}]; // Stack of grids starts with the initial grid
    let mut words = read_and_clean_words();
    let table = Words::load(words.clone());
    words.sort_by(|a, b| b.len().cmp(&a.len()));
    let words = words;
    place_words_backtrack_convolution(&mut grid_stack, &words, 0);
    let mut g = grid_stack.last().unwrap().grid.clone();
    //replace_dots_with_random_letters(&mut g);
    println!("\ngrid size: {}", grid_stack.len());
    print_grid(&g);
                   
}

#[derive(PartialEq)]
#[derive(Clone)]
enum Orientation {
    Horizontal,
    Vertical,
    None,
}

#[derive(PartialEq)]
#[derive(Clone)]
enum Direction {
    EE,
    NE,
    NN,
    NW,
    WW,
    SW,
    SS,
    SE,
}

#[derive(PartialEq)]
#[derive(Clone)]
struct Candidate {
    word: String,
    dir: Direction,
    as_grid: Grid<Character>,
    placements: Grid<f32>,
    max_placements: Vec<(usize, usize)>,
    max_placement_value: f32,
}

fn to_grid(word: &String, dir: Direction) -> Grid<Character> {
    use Direction::*;
    let cols = match dir {
        NN | SS => 1,
        _ => word.len()
    };
    let rows = match dir {
        EE | WW => 1,
        _ => word.len()
    };
    // row increment, col increment, row start, col start, char direction
    let L = word.len() as isize - 1;
    let (ri, ci, rs, cs, char_d) = match dir {
        EE => ( 0,  1, 0, 0, CharDirection::E),
        WW => ( 0, -1, 0, L, CharDirection::E ),
        NN => (-1,  0, L, 0, CharDirection::N),
        SS => ( 1,  0, 0, 0, CharDirection::N),
        SE => ( 1,  1, 0, 0, CharDirection::SE),
        SW => ( 1, -1, 0, L, CharDirection::NE),
        NE => (-1,  1, L, 0, CharDirection::NE),
        NW => (-1, -1, L, L, CharDirection::SE),
    };
    let mut g = Grid::init(rows,cols, Character::default() );
    let mut r = rs;
    let mut c = cs;
    for letter in word.chars() {
        g[(r as usize,c as usize)] = Character {letter, directions: CharDirection::from(char_d)};
        r = r + ri;
        c = c + ci;
    }
    return g;
}

impl Candidate {
    fn create(grid: &Grid<Character>, word: &String, valid_directions: &[Direction]) -> Vec<Candidate>{
        let mut candidates: Vec<Candidate> = vec![];
        for dir in valid_directions {
            let word_grid = to_grid(word, dir.clone());
            let placements = convolve(grid, &word_grid);
            let (mut max_list, max_val) = placements.max_list();
            max_list.shuffle(&mut rand::thread_rng());
            if max_val > 0.0 {
                candidates.push(Candidate {
                    word: word.clone(),
                    dir: dir.clone(),
                    as_grid: word_grid,
                    placements: placements,
                    max_placement_value: max_val,
                    max_placements: max_list,
                });
            }
        }
        candidates
    }
}

fn combine(grid: &mut Grid<Character>, word: &Grid<Character>, row: usize, column: usize) {
    for ((r,c),val) in grid.indexed_iter_mut() {
        if r >= row && c >= column && c-column < word.cols() &&  r-row < word.rows() {
            let letter = word[(r-row,c-column)];
            if !letter.is_empty() {
                val.combine_with(&letter);
            }
        }
    }
}



fn place_words_backtrack_convolution(grid_stack: &mut Vec<Board>, words: &Vec<String>, index: usize) -> bool {
    if index == words.len() {
        return true; // All words placed
    }

    let word = &words[index];
    let last = grid_stack.last().unwrap().clone();
    
    let mut candidates = Candidate::create(&last.grid, word, &VALID_DIRS);
    if candidates.is_empty() {
        return false;
    }
    candidates.sort_by(|a,b| return b.max_placement_value.partial_cmp(&a.max_placement_value).unwrap() );
    for candidate in candidates {
        for placement in candidate.max_placements {
            let mut current_grid  = last.grid.clone(); 
            combine(&mut current_grid, &candidate.as_grid, placement.0, placement.1);
            grid_stack.push(Board { grid: current_grid.clone(), dir: Orientation::None });
            if place_words_backtrack_convolution(grid_stack, words, index+1) {
                return true;
            }
            grid_stack.pop();
        }
    }
    println!("gridstack size {}", grid_stack.len());
    return true;
}


fn replace_dots_with_random_letters(grid: &mut Grid<char>) {
    let mut rng = rand::thread_rng();
    for ((row,col), val) in grid.indexed_iter_mut() {
            if *val == EMPTY {
                *val = (rng.gen::<u8>() % 26 + 65) as char;
            }
    }
}

fn try_place_word(grid: &Grid<Character>, word: &str, row: usize, col: usize, orientation: &Orientation) -> bool {
    print!("{} {} {}", word, row, col);
    match orientation {
        Orientation::Horizontal => {
            if col + word.len() > grid.cols() { return false; }
            for (i, c) in word.chars().enumerate() {
                if !grid[(row,col + i)].is_empty() && grid[(row,col + i)].letter != c {
                    return false; // Clash with already placed word
                }
            }
        },
        Orientation::Vertical => {
            if row + word.len() > grid.rows() { return false; }
            for (i, c) in word.chars().enumerate() {
                if !grid[(row + i,col)].is_empty() && grid[(row + i,col)].letter != c {
                    return false; // Clash with already placed word
                }
            }
        },
        Orientation::None => {},
    }
    true
}

fn place_words_backtrack(grid_stack: &mut Vec<Board>, words: &Vec<String>, index: usize) -> bool {
    if index == words.len() {
        return true; // All words placed
    }

    let word = &words[index];
    let last = grid_stack.last().unwrap();
    let current_grid  = last.grid.clone(); // Work with the latest grid state
    let current_orientation = last.dir.clone(); // Work with the latest orientation state
    let mut rnd: Vec<usize> = (0..(current_grid.rows())).collect();
    rnd.shuffle(&mut rand::thread_rng());

    let orients = if current_orientation != Orientation::Horizontal {
        [Orientation::Vertical, Orientation::Horizontal]
    } else {
        [Orientation::Horizontal, Orientation::Vertical]
    };
    for orientation in orients {
    // fill each row of random grid with numbers between 0 and len()-1, using each number exactly once
        for row in 0..current_grid.rows() {
            for col in 0..current_grid.cols() {
            
                if try_place_word(&current_grid, word, rnd[row], rnd[col], &orientation) {
                    let mut grid_attempt = current_grid.clone(); // Clone the current grid for this attempt
                    match orientation {
                        Orientation::Horizontal => {
                            for (i, c) in word.chars().enumerate() {
                                grid_attempt[(rnd[row],rnd[col] + i)] = Character::from(c);
                            }
                        },
                        Orientation::Vertical => {
                            for (i, c) in word.chars().enumerate() {
                                grid_attempt[(rnd[row] + i, rnd[col])] = Character::from(c);
                            }
                        },
                        Orientation::None => {},
                    }
                    grid_stack.push(Board{grid: grid_attempt, dir: orientation.clone()}); // Push the successful attempt onto the stack
                    if place_words_backtrack(grid_stack, words, index + 1) {
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

fn print_grid<T: std::fmt::Display + Copy>(grid: &Grid<T>) {
    for row in grid.iter_rows() {
        for &cell in row {
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



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convolve() {
        let g = gg(grid![  
            [' ',' ','a','y']
            [' ','b',' ',' ']
            [' ',' ',' ','z']
            [' ',' ',' ',' ']
        ]);
        let w1 = gg(grid![['a', 'b', 'c', 'd']]);
        let w2 = gg(grid![['a'] ['b'] ['c'] ['d']]);
        let mut z1 = convolve(&g, &w1);
        let mut z2 = convolve(&g, &w2);

        assert!(z1 == grid![[0.0] [2.0] [0.0] [1.0]]);
        assert!(z2 == grid![[1.0, 2.0, 2.0, 0.0]]);
    }

    #[test]
    fn test_convolve_2() {
        let g = gg(grid![  
            [' ',' ','a','y']
            [' ','b',' ',' ']
            [' ',' ',' ','z']
            ['a',' ',' ','a']
            ['a',' ',' ','b']
            ['a',' ',' ',' ']
            ['z',' ',' ',' ']
        ]);
        let w1: Grid<Character> = gg(grid![['a', 'b', 'c', 'd']]);
        let w2 = gg(grid![['a'] ['b'] ['c'] ['d']]);
        let z1 = convolve(&g, &w1);
        let z2 = convolve(&g, &w2);

        assert!(z1 == grid![[0.0] [2.0] [0.0] [0.0] [0.0] [2.0] [0.0]]);
        assert!(z2 == grid![
            [0.0, 2.0, 2.0, 0.0]
            [0.0, 0.0, 1.0, 0.0]
            [0.0, 1.0, 1.0, 0.0]
            [0.0, 1.0, 1.0, 4.0]
        ]);
    }

    #[test]
    fn test_to_grid() {
        let s = "hello".to_string();
        let ge = to_grid(&s, Direction::EE);
        let gs = to_grid(&s, Direction::SS);
        let gne = to_grid(&s, Direction::NE);
        let gse = to_grid(&s, Direction::SE);
        let EEE: char = EMPTY;  
        print!("{:?}", ge);
        assert_eq!(ge, ggd(grid![['h','e','l','l','o']], CharDirection::E));
        assert_eq!(gs, ggd(grid![['h'] ['e'] ['l'] ['l'] ['o']], CharDirection::N));
        assert_eq!(gne, ggd(grid![
            [EEE,EEE,EEE,EEE,'o']
            [EEE,EEE,EEE,'l',EEE]
            [EEE,EEE,'l',EEE,EEE]
            [EEE,'e',EEE,EEE,EEE]
            ['h',EEE,EEE,EEE,EEE]
            ], CharDirection::NE));
        assert_eq!(gse, ggd(grid![
            ['h',EEE,EEE,EEE,EEE]
            [EEE,'e',EEE,EEE,EEE]
            [EEE,EEE,'l',EEE,EEE]
            [EEE,EEE,EEE,'l',EEE]
            [EEE,EEE,EEE,EEE,'o']
            ], CharDirection::SE));
    }

}