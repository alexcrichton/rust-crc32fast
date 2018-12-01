#[cfg(all(
    crc32fast_stdarchx86,
    any(ctarget_arch = "x86", target_arch = "x86_64")
))]
mod pclmulqdq;
#[cfg(all(
    crc32fast_stdarchx86,
    any(ctarget_arch = "x86", target_arch = "x86_64")
))]
pub use self::pclmulqdq::State;

#[cfg(not(all(
    crc32fast_stdarchx86,
    any(ctarget_arch = "x86", target_arch = "x86_64")
)))]
#[derive(Clone)]
pub enum State {}
#[cfg(not(all(
    crc32fast_stdarchx86,
    any(ctarget_arch = "x86", target_arch = "x86_64")
)))]
impl State {
    pub fn new() -> Option<Self> {
        None
    }

    pub fn update(&mut self, _buf: &[u8]) {
        match *self {}
    }

    pub fn finalize(self) -> u32 {
        match self{}
    }

    pub fn reset(&mut self) {
        match *self {}
    }

    pub fn combine(&mut self, _other: u32, _amount: u64) {
        match *self {}
    }
}
