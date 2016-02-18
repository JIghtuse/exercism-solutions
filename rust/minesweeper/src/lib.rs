#![feature(plugin)]
#![plugin(clippy)]

#[derive(Clone,Debug,PartialEq)]
pub enum Cell {
    Empty,
    Mine,
    Number(u32),
}

pub fn disassemble(s: &str) -> Vec<Cell> {
    s.chars()
     .map(|c: char| {
         match c {
             '*' => Cell::Mine,
             ' ' => Cell::Empty,
             x => Cell::Number(x.to_digit(10).unwrap()),
         }
     })
     .collect()
}

pub fn assemble(v: Vec<Cell>) -> String {
    v.iter()
     .map(|cell| {
         match *cell {
             Cell::Mine => '*',
             Cell::Empty => ' ',
             Cell::Number(x) => (x as u8 + b'0') as char,
         }
     })
     .collect()
}

pub fn annotate(field: &[&str]) -> Vec<String> {
    let mut res = vec![];
    for line in field {
        let mut mutline = disassemble(line);
        let line = mutline.clone();

        let neighbors_pos = |p| {
            let mut positions = vec![];
            if p + 1 < line.len() {
                positions.push(p + 1);
            }
            if p != 0 {
                positions.push(p - 1);
            }
            positions
        };

        for (current_pos, cell) in line.iter().enumerate() {
            if *cell == Cell::Mine {
                for pos in neighbors_pos(current_pos) {
                    match mutline[pos] {
                        Cell::Empty => mutline[pos] = Cell::Number(1),
                        Cell::Number(x) => mutline[pos] = Cell::Number(x + 1),
                        _ => (),
                    };
                }
            }
        }
        res.push(assemble(mutline));
    }
    res
}

#[cfg(test)]
mod test {
    use super::{Cell, assemble, disassemble};
    #[test]
    fn disassemble_works() {
        let empty: Vec<Cell> = vec![];
        assert_eq!(empty, disassemble(""));

        let no_mines: Vec<Cell> = vec![Cell::Empty, Cell::Empty, Cell::Empty];
        assert_eq!(no_mines, disassemble("   "));

        let one_mine: Vec<Cell> = vec![Cell::Empty, Cell::Mine, Cell::Empty];
        assert_eq!(one_mine, disassemble(" * "));

        let two_mines: Vec<Cell> = vec![Cell::Mine, Cell::Empty, Cell::Mine];
        assert_eq!(two_mines, disassemble("* *"));
    }

    fn str_is_restored(s: &str) {
        assert_eq!(s.to_owned(), assemble(disassemble(s)));
    }
    #[test]
    fn assemble_works() {
        str_is_restored("");
        str_is_restored("   ");
        str_is_restored(" * ");
        str_is_restored("* *");
    }
}
