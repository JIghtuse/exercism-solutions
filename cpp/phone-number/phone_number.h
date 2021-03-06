#ifndef PHONE_NUMBER_H
#define PHONE_NUMBER_H

#include <experimental/string_view>
#include <string>

class phone_number {
public:
    explicit phone_number(std::experimental::string_view sv);
    std::string number() const;
    std::string area_code() const;
    operator std::string() const;

private:
    std::string number_;
};

#endif // PHONE_NUMBER_H
