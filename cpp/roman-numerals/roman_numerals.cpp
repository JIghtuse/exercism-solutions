#include "roman_numerals.h"

std::string roman::convert(unsigned number)
{
    std::string roman_number;
    for (int i = 0; i < number; ++i) {
        roman_number += "I";
    }
    return roman_number;
}
