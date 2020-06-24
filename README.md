# sctd - set color temperature daemon

Based on [sct](https://flak.tedunangst.com/post/sct-set-color-temperature) by Ted Unangst. Calculates sunrise and sunset based on geo-pos lat/lon and sets the temperature accordingly. Can be left running, in which case transitions between tempratures. Transition logic is based on [redshift](https://github.com/jonls/redshift/).

## Installation

Precompiled binaries are [available](https://github.com/amir/sctd/releases).

## Usage

### Set latitude and longitude
```bash
$ sctd --latitude 53.3498 --longitude -6.2603
```

### Reset
```bash
$ sctd --reset
```

Executing `sctd` in `.xinitrc` is an easy way to start it with `startx`.
