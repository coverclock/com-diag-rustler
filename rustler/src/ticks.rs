/* vi: set ts=4 expandtab shiftwidth=4: */

// Copyright 2018 by the Digital Aggregates Corporation
// Licensed under the terms in LICENSE.txt
// Author: Chip Overclock
// mailto:coverclock@diag.com
// https://github.com/coverclock/com-diag-rustler
//

pub mod ticks {

    use std::sync;
    use std::option;
    use std::time;
    use std::thread;

    pub type Ticks = u64;

    static INIT: sync::Once = sync::Once::new();
    static mut EPOCH: option::Option<time::Instant> = option::Option::None;

    pub fn frequency() -> Ticks {
        1_000_000_000
    }
 
    pub fn now() -> Ticks {
        unsafe {
            INIT.call_once(|| { EPOCH = option::Option::Some(time::Instant::now()); } );
            let then: time::Instant = EPOCH.unwrap();
            let now: time::Instant = time::Instant::now();
            let elapsed: time::Duration = now.duration_since(then);
            let mut ticks: Ticks = elapsed.as_secs();
            let fraction: u32 = elapsed.subsec_nanos();

            ticks *= frequency();
            ticks += fraction as u64;
            
            return ticks;
        }
    }

    pub fn sleep(ticks: Ticks) {
        if ticks > 0 {
            let s: u64 = ticks / frequency();
            let ns: u64 = ticks % frequency();
            thread::sleep(time::Duration::new(s, ns as u32)); 
        } else {
            thread::yield_now();
        }
    }

}
