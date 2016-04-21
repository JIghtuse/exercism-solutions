#include "crypto_square.h"
#include <cctype>
#include <cmath>
#include <tuple>

namespace crypto_square {

std::pair<Size, Size> calculate_size(Size length)
{
    auto square_root = std::floor(std::sqrt(length));
    return std::make_pair(square_root, square_root);
}

cipher::cipher(const std::string& message)
    : m_normalized_message{}
{
    for (auto c: message) {
        if (std::isalnum(c)) {
            m_normalized_message += std::tolower(c);
        }
    }

    std::tie(m_rows, m_cols) = calculate_size(m_normalized_message.length());
}

std::string cipher::normalize_plain_text() const
{
    return m_normalized_message;
}

Size cipher::size() const
{
    return m_cols;
}

} // namespace crypto_square
