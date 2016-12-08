/// Macro for kernel-level logging.
macro_rules! klog {
    ($f:expr $(,$arg:expr)*) => {{
        device_write!($crate::DEV_SERIAL0, concat!($f, "\r\n") $(,$arg)*);
    }};
}