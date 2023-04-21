use echo::calendar::{*, baseline_clock_driver::BaselineClockDriver};
use chrono::*;

#[test]
fn switch_clocks()
{
    let start = clock::now();

    let baseline = Box::new(BaselineClockDriver::from_local(start + Duration::days(2)));
    let now = clock::quick_switch(baseline, ||
    {
        let now = clock::now();
        now
    });

    assert_ne!(start.weekday(), now.weekday());
}