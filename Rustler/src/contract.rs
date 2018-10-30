/* vi: set ts=4 expandtab shiftwidth=4: */

// Copyright 2018 by the Digital Aggregates Corporation
// Licensed under the terms in LICENSE.txt
// Author: Chip Overclock
// mailto:coverclock@diag.com
// https://github.com/coverclock/com-diag-rustler

/// Implements a traffic contract that is a composite of two GCRAs: one that
/// describes the peak rate, and one that describes the sustainable rate. The
/// event stream must conform to both GCRAs. The interface still appears to be
/// a single GCRA from the point of view of the calling application. The
/// implementation consists of two gcras, one for the peak GCRA, the other
/// for the sustained GCRA. The peak gcra contains the peak increment, and
/// the peak limit that is the jitter tolerance. The sustained gcra contains
/// the sustained increment, and the sustained limit computed from maximum burst
/// size and the jitter tolerance.
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
pub mod contract {

    use std::marker;
    use std::clone;
    use std::string;
    use std::mem;
    use ticks::ticks;
    use throttle::throttle;
    use gcra::gcra;
  
    pub struct Contract {
        peak:       gcra::Gcra,
        sustained:  gcra::Gcra,
    }
    
    impl marker::Copy for Contract {
         
        /***************************************************************************
         * COPIERS
         **************************************************************************/
        
    }

    impl clone::Clone for Contract {
         
        /***************************************************************************
         * CLONERS
         **************************************************************************/

        fn clone(&self) -> Contract {
            *self
        }

    }

    static SIZE_OF: usize = mem::size_of::<Contract>();
    
    impl string::ToString for Contract {
        
        /***************************************************************************
         * CONVERTORS
         **************************************************************************/
        
        fn to_string(& self) -> string::String {
            format!("Contract@{:p}[{}]:{{p:{},s:{}}}",
                self, SIZE_OF,
                self.peak.to_string(),
                self.sustained.to_string())
        }
        
    }
   
    impl throttle::Throttle for Contract {
        
        /***************************************************************************
         * SETTERS
         **************************************************************************/
    
        /// reset a throttle back to its initial state. This is used during construction,
        /// but can also be used by an application when a calamitous happenstance
        /// occurs, like the far end disconnecting and reconnecting.        
        fn reset(& mut self, now: ticks::Ticks) {
            self.peak.reset(now);
            self.sustained.reset(now);
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
            let delay: ticks::Ticks;
            let peak: ticks::Ticks = self.peak.get_expected();
            let sustained: ticks::Ticks = self.sustained.get_expected();
            
            if peak > sustained {
                delay = peak;
            } else {
                delay = sustained;
            }
            
            return delay;
        }
        
        /// is_empty returns true if the throttle is empty, that is, it has no accumulated
        /// deficit ticks.
        fn is_empty(& self) -> bool {
            let peak: bool = self.peak.is_empty();
            let sustained: bool = self.sustained.is_empty();
            
            return peak && sustained;
        }
        
        /// is_full returns true if the throttle is full, that is, its accumulated deficit
        /// ticks is greater than or equal to its limit.
        fn is_full(& self) -> bool {
            let peak: bool = self.peak.is_full();
            let sustained: bool = self.sustained.is_full();
            
            return peak || sustained;
        }
        
        /// is_alarmed returns true if the throttle is alarmed, that is, its accumulated
        /// deficit ticks is greater than its limit, indicating that the event
        /// emission stream is out of compliance with the traffic contract.
        fn is_alarmed(& self) -> bool {
            let peak: bool = self.peak.is_alarmed();
            let sustained: bool = self.sustained.is_alarmed();
            
            return peak || sustained;
        }

        /***************************************************************************
         * SENSORS
         **************************************************************************/
    
        /// emptied returns true if the throttle just emptied in the last action.
        fn emptied(& self) -> bool {
            let peak: bool = self.peak.emptied();
            let sustained: bool = self.sustained.emptied();
            
            return peak || sustained;
        }
        
        /// filled returns true if the throttle just filled in the last action.
        fn filled(& self) -> bool {
            let peak: bool = self.peak.filled();
            let sustained: bool = self.sustained.filled();
            
            return peak || sustained;
        }
        
        /// alarmed returns true if the throttle just alarmed in the last action.
        fn alarmed(& self) -> bool {
            let peak: bool = self.peak.alarmed();
            let sustained: bool = self.sustained.alarmed();
            
            return peak || sustained;
        }
        
