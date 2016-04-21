#pragma once

#include <string>

namespace crypto_square {

class cipher {
public:
    cipher(const std::string& message);
    std::string normalize_plain_text() const;
private:
    std::string m_normalized_message;
};

} // namespace crypto_square
