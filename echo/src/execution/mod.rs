
pub fn drop_guard(f: impl FnOnce()) -> impl Drop
{
    struct Dropper<F : FnOnce()>(Option<F>);
    
    impl<F : FnOnce()> Drop for Dropper<F>
    {
        fn drop(&mut self)
        {
            if let Some(f) = self.0.take()
            {
                f()
            }
        }
    }

    Dropper(Some(f))
}