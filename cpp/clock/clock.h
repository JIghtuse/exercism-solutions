#pragma once

#include <string>

namespace date_independent {

class clock {
public:
    static clock at(unsigned);
    operator std::string();
};

} // namespace date_independent
