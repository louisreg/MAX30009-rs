/// Hardware abstraction used by the MAX30009 driver.
///
/// This trait is intentionally low-level and register-oriented so it can be
/// implemented over SPI, I2C, or mock backends for tests.
pub trait RegisterInterface {
    /// Backend-specific error type.
    type Error;

    /// Read one register at `addr`.
    fn read_reg(&mut self, addr: u8) -> Result<u8, Self::Error>;

    /// Burst-read starting at `addr` into `buf`.
    fn read_reg_burst(&mut self, addr: u8, buf: &mut [u8]) -> Result<(), Self::Error>;

    /// Write one register at `addr`.
    fn write_reg(&mut self, addr: u8, value: u8) -> Result<(), Self::Error>;
}
