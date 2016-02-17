#![feature(plugin)]
#![plugin(clippy)]

pub fn disassemble(s: &str) -> Vec<u32> {
    s.chars()
     .map(|c: char| {
         match c {
             '*' => 99,
             ' ' => 0,
             x => x.to_digit(10).unwrap(),
         }
     })
     .collect()
}

pub fn assemble(v: Vec<u32>) -> String {
    v.iter()
     .map(|i: &u32| {
         match *i {
             99 => '*',
             0 => ' ',
             x => (x as u8 + b'0') as char,
         }
     })
     .collect::<String>()
}

pub fn annotate(field: &[&str]) -> Vec<String> {
    let mut res = vec![];
    for line in field {
        let mut mutline = disassemble(line);
        let line = disassemble(line);
        for (i, c) in line.iter().enumerate() {
            if *c == 99 {
                mutline[i - 1] += 1;
                mutline[i + 1] += 1;
            }
        }
        res.push(assemble(mutline));
    }
    res
}

#[cfg(test)]
mod test {
    use super::{assemble, disassemble};
    #[test]
    fn disassemble_works() {
        let empty: Vec<u32> = vec![];
        assert_eq!(empty, disassemble(""));

        let no_mines: Vec<u32> = vec![0, 0, 0];
        assert_eq!(no_mines, disassemble("   "));

        let one_mine: Vec<u32> = vec![0, 99, 0];
        assert_eq!(one_mine, disassemble(" * "));

        let two_mines: Vec<u32> = vec![99, 0, 99];
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
