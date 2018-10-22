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
    let buffer: [u8; 6] = [ b'q', b'w', b'e', b'r', b't', b'y' ];
    let mut cs: fletcher::Fletcher = fletcher::Fletcher::new();
    let mut c: u16;

    println!("f={}", cs.to_string());
    c = cs.checksum(&buffer[..0]);
    println!("f={} c={}", cs.to_string(), c);
    assert!(c == 0);
    c = cs.checksum(&buffer[..]);
    println!("f={} c={}", cs.to_string(), c);
    assert!(c == 0x4dae);
    c = cs.checksum(&buffer[..0]);
    println!("f={} c={}", cs.to_string(), c);
    assert!(c == 0x4dae);
    /**/
    cs.init(0, 0);
    println!("f={}", cs.to_string());
    c = cs.checksum(&buffer[..0]);
    println!("f={} c={}", cs.to_string(), c);
    assert!(c == 0);
    c = cs.checksum(&buffer[..]);
    println!("f={} c={}", cs.to_string(), c);
    assert!(c == 0x4dae);
    c = cs.checksum(&buffer[..0]);
    println!("f={} c={}", cs.to_string(), c);
    assert!(c == 0x4dae);
    /**/
    cs.reset();
    println!("f={}", cs.to_string());
    c = cs.checksum(&buffer[..0]);
    println!("f={} c={}", cs.to_string(), c);
    assert!(c == 0);
    c = cs.checksum(&buffer[..]);
    println!("f={} c={}", cs.to_string(), c);
    assert!(c == 0x4dae);
    c = cs.checksum(&buffer[..0]);
    println!("f={} c={}", cs.to_string(), c);
    assert!(c == 0x4dae);
}
