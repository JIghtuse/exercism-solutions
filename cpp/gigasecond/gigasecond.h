#pragma once

#include <ratio>
#include <boost/date_time/gregorian/gregorian.hpp>
#include <boost/date_time/posix_time/posix_time.hpp>

namespace gigasecond {

using date = boost::gregorian::date;

inline date advance(const date& d)
{
    using namespace boost::posix_time;
    auto time = ptime{ d } + seconds{ std::giga::num };
    return time.date();
}

} // namespace gigasecond
