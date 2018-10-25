# com-diag-rustler

This is an implementation of the Generic Cell Rate Algorithm in the Rust programming language.

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

* com-diag-rustler/Rustler/src/contract.rs - Implements a traffic contract throttle consisting of peak and sustained GCRAs.
* com-diag-rustler/Rustler/src/fletcher.rs - Implements the Fletcher sixteen-bit checksum algorithm.
* com-diag-rustler/Rustler/src/gcra.rs - Implements a Generic Cell Rate Algorithm (GCRA) throttle using a virtual scheduler.
* com-diag-rustler/Rustler/src/harness.rs - Provides at test harness for exercising throttles.
* com-diag-rustler/Rustler/src/throttle.rs - Describes the trait for a rate control algorithm.
* com-diag-rustler/Rustler/src/ticks.rs - Implements basic monotonic time functions for use in rate control.

## Executables

* com-diag-rustler/Rustler/src/bin/fletch - Computes the Fletcher-16 checksum of a data stream admitted from standard input and emitted to standard output.
* com-diag-rustler/Rustler/src/bin/shape - Shapes the data stream admitted from standard input and emitted to standard output.

## Remarks

I think my main issue with Rust was how under-documented it is. And because
it’s new-ish and a moving target, much of what is documented is subtly wrong.
This made the learning experience especially painful and often a matter of
laborious reverse engineering or trial and error. And unlike Go, I didn’t
find using Rust intuitive; for me, it was difficult to predict its
syntax and semantics. But there is much to admire about Rust.

I especially like Rust's memory management model. Instead of garbage collecting
in the background like Go (and Java (and Python (and ...))), it uses a
limited form of reference counting, and restricts how you can use references
(effectively pointers) so that it can manage the lifetime of variables mostly
through scope. It's pretty clever.

I also appreciate how trivial it was to call a C function from libc using
Rust. Since most stuff I work on entails integrating with some legacy
framework or library (typically to manage some custom hardware), this is
an important feature.

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

"The Rust Standard Library",
<https://doc.rust-lang.org/stable/std/>

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

"Gold"    
Raspberry Pi 3B+    
ARM ARMv7 64-bit    
Broadcom BCM2837B0 Cortex-A53 @ 1.4GHz x 4      
Raspbian 9.4 "stretch"    
Linux 4.14.34    
rustc 1.29.2 (17a9dc751 2018-10-05)    

## Clone

    mkdir -p ${HOME}/src
    cd ${HOME}/src
    git clone https://github.com/coverclock/com-diag-rustler
    cd com-diag-rustler/Rustler

## Build

    cd ${HOME}/src/com-diag-rustler/Rustler
    cargo build

## Unit Tests

    cd ${HOME}/src/com-diag-rustler/Rustler
    cargo test -- --nocapture --test-threads=1

## Functional Tests

    cd ${HOME}/src/com-diag-rustler/Rustler
    dd if=/dev/urandom count=1000 | ./target/debug/fletch -V -b 512 | ./target/debug/shape -V -p 2048 -s 1024 -b 512 | ./target/debug/fletch -V -b 512 > /dev/null

Valgrind works just fine with Rust, unlike my experience with Go.

    cd ${HOME}/src/com-diag-rustler/Rustler
    dd if=/dev/urandom count=1000 > DATA
    valgrind ./target/debug/fletch -V -b 512 < DATA > /dev/null
    valgrind ./target/debug/shape -V -p 2048 -s 1024 -b 512 < DATA > /dev/null

## Notes

Here is a cut and paste of the output of one of the functional tests.

    $ dd if=/dev/urandom count=1000 | ./target/debug/fletch -V -b 512 | ./target/debug/shape -V -p 2048 -s 1024 -b 512 | ./target/debug/fletch -V -b 512 > /dev/null
    Contract: Contract@0x7e90e8f8[112]:{p:Gcra@0x7e90e8f8[56]:{t:488282,i:488282,l:0,x:0,x1:0,f:{0,0,0},e:{1,1,1},a:{0,0}},s:Gcra@0x7e90e930[56]:{t:976563,i:976563,l:249511591,x:0,x1:0,f:{0,0,0},e:{1,1,1},a:{0,0}}}
    1000+0 records in
    1000+0 records out
    512000 bytes (512 kB, 500 KiB) copied, 362.642 s, 1.4 kB/s
    Total: 512000B.
    Average: 512B/io.
    Peak: 2310865.5816430617Bps.
    Sustained: 1156.6953908527787Bps.
    Checksum: 0x9c87.
    Total: 512000B.
    Average: 511.4885114885115B/io.
    Peak: 2043.1268645627642Bps.
    Sustained: 1023.993400802849Bps.
    Total: 512000B.
    Average: 355.0624133148405B/io.
    Peak: 4913958.297759311Bps.
    Sustained: 1023.9944224969396Bps.
    Checksum: 0x9c87.
