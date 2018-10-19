/* vi: set ts=4 expandtab shiftwidth=4: */

// Copyright 2018 by the Digital Aggregates Corporation
// Licensed under the terms in LICENSE.txt
// Author: Chip Overclock
// mailto:coverclock@diag.com
// https://github.com/coverclock/com-diag-rustler
//

pub mod gcra {

    use std::string;
    use std::i64;
    use ticks::ticks;
    use throttle::throttle;
  
    pub struct Gcra {
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
    
    impl string::ToString for Gcra {
        
        fn to_string(& self) -> String {
            format!("Gcra(T={},I={},L={},X={},X1={},F=({},{},{}),E=({},{},{}),A=({},{}))",
                self.now - self.then,
                self.increment, self.limit, self.expected, self.deficit,
                self.full0, self.full1, self.full2,
                self.empty0, self.empty1, self.empty2,
                self.alarmed1, self.alarmed2)
        }

    }
   
    impl throttle::Throttle for Gcra {
        
        fn reset(& mut self, now: ticks::Ticks) {
            self.now = now;
            self.then = self.now - self.increment;
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
        
        fn get_expected(& self) -> ticks::Ticks {
            return self.expected;
        }
        
        fn is_empty(& self) -> bool {
            return self.empty1;
        }
        
        fn is_full(& self) -> bool {
            return self.full1;
        }
        
        fn is_alarmed(& self) -> bool {
            return self.alarmed1;
        }
        
        /**/

        fn emptied(& self) -> bool {
            return self.empty1 && (!self.empty2)
        }
        
        fn filled(& self) -> bool {
            return self.full1 && (!self.full2)
        }
        
        fn alarmed(& self) -> bool {
            return self.alarmed1 && (!self.alarmed2);
        }
        
        fn cleared(& self) -> bool {
            return (!self.alarmed1) && self.alarmed2;
        }

        /**/
        
        fn request(& mut self, now: ticks::Ticks) -> ticks::Ticks {
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
            
            return delay;
        }
        
        fn commits(& mut self, events: throttle::Events) -> bool {
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
            
        fn commit(& mut self) -> bool {
            self.commits(1)
        }
        
        fn admits(& mut self, now: ticks::Ticks, events: throttle::Events) -> bool {
            self.request(now);
            self.commits(events)
        }
        
        fn admit(& mut self, now: ticks::Ticks) -> bool {
            self.admits(now, 1)
        }
        
        fn update(& mut self, now: ticks::Ticks) -> bool {
            self.admits(now, 0) 
        }
   
    }
    
    use throttle::throttle::Throttle; // For init(): self.reset(now) below.

    impl Gcra {
        
        pub fn new() -> Gcra {
            Gcra {
                now:        0,
                then:       0,
                increment:  0,
                limit:      0,
                expected:   0,
                deficit:    0,
                full0:      false,
                full1:      false,
                full2:      false,
                empty0:     true,
                empty1:     true,
                empty2:     true,
                alarmed1:   false,
                alarmed2:   false,
            }
        }
         
        pub fn init(& mut self, increment: ticks::Ticks, limit: ticks::Ticks, now: ticks::Ticks) {
            self.increment = increment;
            self.limit = limit;
            self.reset(now);
        }

    }
    
    pub fn increment(numerator: throttle::Events, denominator: throttle::Events, frequency: ticks::Ticks) -> ticks::Ticks {
        let mut increment: ticks::Ticks = 0;
        
        /*
         * rate: EVENTS/SECOND
         * numerator: EVENTS
         * denominator: SECONDS
         * frequency: TICKS/SECOND
         * increment = ( 1 / rate ) * frequency
         * increment = ( 1 / ( numerator / denominator )) * frequency
         * increment = ( denominator / numerator ) * frequency
         * increment = ( denominator * frequency ) / numerator
         * increment: TICKS/EVENT
         */

        if denominator < 1 {
            // Do nothing.
        } else if denominator == 1 {
            increment = frequency;
        } else {
            increment = frequency;
            increment *= denominator;
        }

        if numerator < 1 {
            increment = i64::max_value();
        } else  if numerator == 1 {
            // Do nothing.
        } else if (increment % numerator) == 0 {
            increment /= numerator;
        } else {
            increment /= numerator;
            increment += 1;
        }
        
        return increment;
    }
    
    pub fn jittertolerance(increment: ticks::Ticks, burstsize: throttle::Events) -> ticks::Ticks {
        let mut limit: ticks::Ticks = 0;
        
        if increment <= 0 {
            // Do nothing.
        } else if burstsize <= 1 {
            // Do nothing.
        } else {
            limit = (burstsize - 1) * increment;
        }
        
        return limit;
    }

}