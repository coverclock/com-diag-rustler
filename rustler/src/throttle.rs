use ticks::ticks;

mod throttle {
    
    type Events = i64;
    
    pub trait Throttle {
        
        pub fn reset(& mut self, now: ticks::Ticks);
        
        /**/
        
        pub fn request(& mut self, now: ticks::Ticks) -> ticks::Ticks;
        
        pub fn commits(& mut self, events: Events) -> bool;

        pub fn commit(& mut self) -> bool;
        
        pub fn admits(& mut self, now: ticks::Ticks, events: Events) -> bool;
        
        pub fn admit(& mut self, now: ticks::Ticks) -> bool;
        
        pub fn update(& mut self, now: ticks::Ticks) -> bool;
        
        /**/
        
        pub fn get_expected(& self) -> ticks::Ticks;
        
        pub fn is_empty(& self) -> bool;
        
        pub fn is_full(& self) -> bool;
        
        pub fn is_alarmed(& self) -> bool;
        
        /**/

        pub fn emptied(& self) -> bool;
        
        pub fn filled(& self) -> bool;
        
        pub fn alarmed(& self) -> bool;
        
        pub fn cleared(& self) -> bool;
   
    }
    
}