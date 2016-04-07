pub struct Queen {
    x: i8,
    y: i8,
}

impl Queen {
    pub fn new((x, y): (i8, i8)) -> Result<Queen, &'static str> {
        Ok(Queen { x: x, y: y })
    }

    pub fn can_attack(&self, _: &Queen) -> bool {
        false
    }
}
