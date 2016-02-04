#ifndef RNA_TRANSCRIPTION_H
#define RNA_TRANSCRIPTION_H

#include <string>
#include <experimental/string_view>

namespace transcription {
    char to_rna(char dna_nucleotide);
    std::string to_rna(std::experimental::string_view strand);
}

#endif // RNA_TRANSCRIPTION_H
