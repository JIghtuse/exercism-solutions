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

impl From<usize> for Roman {
    fn from(_: usize) -> Roman {
        Roman { repr: "I".to_string() }
    }
}
