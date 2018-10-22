/* vi: set ts=4 expandtab shiftwidth=4: */

// Copyright 2018 by the Digital Aggregates Corporation
// Licensed under the terms in LICENSE.txt
// Author: Chip Overclock
// mailto:coverclock@diag.com
// https://github.com/coverclock/com-diag-rustler

extern crate rustler;

use rustler::fletcher::fletcher;

#[test]
fn test_fletcher_100_checksum() {
    let buffer: [u8; 6] = [ 'q' as u8, 'w' as u8, 'e' as u8, 'r' as u8, 't' as u8, 'y' as u8 ];
    let cs: fletcher::Fletcher = fletcher::new();
    let mut c: u16;

    println!("f={}", cs.to_string());
    c = cs.checksum(buffer[..0]);
    println!("f={} c={}", cs.to_string(), c);
    assert!(c == 0);
    c = cs.checksum(buffer[..]);
    println!("f={} c={}", cs.to_string(), c);
    assert!(c == 0x4dae);
    cs.init();
    println!("f={}", cs.to_string());
    c = cs.checksum(buffer[..0]);
    println!("f={} c={}", cs.to_string(), c);
    assert!(c == 0);
    c = cs.checksum(buffer[..]);
    println!("f={} c={}", cs.to_string(), c);
    assert!(c == 0x4dae);
    cs.reset();
    println!("f={}", cs.to_string());
    c = cs.checksum(buffer[..0]);
    println!("f={} c={}", cs.to_string(), c);
    assert!(c == 0);
    c = cs.checksum(buffer[..]);
    println!("f={} c={}", cs.to_string(), c);
    assert!(c == 0x4dae);
    c = cs.checksum(buffer[..0]);
    println!("f={} c={}", cs.to_string(), c);
    assert!(c == 0x4dae);
}
