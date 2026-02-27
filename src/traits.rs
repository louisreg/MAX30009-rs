use crate::device::Max30009;
use crate::register_interface::RegisterInterface;

/// Trait implemented by register wrappers that can be written to the device.
pub trait WritableRegister {
    const ADDR: u8;
    fn value(&self) -> u8;
    fn name() -> &'static str;
}

/// Trait implemented by register wrappers that can be read from the device.
pub trait ReadableRegister: Sized {
    const ADDR: u8;
    fn from_raw(v: u8) -> Self;
    fn name() -> &'static str;
}

/// Apply a full register payload (direct write path).
pub trait Apply<I: RegisterInterface> {
    fn apply(self, dev: &mut Max30009<I>) -> Result<(), I::Error>;
}

/// Read a strongly-typed configuration/status payload from registers.
pub trait Readback<I: RegisterInterface>: Sized {
    fn read_from(dev: &mut Max30009<I>) -> Result<Self, I::Error>;
}

/// Human-readable dump for debugging and bring-up traces.
pub trait DebugDump {
    fn debug_dump(&self);
}

impl<T: WritableRegister> DebugDump for T {
    fn debug_dump(&self) {
        crate::debug!(
            " {} = 0x{:02X} | b{:08b}",
            T::name(),
            self.value(),
            self.value()
        );
    }
}

/// Merge a partial update over a currently-read value.
pub trait Merge {
    fn merge(self, current: Self) -> Self;
}

/// Read-modify-write update contract.
///
/// Typical implementation:
/// 1. read current state
/// 2. merge a sparse patch
/// 3. write updated registers
pub trait Update<I: RegisterInterface> {
    fn update(self, dev: &mut Max30009<I>) -> Result<(), I::Error>;
}

impl<I, T> Update<I> for T
where
    I: RegisterInterface,
    T: Merge + Apply<I> + Readback<I> + DebugDump,
{
    fn update(self, dev: &mut Max30009<I>) -> Result<(), I::Error> {
        let current = dev.read::<T>()?;
        let merged = self.merge(current);
        dev.configure(merged)
    }
}
