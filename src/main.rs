struct Cell {
    value: u32,
    isFixed: bool,
}

impl Cell {
    fn new(value: u32, isFixed: bool) -> Self {
        Cell { value: value, isFixed: isFixed }
    }
}

struct Board {
    cells: Vec<Cell>,
    length: usize,
}

impl Board {
    fn new(puzzle: Vec<u32>, length: usize) -> Self {
        let mut cells = vec![];
        for value in puzzle {
            let isFixed = match value {
                0 => true,
                _ => false,
            };
            let cell = Cell::new(value, isFixed);
            cells.push(cell);
        }
        Board { cells: cells, length: length }
    }

    fn display(&mut self) {
        for i in 0..self.length {
            if i % 3 == 0 {
                println!("+---------+---------+---------+");
            }
            for j in 0..self.length {
                if j % 3 == 0 {
                    print!("|");
                }
                let cell = self.cells[i*self.length + j].value;
                if cell == 0 {
                    print!(" . ")
                }
                else {
                    print!(" {} ", cell);
                }
            }
            println!("|");
        }
        println!("+---------+---------+---------+");
    }
}

fn main() {
    let puzzle = vec![
        0, 0, 0, 0, 0, 0, 0, 1, 0,
        4, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 2, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 5, 0, 4, 0, 7,
        0, 0, 8, 0, 0, 0, 3, 0, 0,
        0, 0, 1, 0, 9, 0, 0, 0, 0,
        3, 0, 0, 4, 0, 0, 2, 0, 0,
        0, 5, 0, 1, 0, 0, 0, 0, 0,
        0, 0, 0, 8, 0, 6, 0, 0, 0,
    ];

    let mut board = Board::new(puzzle, 9);
    board.display();
}
