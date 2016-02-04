#include <algorithm>
#include <stdexcept>
#include "nucleotide_count.h"

using namespace std;
using std::experimental::string_view;

const string kNucleotides{ "ACGT" };

inline bool is_valid_nucleotide(char c)
{
    return find(kNucleotides.begin(), kNucleotides.end(), c) != kNucleotides.end();
}

dna::counter::counter(string_view strand)
    : strand{ strand }
    , stats{}
{
    for (auto nucleotide : kNucleotides)
        stats[nucleotide] = 0;

    for (auto c : strand) {
        if (!is_valid_nucleotide(c))
            throw invalid_argument{ "No such nucleotide" };
        stats[c]++;
    }
}

map<char, int> dna::counter::nucleotide_counts() const
{
    return stats;
}

int dna::counter::count(char c) const
{
    if (!is_valid_nucleotide(c))
        throw std::invalid_argument{ "No such nucleotide" };
    return stats.at(c);
}
