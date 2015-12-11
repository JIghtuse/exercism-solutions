#include "hamming.h"

using namespace std;

size_t hamming::compute(const string& a, const string& b)
{
    size_t distance = 0;
    for (string::size_type i = 0; i < a.length(); ++i)
        if (a[i] != b[i])
            ++distance;
    return distance;
}
