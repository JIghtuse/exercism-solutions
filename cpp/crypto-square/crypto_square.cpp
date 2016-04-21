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
    : m_message{}
{
    for (auto c: message) {
        if (std::isalnum(c)) {
            m_message += std::tolower(c);
        }
    }

    std::tie(m_rows, m_cols) = calculate_size(m_message.length());
}

std::string cipher::normalize_plain_text() const
{
    return m_message;
}

Size cipher::size() const
{
    return m_cols;
}

std::vector<std::string> cipher::plain_text_segments() const
{
    auto result = std::vector<std::string>{};
    for (auto i = Size{0}; i < m_message.length(); i += m_cols) {
        auto count = std::min(m_cols, m_message.length() - i);
        result.emplace_back(m_message.substr(i, count));
    }
    return result;
}

std::string cipher::cipher_text(bool with_spaces) const
{
    auto result = std::string{};

    for (auto i = Size{0}; i < m_cols; ++i) {
        for (auto j = i; j < m_message.length(); j += m_cols) {
            result.push_back(m_message[j]);
        }
        if (with_spaces) {
            result.push_back(' ');
        }
    }
    return result;
}

std::string cipher::normalized_cipher_text() const
{
    return cipher_text(true);
}

} // namespace crypto_square
