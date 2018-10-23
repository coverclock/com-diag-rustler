/* vi: set ts=4 expandtab shiftwidth=4: */

// Copyright 2018 by the Digital Aggregates Corporation
// Licensed under the terms in LICENSE.txt
// Author: Chip Overclock
// mailto:coverclock@diag.com
// https://github.com/coverclock/com-diag-rustler

extern crate clap;
extern crate rustler;

use std::io;
use std::io::Read;
use std::io::Write;
use std::io::ErrorKind;
use clap::Arg;
use clap::App;
use rustler::ticks::ticks;
use rustler::fletcher::fletcher;

fn main() {
    let frequency: f64 = ticks::frequency() as f64;
    let before: ticks::Ticks;
    let after: ticks::Ticks;
    let mut cs: fletcher::Fletcher = fletcher::Fletcher::new();
    let mut total: usize = 0;
    let mut peak: f64 = 0.0;
    let mut count: usize = 0;
    let mut now: ticks::Ticks = 0;
    let mut c: u16 = 0;
    let mut then: ticks::Ticks;
    let mut rate: f64;
    let mut length: usize;
    let mut buffer = [0u8; 65536] ; // Does Rust really have no way to allocate a u8 array on the heap at run time?

    let matches = App::new("fletch")
                          .version("1.0")
                          .author("Chip Overclock <coverclock@diag.com>")
                          .about("This filter computes a 16-bit Fletcher checksum on the data stream.")
                          .arg(Arg::with_name("Debug")
                               .short("D")
                               .help("Enable debug output."))
                          .arg(Arg::with_name("Verbose")
                               .short("V")
                               .help("Enable verbose output."))
                           .arg(Arg::with_name("blocksize")
                               .short("b")
                               .help("Set I/O block size.")
                               .takes_value(true)
                               .value_name("BYTES"))
                          .get_matches();

    let debug: bool = matches.is_present("Debug");

    let verbose: bool = matches.is_present("Verbose");

    let mut blocksize: usize = 1;
    if matches.is_present("blocksize") {
        blocksize = match usize::from_str_radix(matches.value_of("blocksize").unwrap(), 10) {
            Ok(value) => value,
            Err(_) => 0,
        }
    }
    if !((0 < blocksize) && (blocksize <= 65536)) {
        panic!("blocksize not valid!");
    }

    before = ticks::now();

    loop {
        
        length = match io::stdin().read(& mut buffer[..blocksize]) {
            Ok(0) => break,
            Ok(value) => value,
            Err(ref error) if error.kind() == ErrorKind::Interrupted => continue,
            Err(_) => break,     
        };
        if debug { eprintln!("Read: {}", length); }

        c = cs.checksum(&buffer[..length]);
        
        then = now;
        now = ticks::now();

        if count <= 0 {
            // Do nothing.
        } else if length <= 0 {
            // Should never happen.
        } else if now <= then {
            // Should never happen.
        } else {
            rate = (length as f64) * frequency / ((now - then) as f64);
            if rate > peak {
                peak = rate
            }
        }

        match io::stdout().write_all(&buffer[..length]) {
            Ok(_) => { },
            Err(_) => break,
        }
        if debug { eprintln!("Written: {}", length); }

        total += length as usize;
        count += 1;
        
    }
    
    after = ticks::now();

    if verbose {
        eprintln!("Total: {}B.", total);
        eprintln!("Average: {}B/io.", (total as f64) / (count as f64));
        eprintln!("Peak: {}Bps.", peak);
        eprintln!("Sustained: {}Bps.", (total as f64) * (frequency as f64) / ((after - before) as f64));
        eprintln!("Checksum: 0x{:04x}.", c);        
    }

}
