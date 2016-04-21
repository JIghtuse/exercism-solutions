#pragma once

#include <string>
#include <vector>

namespace crypto_square {

using Size = std::string::size_type;

class cipher {
public:
    cipher(const std::string& message);
    std::string normalize_plain_text() const;
    Size size() const;
    std::vector<std::string> plain_text_segments() const;
private:
    std::string m_normalized_message;
    Size m_rows;
    Size m_cols;
};

} // namespace crypto_square
