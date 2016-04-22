#pragma once

#include <experimental/string_view>

namespace scrabble_score {

using Score = int;

Score score(std::experimental::string_view word);

} // namespace scrabble_score
