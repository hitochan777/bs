# bjc

BSON <-> JSON converter

:warning: Currently only Linux is supported

## Installation

```bash
brew tap hitochan777/bjc
brew install hitochan777/tap/bjc
```

## Usage

* From BSON to JSON
  ```bash
  cat /path/to/bson/file | bjc -d true
  ```
  
* From JSON to BSON
  ```bash
  cat /path/to/json/file | bjc
  ```
  
For full documentation, run `bjc -h`.

## License
MIT
