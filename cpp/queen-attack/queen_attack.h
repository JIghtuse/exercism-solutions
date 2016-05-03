#pragma once

#include <tuple>

namespace queen_attack {

class chess_board {
public:
    std::pair<int, int> white() const;
    std::pair<int, int> black() const;
};

} // namespace queen_attack
