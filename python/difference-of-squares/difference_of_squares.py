def square_of_sum(n):
    sum_up_to_n = n * (n + 1) / 2
    return sum_up_to_n**2

def sum_of_squares(n):
    return (2 * n**2 + 3 * n + 1) * n / 6

def difference(n):
    return square_of_sum(n) - sum_of_squares(n)
