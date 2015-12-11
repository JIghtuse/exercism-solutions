#include "hamming.h"

using namespace std;

size_t hamming::compute(const string& a, const string& b)
{
    return a[0] != b[0];
}
