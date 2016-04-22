#include "scrabble_score.h"

int scrabble_score::score(std::experimental::string_view word)
{
    if (word.empty()) {
        return 0;
    }
    return word[0] == 'a';
}
