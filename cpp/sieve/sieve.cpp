#include "sieve.h"
#include <algorithm>
#include <numeric>

std::vector<int> sieve::primes(int n)
{
    auto res{ std::vector<int>(n - 1) };
    std::iota(res.begin(), res.end(), 2);

    for (auto pos = 0u; pos != res.size(); ++pos) {
        auto prime = res[pos];
        auto not_prime = [prime](int i) { return i != prime && i % prime == 0; };
        res.erase(std::remove_if(res.begin(), res.end(), not_prime), res.end());
    }

    return res;
}
