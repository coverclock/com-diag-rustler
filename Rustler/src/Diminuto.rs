#[link(name = "diminuto")]
extern {
    
    fn diminuto_frequency_f() -> i64;
    
    fn diminuto_time_elapsed() -> i64;
    
    fn diminuto_delay(ticks: i64, interruptible: int) -> i64;
    
    fn diminuto_yield() -> int;
    
}
