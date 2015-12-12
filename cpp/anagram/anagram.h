#pragma once

#include <initializer_list>
#include <string>
#include <vector>

namespace anagram {

class anagram {
public:
    anagram(const std::string& s)
        : word{ s }
    {
    }
    ~anagram() = default;

    std::vector<std::string> matches(const std::initializer_list<const std::string>);

private:
    std::string word;
};
};
