#[cfg(debug_assertions)]
pub fn log(msg: &str) {
    unsafe { crate::include::DbgPrintEx(0, 0, msg.as_ptr() as _) };
}

#[cfg(not(debug_assertions))]
pub fn log(_msg: &str) {}
