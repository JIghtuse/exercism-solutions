pub struct Queen {
    x: i8,
    y: i8,
}

impl Queen {
    pub fn new((x, y): (i8, i8)) -> Result<Queen, &'static str> {
        let incorrect_coordinate = |c: i8| c < 0 || c > 7;
        if incorrect_coordinate(x) || incorrect_coordinate(y) {
            Err("Incorrect coordinates")
        } else {
            Ok(Queen { x: x, y: y })
        }
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        self.x == other.x ||
        self.y == other.y ||
        (self.x - other.x).abs() == (self.y - other.y).abs()
    }
}
