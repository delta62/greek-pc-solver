mod printer;
mod puzzle;
mod solver;

use printer::Printer;
use puzzle::Puzzle;
use solver::Solver;

fn main() {
    let puzzle = Puzzle::new();
    let mut solver = Solver::new();

    let cells = solver
        .find(|rot| puzzle.is_solution(*rot))
        .map(|rot| puzzle.cells(&rot))
        .unwrap();

    let printer = Printer::new();
    printer.print(cells);
}
