use std::time::Duration;

pub const ALIVE_TIMEOUT: u64 = 20; // 20ms
pub const SEND_TIMEOUT: Duration = Duration::from_millis(2u64); // 2ms
pub const INIT_ITER: Duration = Duration::from_millis(1); // 1ms
pub const RECV_TIMEOUT: Duration = Duration::from_millis(5);