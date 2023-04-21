use echo::calendar::*;
use echo::calendar::system_clock_driver::*;

#[test]
fn initialization() 
{
    let clock = SystemClockDriver::new();
    let _utc_now = clock.utc_now();
    let _now = clock.now();
}