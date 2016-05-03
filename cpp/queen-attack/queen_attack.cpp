#include "queen_attack.h"
#include <stdexcept>

namespace queen_attack {

chess_board::chess_board(Position w, Position b)
    : white_pos{ w }
    , black_pos{ b }
{
    if (white_pos == black_pos) {
        throw std::domain_error("Queen positions must be distinct");
    }
}

Position chess_board::white() const
{
    return white_pos;
}

Position chess_board::black() const
{
    return black_pos;
}

} // namespace queen_attack
