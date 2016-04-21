#include "crypto_square.h"
#include <cctype>

namespace crypto_square {

cipher::cipher(const std::string& message)
    : m_normalized_message{}
{
    for (auto c: message) {
        if (std::isalnum(c)) {
            m_normalized_message += std::tolower(c);
        }
    }
}

std::string cipher::normalize_plain_text() const
{
    return m_normalized_message;
}

} // namespace crypto_square
