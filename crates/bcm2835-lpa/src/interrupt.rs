#[doc = r"Enumeration of all the interrupts."]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Interrupt {
    #[doc = "0 - Timer 0 matched"]
    TIMER_0 = 0,
    #[doc = "1 - Timer 1 matched"]
    TIMER_1 = 1,
    #[doc = "2 - Timer 2 matched"]
    TIMER_2 = 2,
    #[doc = "3 - Timer 3 matched"]
    TIMER_3 = 3,
    #[doc = "9 - USB interrupt"]
    USB = 9,
    #[doc = "29 - Interrupt from AUX"]
    AUX = 29,
    #[doc = "49 - Interrupt from bank 0"]
    GPIO0 = 49,
    #[doc = "50 - Interrupt from bank 1"]
    GPIO1 = 50,
    #[doc = "51 - Interrupt from bank 2"]
    GPIO2 = 51,
    #[doc = "52 - OR of all GPIO interrupts"]
    GPIO = 52,
    #[doc = "53 - OR of all I2C interrupts"]
    I2C = 53,
    #[doc = "54 - OR of all SPI interrupts except 1 and 2"]
    SPI = 54,
    #[doc = "57 - OR of all UART interrupts except 1"]
    UART = 57,
    #[doc = "62 - OR of EMMC and EMMC2"]
    EMMC = 62,
}
#[doc = r" TryFromInterruptError"]
#[derive(Debug, Copy, Clone)]
pub struct TryFromInterruptError(());
impl Interrupt {
    #[doc = r" Attempt to convert a given value into an `Interrupt`"]
    #[inline]
    pub fn try_from(value: u8) -> Result<Self, TryFromInterruptError> {
        match value {
            0 => Ok(Interrupt::TIMER_0),
            1 => Ok(Interrupt::TIMER_1),
            2 => Ok(Interrupt::TIMER_2),
            3 => Ok(Interrupt::TIMER_3),
            9 => Ok(Interrupt::USB),
            29 => Ok(Interrupt::AUX),
            49 => Ok(Interrupt::GPIO0),
            50 => Ok(Interrupt::GPIO1),
            51 => Ok(Interrupt::GPIO2),
            52 => Ok(Interrupt::GPIO),
            53 => Ok(Interrupt::I2C),
            54 => Ok(Interrupt::SPI),
            57 => Ok(Interrupt::UART),
            62 => Ok(Interrupt::EMMC),
            _ => Err(TryFromInterruptError(())),
        }
    }
}
