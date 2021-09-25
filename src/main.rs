mod lib;
use lib::*;

fn main() {
    let puzzle: Board = make_puzzle();
    let solution = solvepuzzle(puzzle.clone());
    let difficulty = ratepuzzle(&puzzle, 4);
    println!("{:?}, {:?}, {:?}", puzzle, solution, difficulty);
}