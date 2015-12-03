pub fn is_leap_year(year: i32) -> bool {
    let year_divisible_by = |factor: i32| year % factor == 0;

    year_divisible_by(4) && !year_divisible_by(100) || year_divisible_by(400)
}
