#include "sum_of_multiples.h"

unsigned sum_of_multiples::to(unsigned n)
{
    return to({3, 5}, n);
}

unsigned last_multiplier_index(unsigned multiplier, unsigned n)
{
    return n / multiplier - !(n % multiplier);
}

unsigned sum_from_1_to_n(unsigned n)
{
    return n * (n + 1) / 2;
}

unsigned sum_of_multiples::to(std::initializer_list<unsigned> multipliers, unsigned n)
{
    unsigned sum = 0;
    for (auto multiplier: multipliers) {
        auto last = last_multiplier_index(multiplier, n);
        sum += multiplier * sum_from_1_to_n(last);
    }
    for (auto it = multipliers.begin(); it != multipliers.end(); ++it) {
        for (auto nit = it + 1; nit != multipliers.end(); ++nit) {
            unsigned multiplier = *it * *nit;
            auto last = last_multiplier_index(multiplier, n);
            sum -= multiplier * sum_from_1_to_n(last);
        }
    }
    return sum;
}
