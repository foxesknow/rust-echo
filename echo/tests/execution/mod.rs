
use echo::execution;

#[test]
fn guard_runs()
{
    let mut counter = 0;

    {
        let _guard = execution::drop_guard(|| counter = counter + 1);
        //assert_eq!(counter, 0);
    }

    assert_eq!(counter, 1);
}