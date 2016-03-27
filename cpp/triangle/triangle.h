#pragma once

namespace triangle {

enum TriangleKind {
    equilateral,
};

TriangleKind kind(unsigned a, unsigned b, unsigned c);

} // namespace triangle
