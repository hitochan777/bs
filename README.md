# bs

BSON <-> JSON converter

:warning: Currently only Linux is supported

## Installation

```bash
brew tap hitochan777/bs
brew install hitochan777/tap/bs
```

## Usage

* From BSON to JSON
  ```bash
  cat /path/to/bson/file | bs -d true
  ```
  
* From JSON to BSON
  ```bash
  cat /path/to/json/file | bs
  ```
  
For full documentation, run `bs -h`.

## License
MIT
