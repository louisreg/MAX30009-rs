/// Minimal register interface required by the PLL driver
pub trait RegisterInterface {
    type Error;

    fn write_reg(&mut self, addr: u8, value: u8) -> Result<(), Self::Error>;
    fn delay_ms(&mut self, ms: u32);
    fn wait_for_pll_lock(&mut self) -> Result<(), Self::Error>; //will be removed
}
