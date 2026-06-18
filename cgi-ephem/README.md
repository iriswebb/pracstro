# cgi-ephem - Pretty CLI Astronomy

This is not a standalone CGI script, rather, a tool made to be used in close conjunction with one

```
$ cgi-ephem text moon
+--------------------- Location ----------------------+----------------------- Phase -----------------------+
|                                                     |                                                     |
|                                                     |           @@@@                                      |
|           In the Constellation Capricornus          |           @@@@@@@                                   |
|                  Zodiac: Aquarius                   |            @@@@@@@@                                 |
|                                                     |            @@@@@@@@@                                |
|                                                     |            @@@@@@@@@                                |
|              Coordinates (Equatorial):              |            @@@@@@@@@     Waxing Crescent (40.1%)    |
|               21h46m59s -15°00′-55.9″               |            @@@@@@@@@                                |
|                                                     |            @@@@@@@@                                 |
|               Coordinates (Ecliptic):               |           @@@@@@@                                   |
|              323°48′33.8″ -1°00′-9.2″               |           @@@@                                      |
|                                                     |                                                     |
|                                                     |                                                     |
+--------------------- Distance ----------------------+-------------------- Brightness ---------------------+
|                                                     |                                                     |
|                Distance: 385127.7 km                |                 Brightness: -11.74                  |
|                                                     |                                                     |
|                  Moon (max): 34'6"                  |                          -                          |
|                  Sun (max): 32'32"                  |                  Sun (avg): -26.83                  |
|                  Sun (min): 31'27"                  |                  Full Moon: -12.6                   |
|           Current Observation: 00°31′1.6″           |             Current Observation: -11.74             |
|                  Venus (max): 1'6"                  |                 Venus (max): -4.92                  |
|             Human Eye Resolution: 1'0"              |                  Mars (max): -2.94                  |
|               Jupiter (max): 0'50.1"                |                Jupiter (max): -2.94                 |
|                                                     |                                                     |
|                                                     |                                                     |
|                                                     |                                                     |
+-----------------------------------------------------+-----------------------------------------------------+
```

# Usage:

`cgi-ephem [FORMAT] [OBJECT]`

format:
* `ansi` for ANSI escape codes
* `html` for html
* `text` for plaintext
