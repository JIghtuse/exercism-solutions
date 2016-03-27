#include "triangle.h"
#include <stdexcept>

namespace triangle {

TriangleKind kind(double a, double b, double c)
{
    if (a <= 0 || b <= 0 || c <= 0) {
        throw std::domain_error("non-positive side length");
    }
    if (a + b <= c || b + c <= a || a + c <= b)  {
        throw std::domain_error("triangle inequality violated");
    }

    if (a == b && b == c) {
        return equilateral;
    } else if (a == b || b == c || a == c) {
        return isosceles;
    } else {
        return scalene;
    }
}

} // namespace triangle
