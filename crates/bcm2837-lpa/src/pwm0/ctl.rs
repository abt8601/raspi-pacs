#[doc = "Register `CTL` reader"]
pub type R = crate::R<CTL_SPEC>;
#[doc = "Register `CTL` writer"]
pub type W = crate::W<CTL_SPEC>;
#[doc = "Field `PWEN1` reader - Enable channel 1"]
pub type PWEN1_R = crate::BitReader;
#[doc = "Field `PWEN1` writer - Enable channel 1"]
pub type PWEN1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MODE1` reader - Channel 1 mode"]
pub type MODE1_R = crate::BitReader<MODE1_A>;
#[doc = "Channel 1 mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MODE1_A {
    #[doc = "0: `0`"]
    PWM = 0,
    #[doc = "1: `1`"]
    SERIAL = 1,
}
impl From<MODE1_A> for bool {
    #[inline(always)]
    fn from(variant: MODE1_A) -> Self {
        variant as u8 != 0
    }
}
impl MODE1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MODE1_A {
        match self.bits {
            false => MODE1_A::PWM,
            true => MODE1_A::SERIAL,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_pwm(&self) -> bool {
        *self == MODE1_A::PWM
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_serial(&self) -> bool {
        *self == MODE1_A::SERIAL
    }
}
#[doc = "Field `MODE1` writer - Channel 1 mode"]
pub type MODE1_W<'a, REG> = crate::BitWriter<'a, REG, MODE1_A>;
impl<'a, REG> MODE1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn pwm(self) -> &'a mut crate::W<REG> {
        self.variant(MODE1_A::PWM)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn serial(self) -> &'a mut crate::W<REG> {
        self.variant(MODE1_A::SERIAL)
    }
}
#[doc = "Field `RPTL1` reader - Repeat last value from FIFO for channel 1"]
pub type RPTL1_R = crate::BitReader;
#[doc = "Field `RPTL1` writer - Repeat last value from FIFO for channel 1"]
pub type RPTL1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SBIT1` reader - State when not transmitting on channel 1"]
pub type SBIT1_R = crate::BitReader;
#[doc = "Field `SBIT1` writer - State when not transmitting on channel 1"]
pub type SBIT1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POLA1` reader - Channel 1 polarity inverted"]
pub type POLA1_R = crate::BitReader;
#[doc = "Field `POLA1` writer - Channel 1 polarity inverted"]
pub type POLA1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USEF1` reader - Use FIFO for channel 1"]
pub type USEF1_R = crate::BitReader;
#[doc = "Field `USEF1` writer - Use FIFO for channel 1"]
pub type USEF1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLRF1` reader - Clear FIFO"]
pub type CLRF1_R = crate::BitReader;
#[doc = "Field `CLRF1` writer - Clear FIFO"]
pub type CLRF1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MSEN1` reader - M/S mode for channel 1"]
pub type MSEN1_R = crate::BitReader;
#[doc = "Field `MSEN1` writer - M/S mode for channel 1"]
pub type MSEN1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWEN2` reader - Enable channel 2"]
pub type PWEN2_R = crate::BitReader;
#[doc = "Field `PWEN2` writer - Enable channel 2"]
pub type PWEN2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MODE2` reader - Channel 2 mode"]
pub type MODE2_R = crate::BitReader<MODE2_A>;
#[doc = "Channel 2 mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MODE2_A {
    #[doc = "0: `0`"]
    PWM = 0,
    #[doc = "1: `1`"]
    SERIAL = 1,
}
impl From<MODE2_A> for bool {
    #[inline(always)]
    fn from(variant: MODE2_A) -> Self {
        variant as u8 != 0
    }
}
impl MODE2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MODE2_A {
        match self.bits {
            false => MODE2_A::PWM,
            true => MODE2_A::SERIAL,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_pwm(&self) -> bool {
        *self == MODE2_A::PWM
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_serial(&self) -> bool {
        *self == MODE2_A::SERIAL
    }
}
#[doc = "Field `MODE2` writer - Channel 2 mode"]
pub type MODE2_W<'a, REG> = crate::BitWriter<'a, REG, MODE2_A>;
impl<'a, REG> MODE2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn pwm(self) -> &'a mut crate::W<REG> {
        self.variant(MODE2_A::PWM)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn serial(self) -> &'a mut crate::W<REG> {
        self.variant(MODE2_A::SERIAL)
    }
}
#[doc = "Field `RPTL2` reader - Repeat last value from FIFO for channel 2"]
pub type RPTL2_R = crate::BitReader;
#[doc = "Field `RPTL2` writer - Repeat last value from FIFO for channel 2"]
pub type RPTL2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SBIT2` reader - State when not transmitting on channel 2"]
pub type SBIT2_R = crate::BitReader;
#[doc = "Field `SBIT2` writer - State when not transmitting on channel 2"]
pub type SBIT2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POLA2` reader - Channel 2 polarity inverted"]
pub type POLA2_R = crate::BitReader;
#[doc = "Field `POLA2` writer - Channel 2 polarity inverted"]
pub type POLA2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USEF2` reader - Use FIFO for channel 2"]
pub type USEF2_R = crate::BitReader;
#[doc = "Field `USEF2` writer - Use FIFO for channel 2"]
pub type USEF2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MSEN2` reader - M/S mode for channel 2"]
pub type MSEN2_R = crate::BitReader;
#[doc = "Field `MSEN2` writer - M/S mode for channel 2"]
pub type MSEN2_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable channel 1"]
    #[inline(always)]
    pub fn pwen1(&self) -> PWEN1_R {
        PWEN1_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel 1 mode"]
    #[inline(always)]
    pub fn mode1(&self) -> MODE1_R {
        MODE1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Repeat last value from FIFO for channel 1"]
    #[inline(always)]
    pub fn rptl1(&self) -> RPTL1_R {
        RPTL1_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - State when not transmitting on channel 1"]
    #[inline(always)]
    pub fn sbit1(&self) -> SBIT1_R {
        SBIT1_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Channel 1 polarity inverted"]
    #[inline(always)]
    pub fn pola1(&self) -> POLA1_R {
        POLA1_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Use FIFO for channel 1"]
    #[inline(always)]
    pub fn usef1(&self) -> USEF1_R {
        USEF1_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Clear FIFO"]
    #[inline(always)]
    pub fn clrf1(&self) -> CLRF1_R {
        CLRF1_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - M/S mode for channel 1"]
    #[inline(always)]
    pub fn msen1(&self) -> MSEN1_R {
        MSEN1_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable channel 2"]
    #[inline(always)]
    pub fn pwen2(&self) -> PWEN2_R {
        PWEN2_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Channel 2 mode"]
    #[inline(always)]
    pub fn mode2(&self) -> MODE2_R {
        MODE2_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Repeat last value from FIFO for channel 2"]
    #[inline(always)]
    pub fn rptl2(&self) -> RPTL2_R {
        RPTL2_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - State when not transmitting on channel 2"]
    #[inline(always)]
    pub fn sbit2(&self) -> SBIT2_R {
        SBIT2_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Channel 2 polarity inverted"]
    #[inline(always)]
    pub fn pola2(&self) -> POLA2_R {
        POLA2_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Use FIFO for channel 2"]
    #[inline(always)]
    pub fn usef2(&self) -> USEF2_R {
        USEF2_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 15 - M/S mode for channel 2"]
    #[inline(always)]
    pub fn msen2(&self) -> MSEN2_R {
        MSEN2_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTL")
            .field("msen2", &format_args!("{}", self.msen2().bit()))
            .field("usef2", &format_args!("{}", self.usef2().bit()))
            .field("pola2", &format_args!("{}", self.pola2().bit()))
            .field("sbit2", &format_args!("{}", self.sbit2().bit()))
            .field("rptl2", &format_args!("{}", self.rptl2().bit()))
            .field("mode2", &format_args!("{}", self.mode2().bit()))
            .field("pwen2", &format_args!("{}", self.pwen2().bit()))
            .field("msen1", &format_args!("{}", self.msen1().bit()))
            .field("clrf1", &format_args!("{}", self.clrf1().bit()))
            .field("usef1", &format_args!("{}", self.usef1().bit()))
            .field("pola1", &format_args!("{}", self.pola1().bit()))
            .field("sbit1", &format_args!("{}", self.sbit1().bit()))
            .field("rptl1", &format_args!("{}", self.rptl1().bit()))
            .field("mode1", &format_args!("{}", self.mode1().bit()))
            .field("pwen1", &format_args!("{}", self.pwen1().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<CTL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - Enable channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn pwen1(&mut self) -> PWEN1_W<CTL_SPEC> {
        PWEN1_W::new(self, 0)
    }
    #[doc = "Bit 1 - Channel 1 mode"]
    #[inline(always)]
    #[must_use]
    pub fn mode1(&mut self) -> MODE1_W<CTL_SPEC> {
        MODE1_W::new(self, 1)
    }
    #[doc = "Bit 2 - Repeat last value from FIFO for channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn rptl1(&mut self) -> RPTL1_W<CTL_SPEC> {
        RPTL1_W::new(self, 2)
    }
    #[doc = "Bit 3 - State when not transmitting on channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn sbit1(&mut self) -> SBIT1_W<CTL_SPEC> {
        SBIT1_W::new(self, 3)
    }
    #[doc = "Bit 4 - Channel 1 polarity inverted"]
    #[inline(always)]
    #[must_use]
    pub fn pola1(&mut self) -> POLA1_W<CTL_SPEC> {
        POLA1_W::new(self, 4)
    }
    #[doc = "Bit 5 - Use FIFO for channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn usef1(&mut self) -> USEF1_W<CTL_SPEC> {
        USEF1_W::new(self, 5)
    }
    #[doc = "Bit 6 - Clear FIFO"]
    #[inline(always)]
    #[must_use]
    pub fn clrf1(&mut self) -> CLRF1_W<CTL_SPEC> {
        CLRF1_W::new(self, 6)
    }
    #[doc = "Bit 7 - M/S mode for channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn msen1(&mut self) -> MSEN1_W<CTL_SPEC> {
        MSEN1_W::new(self, 7)
    }
    #[doc = "Bit 8 - Enable channel 2"]
    #[inline(always)]
    #[must_use]
    pub fn pwen2(&mut self) -> PWEN2_W<CTL_SPEC> {
        PWEN2_W::new(self, 8)
    }
    #[doc = "Bit 9 - Channel 2 mode"]
    #[inline(always)]
    #[must_use]
    pub fn mode2(&mut self) -> MODE2_W<CTL_SPEC> {
        MODE2_W::new(self, 9)
    }
    #[doc = "Bit 10 - Repeat last value from FIFO for channel 2"]
    #[inline(always)]
    #[must_use]
    pub fn rptl2(&mut self) -> RPTL2_W<CTL_SPEC> {
        RPTL2_W::new(self, 10)
    }
    #[doc = "Bit 11 - State when not transmitting on channel 2"]
    #[inline(always)]
    #[must_use]
    pub fn sbit2(&mut self) -> SBIT2_W<CTL_SPEC> {
        SBIT2_W::new(self, 11)
    }
    #[doc = "Bit 12 - Channel 2 polarity inverted"]
    #[inline(always)]
    #[must_use]
    pub fn pola2(&mut self) -> POLA2_W<CTL_SPEC> {
        POLA2_W::new(self, 12)
    }
    #[doc = "Bit 13 - Use FIFO for channel 2"]
    #[inline(always)]
    #[must_use]
    pub fn usef2(&mut self) -> USEF2_W<CTL_SPEC> {
        USEF2_W::new(self, 13)
    }
    #[doc = "Bit 15 - M/S mode for channel 2"]
    #[inline(always)]
    #[must_use]
    pub fn msen2(&mut self) -> MSEN2_W<CTL_SPEC> {
        MSEN2_W::new(self, 15)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTL_SPEC;
impl crate::RegisterSpec for CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctl::R`](R) reader structure"]
impl crate::Readable for CTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctl::W`](W) writer structure"]
impl crate::Writable for CTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTL to value 0"]
impl crate::Resettable for CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
