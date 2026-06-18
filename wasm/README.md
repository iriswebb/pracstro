# WebEphem - Modular Web Astronomy

webephem is a WASM/Rust astronomy calculator, that can print things such as moon phases, approximate sunrise and sunset times, planet and star locations, and more.

[![License](https://img.shields.io/badge/license-0BSD-blue.svg)](https://raw.githubusercontent.com/oliverkwebb/deskephem/main/LICENSE)

webephem has a catalog of the planets, the moon, the sun, and about 100 common stars. Of which it can print:

* Coordinates in the sky (equatorial, horizontal, ecliptic)
* Phase (Emoji, Illuminated Fraction, Name)
* Approximate Rise and Set times
* Brightness (Magnitude)
* Distance
* Angular Size
* Angles between other objects in the sky

webephem is a wrapper around the [`pracstro`](https://github.com/oliverkwebb/pracstro) astronomy library.
Since it is in WASM, and sacrifices some accuracy for algorithmic berevity, it runs very fast.

# API

To use the API, initialize the catalog with `catalog_init()`. Then there are two main functions to query the library:

`webephem_query()` - generates a single property of a celestial object at a given time and location.
`webephem_batch_query()` - generates a array of results over a time-range
