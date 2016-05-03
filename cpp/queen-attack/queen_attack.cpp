#include "queen_attack.h"

namespace queen_attack {

std::pair<int, int> chess_board::white() const
{
    return { 0, 3 };
}

std::pair<int, int> chess_board::black() const
{
    return { 7, 3 };
}

} // namespace queen_attack
