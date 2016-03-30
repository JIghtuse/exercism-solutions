#include "difference_of_squares.h"

int squares::square_of_sums(int n)
{
    auto sum = n * (n + 1) / 2;
    return sum * sum;
}

int squares::sum_of_squares(int n)
{
    return (2 * n * n + 3 * n + 1) * n / 6;
}

int squares::difference(int n)
{
    return square_of_sums(n) - sum_of_squares(n);
}
