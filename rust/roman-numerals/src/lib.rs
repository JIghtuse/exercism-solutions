#[derive(PartialEq, Debug)]
pub struct Roman
{
    repr: String,
}

impl<'a> PartialEq<Roman> for &'a str {
    fn eq(&self, roman: &Roman) -> bool {
        self == &roman.repr
    }
    fn ne(&self, roman: &Roman) -> bool {
        self != &roman.repr
    }
}

static ARABIC_ROMAN : [(i32, &'static str); 13] = [
    (1000, "M"),
    (900, "CM"),
    (500, "D"),
    (400, "CD"),
    (100, "C"),
    (90, "XC"),
    (50, "L"),
    (40, "XL"),
    (10, "X"),
    (9, "IX"),
    (5, "V"),
    (4, "IV"),
    (1, "I"),
];

impl From<i32> for Roman {
    fn from(n: i32) -> Roman {
        let mut n = n;
        let mut repr = String::new();

        for &(i, roman_literal) in ARABIC_ROMAN.into_iter() {
            while n >= i {
                n -= i;
                repr.push_str(roman_literal);
            }
        }
        Roman { repr: repr }
    }
}
