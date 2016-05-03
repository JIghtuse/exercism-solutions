#include "queen_attack.h"

namespace queen_attack {

std::pair<int, int> chess_board::white() const
{
    return white_pos;
}

std::pair<int, int> chess_board::black() const
{
    return black_pos;
}

} // namespace queen_attack
