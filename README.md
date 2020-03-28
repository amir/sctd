# sctd - set color temperature daemon

Based on [sct](https://flak.tedunangst.com/post/sct-set-color-temperature) by Ted Unangst. Can be run as a daemon and calculates sunrise and sunset based on geo-pos lat/lon and sets the temperature accordingly.

## Usage

### Set latitude and longitude
```bash
$ cargo run -- --latitude 53.3498 --longitude -6.2603
```

### Reset
```bash
$ cargo run -- --reset
```
