#include "grains.h"
#include <algorithm>
#include <array>
#include <cmath>

unsigned long long grains::square(unsigned long long n)
{
    return std::pow(2, n - 1);
}

unsigned long long grains::total()
{
    constexpr auto nSquares{ 64 };
    auto squares{ std::array<unsigned long long, nSquares>{} };

    auto i{ 0 };
    std::generate(squares.begin(), squares.end(), [&i] { return square(++i); });
    return std::accumulate(squares.begin(), squares.end(), 0);
}
