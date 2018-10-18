use std::time::Duration;
use std::time::Instant;
use std::thread;

mod ticks {
    
    type Ticks = u64;
    
    static epoch: Instant = Instant::now();
 
    pub fn frequency() -> Ticks {
        return 1_000_000_000;
    }
    
    pub fn now() -> Ticks {
        let now: Instant = Instant::now();
        let elapsed: Duration = now.duration_since(epoch);
        let ticks: Ticks = elapsed.as_secs();
        let fraction: u32 = elapsed.subsec_nanos();

        ticks *= frequency();
        ticks += fraction as u64;

        return ticks;
    }

    pub fn sleep(ticks: Ticks) {
        if ticks > 0 {
            thread::sleep(Duration::new(ticks / frequency(), ticks % frequency())); 
        } else {
            thread::yield_now();
        }
    }

}