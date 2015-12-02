#pragma once

#include <map>
#include <string>

namespace word_count {
    std::map<std::string, int> words(const std::string& text);
}
