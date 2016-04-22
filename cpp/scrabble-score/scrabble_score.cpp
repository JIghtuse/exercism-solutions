#include "scrabble_score.h"
#include <numeric>
#include <unordered_map>

namespace {

const std::unordered_map<char, scrabble_score::Score> kLetterScores{
    { 'A', 1 }, { 'E', 1 }, { 'I', 1 }, { 'O', 1 }, { 'U', 1 },
    { 'L', 1 }, { 'N', 1 }, { 'R', 1 }, { 'S', 1 }, { 'T', 1 },
    { 'D', 2 }, { 'G', 2 },
    { 'B', 3 }, { 'C', 3 }, { 'M', 3 }, { 'P', 3 },
    { 'F', 4 }, { 'H', 4 }, { 'V', 4 }, { 'W', 4 }, { 'Y', 4 },
    { 'K', 5 },
    { 'J', 8 }, { 'X', 8 },
    { 'Q', 10 }, { 'Z', 10 },
};

} // namespace

namespace scrabble_score {

Score score(std::experimental::string_view word)
{
    return std::accumulate(word.begin(), word.end(), 0, [](Score score, char c) {
        return score + kLetterScores.at(std::toupper(c));
    });
}

} // scrabble_score
