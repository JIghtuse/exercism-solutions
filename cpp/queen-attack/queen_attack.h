#pragma once

#include <tuple>

namespace queen_attack {

using Position = std::pair<int, int>;

class chess_board {
public:
    chess_board(Position w = { 0, 3 }, Position b = { 7, 3 })
        : white_pos{ w }
        , black_pos{ b }
    {
    }
    Position white() const;
    Position black() const;

private:
    Position white_pos;
    Position black_pos;
};

} // namespace queen_attack
