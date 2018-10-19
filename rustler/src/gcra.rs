/* vi: set ts=4 expandtab shiftwidth=4: */

// Copyright 2018 by the Digital Aggregates Corporation
// Licensed under the terms in LICENSE.txt
// Author: Chip Overclock
// mailto:coverclock@diag.com
// https://github.com/coverclock/com-diag-rustler
//

pub mod gcra {

    use std::string::ToString;
    use ticks::ticks;
    use throttle::throttle;
  
    struct Gcra {
        now:        ticks::Ticks,
        then:       ticks::Ticks,
        increment:  ticks::Ticks,
        limit:      ticks::Ticks,
        expected:   ticks::Ticks,
        deficit:    ticks::Ticks,
        full0:      bool,
        full1:      bool,
        full2:      bool,
        empty0:     bool,
        empty1:     bool,
        empty2:     bool,
        alarmed1:   bool,
        alarmed2:   bool,
    }
    
    impl ToString for Gcra {
        
        pub fn to_string(& self) -> String {
            format!("Gcra(T={},I={},L={},X={},X1={},F=({},{},{}),E=({},{},{}),A=({},{}))",
                self.now - self.then,
                self.increment, self.limit, self.expected, self.deficit,
                self.full0, self.full1, self.full2,
                self.empty0, self.empty1, self.empty2,
                self.alarmed1, self.alarmed2);
        }

    }
   
    impl Throttle for Gcra {
        
        pub fn reset(& mut self, now: ticks::Ticks) {
            self.now = now;
            self.then = this.now - self.increment;
            self.expected = 0;
            self.deficit = 0;
            self.full0 = false;
            self.full1 = false;
            self.full2 = false;
            self.empty0 = true;
            self.empty1 = true;
            self.empty2 = true;
            self.alarmed1 = false;
            self.alarmed2 = false;         
        }
        
        /**/
        
        pub fn get_expected(& self) -> ticks::Ticks {
            return self.expected;
        }
        
        pub fn is_empty(& self) -> bool {
            return self.empty1;
        }
        
        pub fn is_full(& self) -> bool {
            return self.full1;
        }
        
        pub fn is_alarmed(& self) -> bool {
            return self.alarmed1;
        }
        
        /**/

        pub fn emptied(& self) -> bool {
            return self.empty1 && (!self.empty2)
        }
        
        pub fn filled(& self) -> bool {
            return self.full1 && (!self.full2)
        }
        
        pub fn alarmed(& self) -> bool {
            return self.alarmed1 && (!self.alarmed2);
        }
        
        pub fn cleared(& self) -> bool {
            return (!self.alarmed1) && self.alarmed2;
        }

        /**/
        
        pub fn request(& mut self, now: ticks::Ticks) -> ticks::Ticks {
            let delay: ticks::Ticks;
            let elapsed: ticks::Ticks;
            
            self.now = now;
            elapsed = self.now - self.then;
            if self.expected <= elapsed {
                self.deficit = 0;
                self.full0 = false;
                self.empty0 = true;
                delay = 0;
            } else {
                self.deficit = self.expected - elapsed;
                if self.deficit <= self.limit {
                    self.full0 = false;
                    self.empty0 = false;
                    delay = 0;
                } else {
                    self.full0 = true;
                    self.empty0 = false;
                    delay = self.deficit - self.limit;
                }
            }
        }
        
        pub fn commits(& mut self, events: Events) -> bool {
            self.then = self.now;
            self.expected = self.deficit;
            if events <= 0 {
                // Do nothing.
            } else if events == 1 {
                self.expected += self.increment;
            } else {
                self.expected += self.increment * events;
            }
            self.full2 = self.full1;
            self.full1 = self.full0;
            self.empty2 = self.empty1;
            self.empty1 = self.empty0;
            self.alarmed2 = self.alarmed1;
            if self.emptied() {
                self.alarmed1 = false;
            } else if self.filled() {
                self.alarmed1 = true;
            } else {
                // Do nothing.
            }
            return !self.alarmed1;
        }
            
        pub fn commit(& mut self) -> bool {
            self.commits(1)
        }
        
        pub fn admits(& mut self, now: ticks::Ticks, events: throttle::Events) -> bool {
            self.request(now);
            self.commits(events);
        }
        
        pub fn admit(& mut self, now: ticks::Ticks) -> bool {
            self.admits(now, 1);
        }
        
        pub fn update(& mut self, now: ticks::Ticks) -> bool {
            self.admits(now, 0) 
        }
   
    }
        
    impl Gcra {
        
        pub fn init(& mut self, increment: ticks::Ticks, limit: ticks::Ticks, now: ticks::Ticks) {
            self.increment = increment;
            self.limit = limit;
            reset(now);
        }
        
        pub fn new(increment: ticks::Ticks, limit: ticks::Ticks, now: ticks::Ticks) -> Gcra {
            let mut gcra = Gcra::new();
            gcra.init(increment, limit, now);
            return gcra;
       }

    }
    
    pub fn increment(numerator: throttle::Events, denominator: throttle::Events, frequency: ticks::Ticks) -> ticks::Ticks {
        let i: ticks::Ticks;
        let n: throttle::Events = numerator;
        let d: throttle::Events = denominator;
        
        i = frequency;
        if d > 1 {
            i *= d;
        }
        if n <= 1 {
            // Do nothing.
        } else if (i % n) > 0 {
            i /= n;
            i += 1;
        } else {
            i /= n;
        }
    }
    
    pub fn jittertolerance(peak: ticks::Ticks, burstsize: throttle::Events) -> ticks::Ticks {
        let l: ticks::Ticks;
        
        if burstsize > 1 {
            l = (burstsize - 1) * peak;
        } else {
            l = 0;
        }
    }

}