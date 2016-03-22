#pragma once

namespace leap {

inline bool is_leap_year(int year)
{
    auto has_factor = [year](int factor) {
        return year % factor == 0;
    };
    return has_factor(4) && !has_factor(100) || has_factor(400);
}

} // namespace leap
