use Time;

mod Throttle {
    
    type Events = i64;
    
    pub trait Throttle {
        
        pub fn reset(& mut self, now: Time::Ticks);
        
        /**/
        
        pub fn request(& mut self, now: Time::Ticks) -> Time::Ticks;
        
        pub fn commits(& mut self, events: Events) -> bool;

        pub fn commit(& mut self) -> bool;
        
        pub fn admits(& mut self, now: Time::Ticks, events: Events) -> bool;
        
        pub fn admit(& mut self, now: Time::Ticks) -> bool;
        
        pub fn update(& mut self, now: Time::Ticks) -> bool;
        
        /**/
        
        pub fn expected(& self) -> Time::Ticks;
        
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