/* vi: set ts=4 expandtab shiftwidth=4: */

// Copyright 2018 by the Digital Aggregates Corporation
// Licensed under the terms in LICENSE.txt
// Author: Chip Overclock
// mailto:coverclock@diag.com
// https://github.com/coverclock/com-diag-rustler

/// Basic time-related functions for use in Rustler.
///
pub mod ticks {

    use std::sync;
    use std::option;
    use std::time;
    use std::thread;

    /// ticks is a type big enough to contain a monotonic elapsed time value.
    pub type Ticks = i64;

    static INIT: sync::Once = sync::Once::new();
    static mut EPOCH: option::Option<time::Instant> = option::Option::None;
    
    pub const FREQUENCY: Ticks = 1_000_000_000;

    /// frequency returns the resolution of the time value that can be stored in a
    /// Tick in units of cycles per second or Hertz.
    pub fn frequency() -> Ticks {
        FREQUENCY
    }
 
    /// now returns the current value of Ticks for the monotonically increasing time
    /// that is now.
    pub fn now() -> Ticks {
        unsafe {
            INIT.call_once(|| { EPOCH = option::Option::Some(time::Instant::now()); } );
            let then: time::Instant = EPOCH.unwrap();
            let now: time::Instant = time::Instant::now();
            let elapsed: time::Duration = now.duration_since(then);
            let seconds: u64 = elapsed.as_secs();
            let nanoseconds: u32 = elapsed.subsec_nanos();
            let mut ticks: Ticks;

            ticks = seconds as i64;
            ticks *= frequency();
            ticks += nanoseconds as i64;
            
            return ticks;
        }
    }

    /// sleep delays the caller for at least as many ticks as specified. If zero
    /// ticks are specified, the caller yields the processor.
    pub fn sleep(ticks: Ticks) {
        if ticks > 0 {
            let s: Ticks = ticks / frequency();
            let ns: Ticks = ticks % frequency();
            thread::sleep(time::Duration::new(s as u64, ns as u32)); 
        } else {
            thread::yield_now();
        }
    }

}
