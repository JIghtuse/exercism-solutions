#include <array>
#include <cmath>
#include "nth_prime.h"

std::array<unsigned, 5> first_primes{
    2, 3, 5, 7, 11
};

unsigned prime::nth(unsigned n)
{
    if (n <= first_primes.size()) {
        return first_primes[n - 1];
    }
    return 0;
}
