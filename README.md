# COLLATZ-PROBLEM-TESTER
Just a minimal tester for collatz conjecture

## Building
```sh
$ git clone https://codeberg.org/XDream8/collatz-problem-tester
$ cd collatz-problem-tester
$ cargo build --profile optimized
$ ./target/optimized/collatz-problem-tester -h
```

## Usage
```sh
$ collatz-problem-tester -h
$ collatz-problem-tester [flags] <number>
```

## Flags

### random(-r, --random + -u8, -u16, -u32, -u64)
Test a random number.
By default u16 numbers are generated
With `-u8`, `-u16`, `-u32`, `-u64` flags you can set how big you want your random number to be
```sh
$ collatz-problem-tester --random
$ collatz-problem-tester --random -u8
$ collatz-problem-tester --random -u16
$ collatz-problem-tester --random -u32
$ collatz-problem-tester --random -u64
```

### verbose(-v --verbose)
Log everything that's going on(might cause performance issues for big numbers).
Disabled by default.
```sh
$ collatz-problem-tester --verbose 6
```

### optimised odd algorithm(-o, --optimised-odd-algorithm)
Optimised odd algorithm(instead of '3k+1' use '2k+2'.
This will speed up the process a lot since this will reduce the tries that needs to be made to find the end result.
Just try and see it yourself.
```sh
$ collatz-problem-tester --optimised-odd-algorithm 91
```

## References
[Wikipedia](https://en.wikipedia.org/wiki/Collatz_conjecture)
