pub mod throttle {

    use ticks::ticks;
 
    type Events = i64;
    
    trait Throttle {
        
        fn reset(& mut self, now: ticks::Ticks);
        
        /**/
        
        fn request(& mut self, now: ticks::Ticks) -> ticks::Ticks;
        
        fn commits(& mut self, events: Events) -> bool;

        fn commit(& mut self) -> bool;
        
        fn admits(& mut self, now: ticks::Ticks, events: Events) -> bool;
        
        fn admit(& mut self, now: ticks::Ticks) -> bool;
        
        fn update(& mut self, now: ticks::Ticks) -> bool;
        
        /**/
        
        fn get_expected(& self) -> ticks::Ticks;
        
        fn is_empty(& self) -> bool;
        
        fn is_full(& self) -> bool;
        
        fn is_alarmed(& self) -> bool;
        
        /**/

        fn emptied(& self) -> bool;
        
        fn filled(& self) -> bool;
        
        fn alarmed(& self) -> bool;
        
        fn cleared(& self) -> bool;
   
    }
    
}