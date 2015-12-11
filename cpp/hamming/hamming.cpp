#include "hamming.h"
#include <stdexcept>

using namespace std;

size_t hamming::compute(const string& a, const string& b)
{
    if (a.length() != b.length())
        throw domain_error("inputs of different length");

    size_t distance = 0;
    for (string::size_type i = 0; i < a.length(); ++i)
        if (a[i] != b[i])
            ++distance;
    return distance;
}
