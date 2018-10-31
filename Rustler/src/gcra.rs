/* vi: set ts=4 expandtab shiftwidth=4: */

// Copyright 2018 by the Digital Aggregates Corporation
// Licensed under the terms in LICENSE.txt
// Author: Chip Overclock
// mailto:coverclock@diag.com
// https://github.com/coverclock/com-diag-rustler

/// Implements a Generic Cell Rate Algorithm (GCRA) using a Virtual Scheduler.
/// This can in turn be used to implement a variety of traffic shaping and rate
/// control algorithms. The VS works by monitoring the inter-arrival interval of
/// events and comparing that interval to the expected value. When the cumulative
/// error in the inter-arrival interval exceeds a threshold, the gcra becomes
/// "alarmed" and the traffic stream is in violation of its contract. In the
/// original TM spec, an event was the emission (if traffic shaping) or arrival
/// (if traffic policing) of an ATM cell, but it could be data blocks, error
/// reports, or any other kind of real-time activity. In this implementation,
/// it can even be variable length data blocks, in which the traffic contract
/// describes the mean bandwidth of the traffic stream, not the instantaneous
/// bandwidth as with ATM. In the original TM spec, the variable "i" was the
/// increment or contracted inter-arrival interval, "l" was the limit or
/// threshold, "x" was the expected inter-arrival interval for the next event,
/// and "x1" was the inter-arrival deficit accumulated so far. A gcra can
/// be used to smooth out low frequency events over a long duration, or to
/// implement a leaky bucket algorithm.
///
/// This module is based on the Go implementation in the Vamoose repository.
///
/// REFERENCES
///
/// N. Giroux et al., Traffic Management Specification Version 4.1, ATM Forum,
/// af-tm-0121.000, 1999-03
///
/// C. Overclock, "Traffic Management", 2006-12,
/// http://coverclock.blogspot.com/2006/12/traffic-management.html
///
/// C. Overclock, "Rate Control Using Throttles", 2007-01,
/// http://coverclock.blogspot.com/2007/01/rate-control-and-throttles.html
///
/// C. Overclock, "Traffic Contracts", 2007-01,
/// http://coverclock.blogspot.com/2007/01/traffic-contracts.html
///
/// J. Sloan, "ATM Traffic Management", 2005-08,
/// http://www.diag.com/reports/ATMTrafficManagement.html
///
pub mod gcra {

    use std::marker;
    use std::clone;
    use std::string;
    use std::mem;
    use std::i64;
    use ticks::ticks;
    use throttle::throttle;
  
    pub struct Gcra {
        now:        ticks::Ticks,         // Current timestamp
        then:       ticks::Ticks,         // Prior timestamp
        increment:  ticks::Ticks,         // GCRA i: ticks per event
        limit:      ticks::Ticks,         // GCRA l: maximum deficit ticks
        expected:   ticks::Ticks,         // GCRA x: expected ticks until next event
        deficit:    ticks::Ticks,         // GCRA x1: current deficit ticks
        full0:      bool,                // The leaky bucket will fill.
        full1:      bool,                // The leaky bucket is filling.
        full2:      bool,                // The leaky bucket was filled.
        empty0:     bool,                // The leaky bucket will empty.
        empty1:     bool,                // The leaky bucket is emptying.
        empty2:     bool,                // The leaky bucket was emptied.
        alarmed1:   bool,                // The gcra is alarmed.
        alarmed2:   bool,                // The gcra was alarmed.
    }
    
    impl marker::Copy for Gcra {
        /*
         * Copyable (no pointers or heap data).
         */        
    }

    impl clone::Clone for Gcra {

        fn clone(&self) -> Self {
            *self
        }

    }

    static SIZE_OF: usize = mem::size_of::<Gcra>(); // Not actually a function call.
    
    fn btoc(b: bool) -> char { if b { return '1'; } else { return '0'; } }
    
    impl string::ToString for Gcra {
      
        fn to_string(& self) -> string::String {
            format!("Gcra@{:p}[{}]:{{t:{},i:{},l:{},x:{},x1:{},f:{{{},{},{}}},e:{{{},{},{}}},a:{{{},{}}}}}",
                self, SIZE_OF,
                self.now - self.then,
                self.increment, self.limit, self.expected, self.deficit,
                btoc(self.full0), btoc(self.full1), btoc(self.full2),
                btoc(self.empty0), btoc(self.empty1), btoc(self.empty2),
                btoc(self.alarmed1), btoc(self.alarmed2))
        }
        
    }
    
    impl throttle::Throttle for Gcra {

        /***************************************************************************
         * INSPECTORS
         **************************************************************************/

        fn size_of(& self) -> usize {
            SIZE_OF
        }

        /***************************************************************************
         * CONVERTORS
         **************************************************************************/
         
        /// as_string returns the underlying object as a printable string.
        fn as_string(& self) -> string::String {
            string::ToString::to_string(self)
        }
       
        /***************************************************************************
         * SETTERS
         **************************************************************************/
    
