#include <algorithm>
#include <array>
#include <stdexcept>
#include <vector>
#include "nth_prime.h"

std::array<unsigned, 5> precomputed_primes{
    2, 3, 5, 7, 11
};

inline bool is_prime(const std::vector<unsigned>& calculated_primes, unsigned candidate)
{
    auto has_factor = [candidate](unsigned prime) { return candidate % prime == 0; };

    return std::none_of(precomputed_primes.begin(), precomputed_primes.end(), has_factor)
        && std::none_of(calculated_primes.begin(), calculated_primes.end(), has_factor);
}

unsigned prime::nth(unsigned n)
{
    if (n == 0) {
        throw std::domain_error("Incorrect input");
    }
    if (n <= precomputed_primes.size()) {
        return precomputed_primes[n - 1];
    }
    std::vector<unsigned> calculated_primes;
    calculated_primes.reserve(n - precomputed_primes.size());

    while (calculated_primes.size() != n - precomputed_primes.size()) {
        unsigned candidate = calculated_primes.empty()
            ? precomputed_primes.back() + 2
            : calculated_primes.back() + 2;
        while (!is_prime(calculated_primes, candidate)) {
            candidate += 2;
        }
        calculated_primes.push_back(candidate);
    }
    return calculated_primes.back();
}
