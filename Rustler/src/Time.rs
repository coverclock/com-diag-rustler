mod Diminuto;

mod Time {
    
    type Ticks = i64;
 
    pub fn frequency() -> Ticks {
        unsafe { Diminuto::diminuto_frequency_f() }
    }
    
    pub fn now() -> Ticks {
        unsafe { Diminuto::diminuto_time_elapsed() }
    }

    pub fn sleep(ticks: Ticks) {
        if ticks > 0 {
            unsafe { Diminuto::diminuto_delay(ticks, 0) }
        } else {
            unsafe { Diminuto::diminuto_yield() }
        }
    }

}