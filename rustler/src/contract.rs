/* vi: set ts=4 expandtab shiftwidth=4: */

// Copyright 2018 by the Digital Aggregates Corporation
// Licensed under the terms in LICENSE.txt
// Author: Chip Overclock
// mailto:coverclock@diag.com
// https://github.com/coverclock/com-diag-rustler

pub mod contract {

    use std::string;
    use ticks::ticks;
    use throttle::throttle;
    use gcra::gcra;
  
    pub struct Contract {
        peak:       gcra::Gcra,
        sustained:  gcra::Gcra,
    }
    
    impl string::ToString for Contract {
        
        fn to_string(& self) -> String {
            format!("Contract@{:p}(p={},s={})",
                self,
                self.peak.to_string(),
                self.sustained.to_string())
        }

    }
   
    impl throttle::Throttle for Contract {
        
        fn reset(& mut self, now: ticks::Ticks) {
            self.peak.reset(now);
            self.sustained.reset(now);
        }
       
        /**/
        
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
        
        fn is_empty(& self) -> bool {
            let peak: bool = self.peak.is_empty();
            let sustained: bool = self.sustained.is_empty();
            
            return peak && sustained;
        }
        
        fn is_full(& self) -> bool {
            let peak: bool = self.peak.is_full();
            let sustained: bool = self.sustained.is_full();
            
            return peak || sustained;
        }
        
        fn is_alarmed(& self) -> bool {
            let peak: bool = self.peak.is_alarmed();
            let sustained: bool = self.sustained.is_alarmed();
            
            return peak || sustained;
        }
        
        /**/

        fn emptied(& self) -> bool {
            let peak: bool = self.peak.emptied();
            let sustained: bool = self.sustained.emptied();
            
            return peak || sustained;
        }
        
        fn filled(& self) -> bool {
            let peak: bool = self.peak.filled();
            let sustained: bool = self.sustained.filled();
            
            return peak || sustained;
        }
        
        fn alarmed(& self) -> bool {
            let peak: bool = self.peak.alarmed();
            let sustained: bool = self.sustained.alarmed();
            
            return peak || sustained;
        }
        
        fn cleared(& self) -> bool {
            let peak: bool = self.peak.cleared();
            let sustained: bool = self.sustained.cleared();
            
            return peak || sustained;
        }

        /**/
        
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
        
        fn commits(& mut self, events: throttle::Events) -> bool {
            let peak: bool = self.peak.commits(events);
            let sustained: bool = self.sustained.commits(events);
            
            return peak && sustained;
        }
            
        fn commit(& mut self) -> bool {
            let peak: bool = self.peak.commit();
            let sustained: bool = self.sustained.commit();
            
            return peak && sustained;
        }
        
        fn admits(& mut self, now: ticks::Ticks, events: throttle::Events) -> bool {
            let peak: bool = self.peak.admits(now, events);
            let sustained: bool = self.sustained.admits(now, events);
            
            return peak && sustained;
        }
        
        fn admit(& mut self, now: ticks::Ticks) -> bool {
            let peak: bool = self.peak.admit(now);
            let sustained: bool = self.sustained.admit(now);
            
            return peak && sustained;
        }
        
        fn update(& mut self, now: ticks::Ticks) -> bool {
            let peak: bool = self.peak.update(now);
            let sustained: bool = self.sustained.update(now);
            
            return peak && sustained;
        }
   
    }
    
    impl Contract {
        
        pub fn new() -> Contract {
            Contract {
                peak:       gcra::Gcra::new(),
                sustained:  gcra::Gcra::new(),
            }
        }
         
        pub fn init(& mut self, peakincrement: ticks::Ticks, jittertolerance: ticks::Ticks, sustainedincrement: ticks::Ticks, bursttolerance: ticks::Ticks, now: ticks::Ticks) {
            self.peak.init(peakincrement, jittertolerance, now);
            self.sustained.init(sustainedincrement, bursttolerance, now);
        }

    }
    
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