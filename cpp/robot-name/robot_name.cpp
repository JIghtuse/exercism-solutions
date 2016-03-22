#include "robot_name.h"
#include <algorithm>
#include <unordered_set>

namespace robot_name {

bool original_name(const std::string& name)
{
    static std::unordered_set<std::string> names;
    auto result = names.insert(name);
    return result.second;
}

std::string random_name()
{
    static std::random_device rd;
    static std::mt19937 gen(rd());

    auto alphabet = std::string(26, ' ');
    std::iota(alphabet.begin(), alphabet.end(), 'A');
    std::shuffle(alphabet.begin(), alphabet.end(), gen);

    auto digits = std::string(10, '0');
    std::iota(digits.begin(), digits.end(), '0');
    std::shuffle(digits.begin(), digits.end(), gen);

    auto name = std::string(5, ' ');
    std::copy_n(alphabet.begin(), 2, name.begin());
    std::copy_n(digits.begin(), 3, name.begin() + 2);
    return name;
}

std::string original_random_name()
{
    auto name = random_name();
    while (!original_name(name)) {
        name = random_name();
    }
    return name;
}

robot::robot()
    : m_name{ original_random_name() }
{
}

std::string robot::name() const
{
    return m_name;
}

} // namespace robot_name
