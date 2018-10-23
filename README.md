# com-diag-rustler

Musings with the Rust programming language.

## Copyright

Copyright 2018 by the Digital Aggregates Corporation.

## License

Licensed under the terms of the Lesser GNU Public License version 2.1.

## Trademarks

"Digital Aggregates Corporation" is a registered trademark.

"Chip Overclock" is a registered trademark.

## Contact

Chip Overclock    
<mailto:coverclock@diag.com>    
Digital Aggregates Corporation    
<http://wwww.diag.com>    
3440 Youngfield St. #209    
Wheat Ridge CO 80033    

## Abstract

This repository contains the results of my attempts to learn the Rust
programming language. I did this in my usual fashion: by porting my "go to"
learning example, the generic cell rate algorithm. So far, I've successfully
implemented and tested the GCRA in C++, Java, C, Go, and Rust.

## Modules

* com-diag-rustler/rustler/src/contract.rs - Implements a traffic contract throttle consisting of peak and sustained GCRAs.
* com-diag-rustler/rustler/src/fletcher.rs - Implements the Fletcher sixteen-bit checksum algorithm.
* com-diag-rustler/rustler/src/gcra.rs - Implements a Generic Cell Rate Algorithm (GCRA) throttle using a virtual scheduler.
* com-diag-rustler/rustler/src/harness.rs - Provides at test harness for exercising throttles.
* com-diag-rustler/rustler/src/throttle.rs - Describes the trait for a rate control algorithm.
* com-diag-rustler/rustler/src/ticks.rs - Implements basic monotonic time functions for use in rate control.

## Remarks

I am reminded of a remark made by a colleage of mine from decades ago about
the Ada programming language: "If you can get your program to compile, it
often runs the first time".

## Repositories

<https://github.com/coverclock/com-diag-rustler>

<https://github.com/coverclock/com-diag-vamoose>

<https://github.com/coverclock/com-diag-diminuto>

<https://github.com/coverclock/com-diag-buckaroo>

<https://github.com/coverclock/com-diag-grandote>

## Articles

C. Overclock, "Traffic Management", 2006-12-25,
<http://coverclock.blogspot.com/2006/12/traffic-management.html>

C. Overclock, "Rate Control and Throttles", 2007-01-12,
<http://coverclock.blogspot.com/2007/01/rate-control-and-throttles.html>

C. Overclock, "Traffic Contracts", 2007-01-17,
<http://coverclock.blogspot.com/2007/01/traffic-contracts.html>

## References

S. Klabnik and C. Nichols, "The Rust Programming Language", No Starch Press,
2018, <https://doc.rust-lang.org/book/2018-edition/foreword.html>

J. Blandy and J. Orendorff, "Programming Rust", O'Reilly, 2018

"Rust By Example",
<https://doc.rust-lang.org/rust-by-example/index.html>

J. Sloan, "ATM Traffic Management", Digital Aggregates Corporation, 2005-08-29,
<http://www.diag.com/reports/ATMTrafficManagement.html>

N. Giroux et al., "Traffic Management Specification Version 4.1", ATM Forum,
af-tm-0121.000, 1999-03

Wikipedia, "Generic cell rate algorithm", 2017-08-23,
<https://en.wikipedia.org/wiki/Generic_cell_rate_algorithm>

## Targets

Various versions of this software has at one time or another been installed
and tested with the following combinations of hardware and software. Your
mileage may vary.

"Nickel"
Intel NUC5i7RYH    
Intel x86_64 64-bit    
Intel Core i7-5557U @ 3.10GHz x 2 x 2    
Ubuntu 18.04 "bionic"    
Linux 4.15.0    
rustc 1.29.2 (17a9dc751 2018-10-05)    

## Clone

    cd $HOME
    mkdir src
    cd src
    git clone https://github.com/coverclock/com-diag-rustler
    cd com-diag-rustler/rustler

## Build

    cargo build

## Unit Tests

    cargo test -- --nocapture --test-threads=1

## Functional Tests

## Notes

