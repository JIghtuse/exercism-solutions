#include "queen_attack.h"
#include <sstream>
#include <stdexcept>

namespace {

constexpr int kFieldWidth = 8;
constexpr int kFieldHeight = 8;
constexpr char kEmptyCell = '_';
constexpr char kBlackQueenCell = 'B';
constexpr char kWhiteQueenCell = 'W';

} // namespace

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

chess_board::operator std::string() const
{
    std::ostringstream oss;

    for (auto i = 0; i < kFieldHeight; ++i) {
        for (auto j = 0; j < kFieldWidth; ++j) {
            if (i == white_pos.first && j == white_pos.second) {
                oss << kWhiteQueenCell;
            } else if (i == black_pos.first && j == black_pos.second) {
                oss << kBlackQueenCell;
            } else {
                oss << kEmptyCell;
            }
            if (j != kFieldWidth - 1) {
                oss << ' ';
            }
        }
        oss << '\n';
    }
    return oss.str();
}

bool chess_board::can_attack() const
{
    auto dx = white_pos.first - black_pos.first;
    auto dy = white_pos.second - black_pos.second;
    return dx * dx == dy * dy
        || dx == 0 && dy != 0
        || dy == 0 && dx != 0;
}

} // namespace queen_attack
