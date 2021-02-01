use colorful::{Color, Colorful};
use super::puzzle::Cell;

pub struct Printer;

impl Printer {
    pub fn new() -> Self {
        Self
    }

    pub fn print(&self, cells: Vec<Vec<Cell>>) {
        self.print_key();

        println!();

        for row in cells {
            for cell in row {
                self.print_cell(cell);
            }
            println!();
        }
    }

    fn print_cell(&self, cell: Cell) {
        let s = format!("{:2}", cell.1);

        match cell.0 {
            0 => print!("{}", s.color(Color::Black).bg_color(Color::White)),
            1 => print!("{}", s.color(Color::Black).bg_color(Color::Green)),
            2 => print!("{}", s.color(Color::Black).bg_color(Color::Blue)),
            3 => print!("{}", s.color(Color::Black).bg_color(Color::Cyan)),
            4 => print!("{}", s.color(Color::White).bg_color(Color::Red)),
            _ => unreachable!(),
        }

        print!(" ");
    }

    fn print_key(&self) {
        println!("{}         top gear", "   ".bg_color(Color::White));
        println!("{}       2nd gear", "     ".bg_color(Color::Green));
        println!("{}     3rd gear", "       ".bg_color(Color::Blue));
        println!("{}   bottom gear", "         ".bg_color(Color::Cyan));
        println!("{} base", "           ".bg_color(Color::Red));
    }
}
