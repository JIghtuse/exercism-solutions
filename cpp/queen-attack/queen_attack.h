#pragma once

#include <string>
#include <tuple>

namespace queen_attack {

using Position = std::pair<int, int>;

class chess_board {
public:
    chess_board(Position w = { 0, 3 }, Position b = { 7, 3 });
    Position white() const;
    Position black() const;
    explicit operator std::string() const;
    bool can_attack() const;

private:
    Position white_pos;
    Position black_pos;
};

} // namespace queen_attack
