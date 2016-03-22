#pragma once

#include <map>

namespace space_age {

namespace {

using OrbitalPeriod = double;

enum class Planet {
    Earth,
    Mercury,
    Venus,
    Mars,
    Jupiter,
    Saturn,
    Uranus,
    Neptune,
};

constexpr double kEarthOrbitalPeriod{ 31557600 };

const std::map<Planet, OrbitalPeriod> kOrbitalPeriods{
    { Planet::Earth, kEarthOrbitalPeriod },
    { Planet::Mercury, 0.2408467 * kEarthOrbitalPeriod },
    { Planet::Venus, 0.61519726 * kEarthOrbitalPeriod },
    { Planet::Mars, 1.8808158 * kEarthOrbitalPeriod },
    { Planet::Jupiter, 11.862615 * kEarthOrbitalPeriod },
    { Planet::Saturn, 29.447498 * kEarthOrbitalPeriod },
    { Planet::Uranus, 84.016846 * kEarthOrbitalPeriod },
    { Planet::Neptune, 164.79132 * kEarthOrbitalPeriod },
};

} // namespace

class space_age {
public:
    explicit space_age(double seconds)
        : m_seconds{ seconds }
    {
    }
    double seconds() const { return m_seconds; }

    double on_earth() const { return on_planet(Planet::Earth); }
    double on_mercury() const { return on_planet(Planet::Mercury); }
    double on_venus() const { return on_planet(Planet::Venus); }
    double on_mars() const { return on_planet(Planet::Mars); }
    double on_jupiter() const { return on_planet(Planet::Jupiter); }
    double on_saturn() const { return on_planet(Planet::Saturn); }
    double on_uranus() const { return on_planet(Planet::Uranus); }
    double on_neptune() const { return on_planet(Planet::Neptune); }
private:
    double on_planet(const Planet& p) const
    {
        return m_seconds / kOrbitalPeriods.at(p);
    }
    double m_seconds;
};

} // namespace space_age
