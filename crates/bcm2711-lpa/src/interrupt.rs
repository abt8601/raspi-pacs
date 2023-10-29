#[doc = r"Enumeration of all the interrupts."]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Interrupt {
    #[doc = "0 - Software generated interrupt 0"]
    SGI0 = 0,
    #[doc = "1 - Software generated interrupt 1"]
    SGI1 = 1,
    #[doc = "2 - Software generated interrupt 2"]
    SGI2 = 2,
    #[doc = "3 - Software generated interrupt 3"]
    SGI3 = 3,
    #[doc = "4 - Software generated interrupt 4"]
    SGI4 = 4,
    #[doc = "5 - Software generated interrupt 5"]
    SGI5 = 5,
    #[doc = "6 - Software generated interrupt 6"]
    SGI6 = 6,
    #[doc = "7 - Software generated interrupt 7"]
    SGI7 = 7,
    #[doc = "8 - Software generated interrupt 8"]
    SGI8 = 8,
    #[doc = "9 - Software generated interrupt 9"]
    SGI9 = 9,
    #[doc = "10 - Software generated interrupt 10"]
    SGI10 = 10,
    #[doc = "11 - Software generated interrupt 11"]
    SGI11 = 11,
    #[doc = "12 - Software generated interrupt 12"]
    SGI12 = 12,
    #[doc = "13 - Software generated interrupt 13"]
    SGI13 = 13,
    #[doc = "14 - Software generated interrupt 14"]
    SGI14 = 14,
    #[doc = "15 - Software generated interrupt 15"]
    SGI15 = 15,
    #[doc = "62 - OR of EMMC and EMMC2"]
    EMMC = 62,
    #[doc = "96 - Timer 0 matched"]
    TIMER_0 = 96,
    #[doc = "97 - Timer 1 matched"]
    TIMER_1 = 97,
    #[doc = "98 - Timer 2 matched"]
    TIMER_2 = 98,
    #[doc = "99 - Timer 3 matched"]
    TIMER_3 = 99,
    #[doc = "105 - USB interrupt"]
    USB = 105,
    #[doc = "125 - Interrupt from AUX"]
    AUX = 125,
    #[doc = "145 - Interrupt from bank 0"]
    GPIO0 = 145,
    #[doc = "146 - Interrupt from bank 1"]
    GPIO1 = 146,
    #[doc = "147 - Interrupt from bank 2"]
    GPIO2 = 147,
    #[doc = "148 - OR of all GPIO interrupts"]
    GPIO = 148,
    #[doc = "149 - OR of all I2C interrupts"]
    I2C = 149,
    #[doc = "150 - OR of all SPI interrupts except 1 and 2"]
    SPI = 150,
    #[doc = "153 - OR of all UART interrupts except 1"]
    UART = 153,
}
#[doc = r" TryFromInterruptError"]
#[derive(Debug, Copy, Clone)]
pub struct TryFromInterruptError(());
impl Interrupt {
    #[doc = r" Attempt to convert a given value into an `Interrupt`"]
    #[inline]
    pub fn try_from(value: u8) -> Result<Self, TryFromInterruptError> {
        match value {
            0 => Ok(Interrupt::SGI0),
            1 => Ok(Interrupt::SGI1),
            2 => Ok(Interrupt::SGI2),
            3 => Ok(Interrupt::SGI3),
            4 => Ok(Interrupt::SGI4),
            5 => Ok(Interrupt::SGI5),
            6 => Ok(Interrupt::SGI6),
            7 => Ok(Interrupt::SGI7),
            8 => Ok(Interrupt::SGI8),
            9 => Ok(Interrupt::SGI9),
            10 => Ok(Interrupt::SGI10),
            11 => Ok(Interrupt::SGI11),
            12 => Ok(Interrupt::SGI12),
            13 => Ok(Interrupt::SGI13),
            14 => Ok(Interrupt::SGI14),
            15 => Ok(Interrupt::SGI15),
            62 => Ok(Interrupt::EMMC),
            96 => Ok(Interrupt::TIMER_0),
            97 => Ok(Interrupt::TIMER_1),
            98 => Ok(Interrupt::TIMER_2),
            99 => Ok(Interrupt::TIMER_3),
            105 => Ok(Interrupt::USB),
            125 => Ok(Interrupt::AUX),
            145 => Ok(Interrupt::GPIO0),
            146 => Ok(Interrupt::GPIO1),
            147 => Ok(Interrupt::GPIO2),
            148 => Ok(Interrupt::GPIO),
            149 => Ok(Interrupt::I2C),
            150 => Ok(Interrupt::SPI),
            153 => Ok(Interrupt::UART),
            _ => Err(TryFromInterruptError(())),
        }
    }
}
