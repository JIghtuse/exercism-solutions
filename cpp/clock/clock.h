#pragma once

#include <string>

namespace date_independent {

using Hours = char;
using Minutes = char;

class clock {
public:
    clock(Hours, Minutes);
    static clock at(Hours h, Minutes m=0);
    operator std::string() const;

private:
    Hours hours;
    Minutes minutes;
};

} // namespace date_independent
