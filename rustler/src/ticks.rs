pub mod ticks {

    use std::time;
    use std::thread;
    use std::sync;

    type Ticks = u64;

    static INIT: sync::Once = sync::Once::new();
    static mut EPOCH: time::Instant = time::Instant::new();
     
    pub fn frequency() -> Ticks {
        return 1_000_000_000;
    }
    
    fn initialize() {
        epoch = time::Instant::now();
    }
 
    pub fn now() -> Ticks {
        unsafe { INIT.call_once(|| { EPOCH = time::Instant::now(); } ) }
        
        let now: time::Instant = time::Instant::now();
        let elapsed: time::Duration = now.duration_since(EPOCH);
        let mut ticks: Ticks = elapsed.as_secs();
        let fraction: u32 = elapsed.subsec_nanos();

        ticks *= frequency();
        ticks += fraction as u64;

        return ticks;
    }

    pub fn sleep(ticks: Ticks) {
        if ticks > 0 {
            let s: u64 = ticks / frequency();
            let ns: u64 = ticks % frequency();
            thread::sleep(time::Duration::new(s, ns as u32)); 
        } else {
            thread::yield_now();
        }
    }

}