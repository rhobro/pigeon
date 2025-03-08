use std::io;

mod words;
use itertools::{iproduct, Itertools};
use words::*;

type Grid = [[char; 4]; 4];
type Coord = (isize, isize);
type Stack = Vec<Coord>;
type Result = (String, Stack);

fn main() {
    // get input 
    let mut inp = String::with_capacity(16);
    io::stdin().read_line(&mut inp).expect("invalid input flat grid length");
    // inp = "eminaynhetdsuwri".into();
    let grid = parse_grid(&inp);
    let words = gen_words();

    let results: Vec<Result> = iproduct!(0..4, 0..4)
        // all starting points
        .map(|c| Board::new(&grid, c, &words))
        // search
        .map(|b| b.search())
        .flatten()
        // remove duplicates
        .unique()
        // sort for longest
        .sorted_by(|(a, _), (b, _)| Ord::cmp(&b.len(), &a.len()))
        .collect();

    let s = results.iter()
        .map(|(w, _)| w)
        .join("\n");

    println!("\n{s}");
}

struct Board<'a, 'b> {
    grid: &'a Grid,
    stack: Stack,
    old_dict: &'b Vec<String>,
}

impl<'a, 'b> Board<'a, 'b> {

    fn search(&self) -> Vec<(String, Stack)> {
        let dict = self.dict();
        if dict.len() == 0 {
            return vec![];
        }
        let (row, col) = self.stack.last().expect("needs starting point");

        let mut words: Vec<(String, Stack)> = [
            (row - 1, col - 1),
            (row - 1, *col),
            (row - 1, col + 1),
            (*row, col - 1),
            (*row, col + 1),
            (row + 1, col - 1),
            (row + 1, *col),
            (row + 1, col + 1),
        ].iter()
            // only valid grid coordinates
            .filter(|c| valid_coord(**c))
            // haven't been before
            .filter(|c| !self.stack.contains(c))
            .cloned()
            // new possibilities
            .map(|c| Board {
                grid: self.grid,
                stack: {
                    let mut new = self.stack.clone();
                    new.push(c);
                    new
                },
                old_dict: &dict,
            })
            .map(|b| b.search())
            .flatten()
            .collect();

        let w = self.word();
        if dict.contains(&w) {
            words.push((w.to_uppercase(), self.stack.clone()));
        }

        return words;

    }

    fn word(&self) -> String {
        self.stack.iter()
            .map(|(i, j)| self.grid[*i as usize][*j as usize])
            .collect()
    }

    fn dict(&self) -> Vec<String> {
        filter_down(self.old_dict, self.word())
    }

    fn new(grid: &'a [[char; 4]; 4], c: (isize, isize), old_dict: &'b Vec<String>) -> Board<'a, 'b> {
        Board {
            grid,
            stack: vec![c],
            old_dict,
        }
    }
}

fn parse_grid(raw: &str) -> [[char; 4]; 4] {
    let raw: Vec<char> = raw.trim().to_lowercase().chars().collect();
    let mut grid = [['a'; 4]; 4];

    for i in 0..4 {
        for j in 0..4 {
            grid[i][j] = raw[i * 4 + j];
        }
    }

    grid
}

fn valid_coord((x, y): (isize, isize)) -> bool {
    x >= 0 && x < 4 && y >= 0 && y < 4
}