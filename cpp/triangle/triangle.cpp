#include "triangle.h"

namespace triangle {

TriangleKind kind(unsigned a, unsigned b, unsigned c)
{
    if ((a == b) && (b == c)) {
        return equilateral;
    }
    throw "not reachable";
}

} // namespace triangle