        /// reset a throttle back to its initial state. This is used during construction,
        /// but can also be used by an application when a calamitous happenstance
        /// occurs, like the far end disconnecting and reconnecting.
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

        /***************************************************************************
         * GETTERS
         **************************************************************************/
        
        /// get_expected returns the number of ticks that would be necessary for the
        /// caller to delay for the event stream  to comply to the traffic contract with
        /// no limit penalty accumulated given the current state of the throttle. For
        /// throttles whose implementations differ from that of the Generic Cell Rate
        /// Algorithm, the value returned may be the same as that returned by Request
        /// given the current state of the throttle, or some other value entirely.
        fn get_expected(& self) -> ticks::Ticks {
            self.expected
        }
        
        /// is_empty returns true if the throttle is empty, that is, it has no accumulated
        /// deficit ticks.
        fn is_empty(& self) -> bool {
            self.empty1
        }
        
        /// is_full returns true if the throttle is full, that is, its accumulated deficit
        /// ticks is greater than or equal to its limit.
        fn is_full(& self) -> bool {
            self.full1
        }
        
        /// is_alarmed returns true if the throttle is alarmed, that is, its accumulated
        /// deficit ticks is greater than its limit, indicating that the event
        /// emission stream is out of compliance with the traffic contract.
        fn is_alarmed(& self) -> bool {
            self.alarmed1
        }

        /***************************************************************************
         * SENSORS
         **************************************************************************/
    
        /// emptied returns true if the throttle just emptied in the last action.
        fn emptied(& self) -> bool {
            self.empty1 && (!self.empty2)
        }
        
        /// filled returns true if the throttle just filled in the last action.
        fn filled(& self) -> bool {
            return self.full1 && (!self.full2)
        }
        
        /// alarmed returns true if the throttle just alarmed in the last action.
        fn alarmed(& self) -> bool {
            self.alarmed1 && (!self.alarmed2)
        }
        
        /// cleared returns true if the throttle just unalarmed in the last action,
        /// indicating that the event emission stream has returned to being
        /// compliant with the traffic contract.
        fn cleared(& self) -> bool {
            (!self.alarmed1) && self.alarmed2
        }
        
        /***************************************************************************
         * MUTATORS
         **************************************************************************/
    
        /// request computes, given the current time in ticks, how long of a delay
        /// in ticks would be necessary before the next event were emitted for that
        /// emission to be in compliance with the traffic contract.
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
            
            delay
        }
        
        /// commits updates the throttle with the number of events having been emitted
        /// starting at the time specified in the previous Request, and returns false
        /// if the throttle is alarmed, indicating the application might want to slow it
        /// down a bit, true otherwise.
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

            !self.alarmed1
        }
            
        /// commit is equivalent to calling Commits with one event.
        fn commit(& mut self) -> bool {
            self.commits(1)
        }
        
        /// admits combines calling Request with the current time in ticks with
        /// calling and returning the value of Commits with the number of events.
        fn admits(& mut self, now: ticks::Ticks, events: throttle::Events) -> bool {
            self.request(now);
            self.commits(events)
        }
        
        /// admit is equivalent to calling Admits with one event.
        fn admit(& mut self, now: ticks::Ticks) -> bool {
            self.admits(now, 1)
        }
        
        /// update is equivalent to calling Admits with zero events. It is a way to
        /// update the throttle with the current time, with no event emission. This
        /// marks the passage of time during which the emission stream is idle, which
        /// may bring the throttle back into compliance with the traffic contract (and
        /// will do so if time has advanced at least as much as the value returned by
        /// get_expected).
        fn update(& mut self, now: ticks::Ticks) -> bool {
            self.admits(now, 0) 
        }
   
    }
    
    use throttle::throttle::Throttle; // For init(): self.reset(now) below.

    impl Gcra {
        
        /// Allocate a new Gcra object with zero values for all its fields.
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
                empty0:     false,
                empty1:     false,
                empty2:     false,
                alarmed1:   false,
                alarmed2:   false,
            }
        }
         
        /// Initialize a Gcra object given an increment and limit in ticks,
        /// and the current time in ticks since the epoch.
        pub fn init(& mut self, increment: ticks::Ticks, limit: ticks::Ticks, now: ticks::Ticks) -> Self {
            self.increment = increment;
            self.limit = limit;
            self.reset(now);
            *self
        }

    }
    
    /// Compute an increment in ticks given the rate specified as the ratio of
    /// a numerator and a denominator, and the frequency.
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
        
        increment
    }
    
    /// Compute a jitter tolerance in ticks given an increment in ticks and a
    /// burst size in events.
    pub fn jittertolerance(increment: ticks::Ticks, burstsize: throttle::Events) -> ticks::Ticks {
        let mut limit: ticks::Ticks = 0;
        
        if increment <= 0 {
            // Do nothing.
        } else if burstsize <= 1 {
            // Do nothing.
        } else {
            limit = (burstsize - 1) * increment;
        }
        
        limit
    }

}
