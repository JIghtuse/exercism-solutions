#pragma once

namespace triangle {

enum TriangleKind {
    equilateral,
    isosceles,
    scalene,
};

TriangleKind kind(double a, double b, double c);

} // namespace triangle
