# ruuvi-logger

[![build](https://github.com/paasim/ruuvi-logger/workflows/build/badge.svg)](https://github.com/paasim/ruuvi-logger/actions)

Log ruuvitag observations queried with [ruuvi][ruuvi] to sqlite database.

Dependencies
------------

See [ruuvi][ruuvi].

Usage
-----

    # download and install the package
    curl -O -L https://github.com/paasim/ruuvi-logger/releases/download/v0.2.0/ruuvi-logger_0.2.0_amd64.deb
    apt install -f ./ruuvi-logger_0.2.0_amd64.deb

    # set db path and correct mac addresses (can be queried with ruuvi)
    # (optionally also set the minimum duration for the log to be downloaded)
    vim /etc/ruuvi-logger/ruuvi-logger.conf

    # enable and start systemctl timer that updates the db daily
    systemctl enable ruuvi-logger.timer
    systemctl start ruuvi-logger.timer


[ruuvi]: https://github.com/paasim/ruuvi
