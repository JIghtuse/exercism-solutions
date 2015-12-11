#include "hamming.h"
#include <functional>
#include <numeric>
#include <stdexcept>

using namespace std;

size_t hamming::compute(const string& a, const string& b)
{
    if (a.length() != b.length())
        throw domain_error("inputs of different length");

    return inner_product(a.begin(), a.end(), b.begin(), 0,
        plus<size_t>(), not_equal_to<char>());
}
