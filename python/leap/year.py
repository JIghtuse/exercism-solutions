def is_leap_year(year):
    def is_year_factor(n):
        return year % n == 0
    return is_year_factor(400) or is_year_factor(4) and not is_year_factor(100)
