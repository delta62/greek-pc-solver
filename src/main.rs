mod puzzle;
mod solver;

use solver::Solver;
use puzzle::Puzzle;

fn main() {
    let puzzle = Puzzle::new();
    let solver = Solver::new();
    let mut tries = 0;

    for rotation in solver {
        tries += 1;
        if puzzle.is_solution(rotation) {
            println!("Solution found: {:?}", rotation);
            puzzle.print(&rotation);
            break;
        }
    }

    println!("Tried {} combinations", tries);
}