        /// cleared returns true if the throttle just unalarmed in the last action,
        /// indicating that the event emission stream has returned to being
        /// compliant with the traffic contract.
        fn cleared(& self) -> bool {
            let peak: bool = self.peak.cleared();
            let sustained: bool = self.sustained.cleared();
            
            return peak || sustained;
        }
        
        /***************************************************************************
         * MUTATORS
         **************************************************************************/
    
        /// request computes, given the current time in ticks, how long of a delay
        /// in ticks would be necessary before the next event were emitted for that
        /// emission to be in compliance with the traffic contract.
        fn request(& mut self, now: ticks::Ticks) -> ticks::Ticks {
            let delay: ticks::Ticks;
            let peak: ticks::Ticks = self.peak.request(now);
            let sustained: ticks::Ticks = self.sustained.request(now);
            
            if peak > sustained {
                delay = peak;
            } else {
                delay = sustained;
            }
            
            return delay;
        }
        
        /// commits updates the throttle with the number of events having been emitted
        /// starting at the time specified in the previous Request, and returns false
        /// if the throttle is alarmed, indicating the application might want to slow it
        /// down a bit, true otherwise.
        fn commits(& mut self, events: throttle::Events) -> bool {
            let peak: bool = self.peak.commits(events);
            let sustained: bool = self.sustained.commits(events);
            
            return peak && sustained;
        }
            
        /// commit is equivalent to calling Commits with one event.
        fn commit(& mut self) -> bool {
            let peak: bool = self.peak.commit();
            let sustained: bool = self.sustained.commit();
            
            return peak && sustained;
        }
        
        /// admits combines calling Request with the current time in ticks with
        /// calling and returning the value of Commits with the number of events.
        fn admits(& mut self, now: ticks::Ticks, events: throttle::Events) -> bool {
            let peak: bool = self.peak.admits(now, events);
            let sustained: bool = self.sustained.admits(now, events);
            
            return peak && sustained;
        }
        
        /// admit is equivalent to calling Admits with one event.
        fn admit(& mut self, now: ticks::Ticks) -> bool {
            let peak: bool = self.peak.admit(now);
            let sustained: bool = self.sustained.admit(now);
            
            return peak && sustained;
        }
        
        /// update is equivalent to calling Admits with zero events. It is a way to
        /// update the throttle with the current time, with no event emission. This
        /// marks the passage of time during which the emission stream is idle, which
        /// may bring the throttle back into compliance with the traffic contract (and
        /// will do so if time has advanced at least as much as the value returned by
        /// get_expected).
        fn update(& mut self, now: ticks::Ticks) -> bool {
            let peak: bool = self.peak.update(now);
            let sustained: bool = self.sustained.update(now);
            
            return peak && sustained;
        }
   
    }
    
    impl Contract {
         
        pub fn size_of(& self) -> usize {
            SIZE_OF
        }
       
        /// Allocate a new Contract object with zero values for all its fields.
        pub fn new() -> Contract {
            Contract {
                peak:       gcra::Gcra::new(),
                sustained:  gcra::Gcra::new(),
            }
        }
         
        /// Initialize a Contract object given an peak increment and jitter
        /// tolerance in ticks, the sustained increment and burst tolerance
        /// in ticks, and the current time in ticks since the epoch.
        pub fn init(& mut self, peakincrement: ticks::Ticks, jittertolerance: ticks::Ticks, sustainedincrement: ticks::Ticks, bursttolerance: ticks::Ticks, now: ticks::Ticks) {
            self.peak.init(peakincrement, jittertolerance, now);
            self.sustained.init(sustainedincrement, bursttolerance, now);
        }

    }
    
    /// Compute the burst tolerance in ticks given the peak increment, jitter
    /// tolerance, and sustained increment in ticks, and the maximum burst
    /// size in events.
    pub fn bursttolerance(peakincrement: ticks::Ticks, jittertolerance: ticks::Ticks, sustainedincrement: ticks::Ticks, burstsize: throttle::Events) -> ticks::Ticks {
        let mut limit: ticks::Ticks = jittertolerance;
        
        if peakincrement >= sustainedincrement {
            // Do nothing.
        } else if burstsize <= 1 {
            // Do nothing.
        } else {
            limit += (burstsize - 1) * (sustainedincrement - peakincrement);
        }
        
        return limit;
    }

}
