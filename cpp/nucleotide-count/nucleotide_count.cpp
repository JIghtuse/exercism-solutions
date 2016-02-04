#include "nucleotide_count.h"

using namespace std;
using std::experimental::string_view;

dna::counter::counter(string_view strand)
    : strand{ strand }
    , stats{
        { 'A', 0 },
        { 'C', 0 },
        { 'G', 0 },
        { 'T', 0 },
    }
{
    for (auto c : strand)
        stats[c]++;
}

map<char, int> dna::counter::nucleotide_counts() const
{
    return stats;
}

int dna::counter::count(char c) const
{
    return stats.at(c);
}
