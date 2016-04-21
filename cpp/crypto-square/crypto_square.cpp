#include "crypto_square.h"
#include <cctype>
#include <cmath>
#include <tuple>

namespace crypto_square {

std::pair<Size, Size> calculate_size(Size length)
{
    auto square_root = std::sqrt(length);
    auto rows = std::floor(square_root);
    auto cols = std::ceil(square_root);
    if (rows * cols != length) {
        ++rows;
    }
    return std::make_pair(rows, cols);
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

std::vector<std::string> cipher::plain_text_segments() const
{
    auto result = std::vector<std::string>{};
    for (auto i = Size{0}; i < m_normalized_message.length(); i += m_cols) {
        auto count = std::min(m_cols, m_normalized_message.length() - i);
        result.emplace_back(m_normalized_message.substr(i, count));
    }
    return result;
}

std::string cipher::cipher_text() const
{
    auto result = std::string(m_normalized_message.length(), ' ');
    auto counter = Size{0};

    for (auto i = Size{0}; i < m_cols; ++i) {
        for (auto j = i; j < m_normalized_message.length(); j += m_cols) {
            result[counter++] = m_normalized_message[j];
        }
    }
    return result;
}

} // namespace crypto_square
