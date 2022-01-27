use crate::get_state;
use ic_cdk::caller;

pub fn controller_guard() -> Result<(), String> {
    if caller() != get_state().controller {
        return Err(String::from("Access denied"));
    }

    Ok(())
}
