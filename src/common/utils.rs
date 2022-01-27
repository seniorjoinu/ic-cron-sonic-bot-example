use ic_cdk::trap;
use std::fmt::Debug;

pub trait UnwrapOrTrap<T> {
    fn unwrap_or_trap(self, msg: &str) -> T;
}

impl<T, E> UnwrapOrTrap<T> for Result<T, E>
where
    E: Debug,
{
    fn unwrap_or_trap(self, msg: &str) -> T {
        self.unwrap_or_else(|e| trap(format!("{:?}: {:?}", msg, e).as_str()))
    }
}
