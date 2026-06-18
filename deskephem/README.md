# deskephem - Modular CLI Astronomy

deskephem is a CLI astronomy calculator, that can print things like moon phases, approximate sunrise and sunset times, planet and star locations, and more.

[![crates.io](https://img.shields.io/crates/v/deskephem)](https://crates.io/crates/deskephem)
[![License](https://img.shields.io/badge/license-0BSD-blue.svg)](https://raw.githubusercontent.com/oliverkwebb/deskephem/main/LICENSE)
![GitHub last commit](https://img.shields.io/github/last-commit/oliverkwebb/deskephem)


```
$ deskephem moon phase # Werewolf early warning system
🌒 Waxing Crescent (5.0%)
$ deskephem sun rise set -l 40,-95 # Sunrise and Sunset
06:53 19:47
$ deskephem -l 30n,60w -E now,1h,+4h venus horiz
===================================================
         Date             Coordinates (Azi/Alt)    
===================================================
 2025-03-31T23:50:03  41°54′54.7″ -45°00′-31.5″    
 2025-04-01T00:50:03  56°49′18.1″ -35°00′-59.7″    
 2025-04-01T01:50:03  67°53′57.5″ -23°00′-10.4″    
 2025-04-01T02:50:03  76°46′8.2″ -11°00′-25.5″     
 2025-04-01T03:50:03  84°33′9.8″ +1°20′41.0″       
# Location and brightness of Uranus at William Herschel's first observation
$ deskephem -d 1781-03-13 -l 53n,1.8w Uranus horiz magnitude
278°45'42.91" 23°22'1.52" 5.60
```

It has a catalog of the planets, the moon and sun, and about 100 common stars. Of which it can print:

* Coordinates in the sky (equatorial, horizontal, ecliptic)
* Phase (Emoji, Illuminated Fraction, Name)
* Approximate Rise and Set times
* Brightness (Magnitude)
* Distance
* Angular Size
* Angles between other objects in the sky

It supports outputs in json, csv, and plain text. It can also generate tables of output:

```
$ time deskephem -l 35n,100w --ephem=-3h,1h,+3h sun horiz -Tcsv
Date,Coordinates (Azi/Alt)
2025-03-31T20:56:28,284°33′21.7″ -12°00′-45.4″
2025-03-31T21:56:28,294°35′42.9″ -24°00′-30.1″
2025-03-31T22:56:28,306°48′56.6″ -34°00′-53.9″
2025-03-31T23:56:28,322°31′54.9″ -43°00′-24.8″
2025-04-01T00:56:28,342°40′49.8″ -49°00′-8.6″
2025-04-01T01:56:28,05°53′2.9″ -50°00′-38.6″
2025-04-01T02:56:28,27°56′2.2″ -46°00′-36.1″
```

deskephem is a wrapper around the [`pracstro`](https://github.com/oliverkwebb/pracstro) astronomy library.

```
$ time deskephem -E 1600-01-01,1mon,9999-06-01 mars ecliptic # Query is ran ~100k times
===================================================
         Date            Coordinates (Ecliptic)    
===================================================
 1599-12-31T18:09:24  143°04′10.6″ +3°48′36.4″     
 1600-01-31T18:09:24  133°03′30.5″ +4°31′51.0″     
[...]
1.89 user 0.20 system
```

# Parameters

All input to deskephem is case insensitive

Options:
* Date: `-d [DATE]` (default: `now`)
* Observer Latitude/Longitude: `-l [LAT,LONG]` (default: `none`)
* Output format: `-T [FORMAT]` (default: `term`, options: `csv`, `json`, `term`, `rawterm`)
* Ephemeris generation: `-E start,step,end` (optional)

Arguments:
* Object being observed
* Properties

# Properties

* Equatorial Coordinates: `equ`, `equa`, `equatorial`
* Horizon Coordinates: `horiz`, `horizontal`
* Ecliptic Coordinates: `ecl`, `ecliptic`
* Distance: `dist`, `distance`
* (Brightness) Magnitude: `mag`, `magnitude`, `brightness`
* Phase Info: `phase`
* Phase Emoji: `phaseemoji`
* Phase Name; `phasename`
* Phase Angle: `phaseangle`
* Angular Diameter: `angdia`
* Illuminated Percent: `phaseprecent`, `illumfrac`
* Rise and set times: `rise`, `set`
* Angle between objects: `angbetween:{Object}`

# Input formatting

Dates:
* Relative Dates: `+/-#y/mon/d/h/min/s` (ex. `+3mon`)
* Time increments: `#y/mon/d/h/min/s` (ex. `1w`)
* ISO format: `YYYY-MM-DD[THH:MM:SS]` (ex. `1970-01-01T14:40:25`, `2000-04-52`)
* Unix dates: `@#` (ex. `@86400`)
* Julian dates: `##j[d]` (ex. `2451545`)
* Now: `now` (ex. `now`)

Angles:
* Lat/Long: `Angle,Angle` (ex. `0,0`, `30n,60w`)
* Degrees: `###°/d[eg]` (ex. `45deg`)

Epehmeris Queries: `Date (Start),Time Increment,Date (End)`

Object:
* Object name: `{name}` (ex. `moon`, `Sun`, `Arcturus`)
* Raw coordinates: `latlong:{lat,long}` (ex. `latlong:0,45w`)
