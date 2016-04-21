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
    std::string cipher_text(bool with_spaces=false) const;
    std::string normalized_cipher_text() const;
private:
    std::string m_message;
    Size m_rows;
    Size m_cols;
};

} // namespace crypto_square
