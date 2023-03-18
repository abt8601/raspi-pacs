#[doc = "Register `CTL` reader"]
pub struct R(crate::R<CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTL` writer"]
pub struct W(crate::W<CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PWEN1` reader - Enable channel 1"]
pub type PWEN1_R = crate::BitReader<bool>;
#[doc = "Field `PWEN1` writer - Enable channel 1"]
pub type PWEN1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL_SPEC, bool, O>;
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
    pub fn variant(&self) -> MODE1_A {
        match self.bits {
            false => MODE1_A::PWM,
            true => MODE1_A::SERIAL,
        }
    }
    #[doc = "Checks if the value of the field is `PWM`"]
    #[inline(always)]
    pub fn is_pwm(&self) -> bool {
        *self == MODE1_A::PWM
    }
    #[doc = "Checks if the value of the field is `SERIAL`"]
    #[inline(always)]
    pub fn is_serial(&self) -> bool {
        *self == MODE1_A::SERIAL
    }
}
#[doc = "Field `MODE1` writer - Channel 1 mode"]
pub type MODE1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL_SPEC, MODE1_A, O>;
impl<'a, const O: u8> MODE1_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn pwm(self) -> &'a mut W {
        self.variant(MODE1_A::PWM)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn serial(self) -> &'a mut W {
        self.variant(MODE1_A::SERIAL)
    }
}
#[doc = "Field `RPTL1` reader - Repeat last value from FIFO for channel 1"]
pub type RPTL1_R = crate::BitReader<bool>;
#[doc = "Field `RPTL1` writer - Repeat last value from FIFO for channel 1"]
pub type RPTL1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL_SPEC, bool, O>;
#[doc = "Field `SBIT1` reader - State when not transmitting on channel 1"]
pub type SBIT1_R = crate::BitReader<bool>;
#[doc = "Field `SBIT1` writer - State when not transmitting on channel 1"]
pub type SBIT1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL_SPEC, bool, O>;
#[doc = "Field `POLA1` reader - Channel 1 polarity inverted"]
pub type POLA1_R = crate::BitReader<bool>;
#[doc = "Field `POLA1` writer - Channel 1 polarity inverted"]
pub type POLA1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL_SPEC, bool, O>;
#[doc = "Field `USEF1` reader - Use FIFO for channel 1"]
pub type USEF1_R = crate::BitReader<bool>;
#[doc = "Field `USEF1` writer - Use FIFO for channel 1"]
pub type USEF1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL_SPEC, bool, O>;
#[doc = "Field `CLRF1` reader - Clear FIFO"]
pub type CLRF1_R = crate::BitReader<bool>;
#[doc = "Field `CLRF1` writer - Clear FIFO"]
pub type CLRF1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL_SPEC, bool, O>;
#[doc = "Field `MSEN1` reader - M/S mode for channel 1"]
pub type MSEN1_R = crate::BitReader<bool>;
#[doc = "Field `MSEN1` writer - M/S mode for channel 1"]
pub type MSEN1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL_SPEC, bool, O>;
#[doc = "Field `PWEN2` reader - Enable channel 2"]
pub type PWEN2_R = crate::BitReader<bool>;
#[doc = "Field `PWEN2` writer - Enable channel 2"]
pub type PWEN2_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL_SPEC, bool, O>;
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
    pub fn variant(&self) -> MODE2_A {
        match self.bits {
            false => MODE2_A::PWM,
            true => MODE2_A::SERIAL,
        }
    }
    #[doc = "Checks if the value of the field is `PWM`"]
    #[inline(always)]
    pub fn is_pwm(&self) -> bool {
        *self == MODE2_A::PWM
    }
    #[doc = "Checks if the value of the field is `SERIAL`"]
    #[inline(always)]
    pub fn is_serial(&self) -> bool {
        *self == MODE2_A::SERIAL
    }
}
#[doc = "Field `MODE2` writer - Channel 2 mode"]
pub type MODE2_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL_SPEC, MODE2_A, O>;
impl<'a, const O: u8> MODE2_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn pwm(self) -> &'a mut W {
        self.variant(MODE2_A::PWM)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn serial(self) -> &'a mut W {
        self.variant(MODE2_A::SERIAL)
    }
}
#[doc = "Field `RPTL2` reader - Repeat last value from FIFO for channel 2"]
pub type RPTL2_R = crate::BitReader<bool>;
#[doc = "Field `RPTL2` writer - Repeat last value from FIFO for channel 2"]
pub type RPTL2_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL_SPEC, bool, O>;
#[doc = "Field `SBIT2` reader - State when not transmitting on channel 2"]
pub type SBIT2_R = crate::BitReader<bool>;
#[doc = "Field `SBIT2` writer - State when not transmitting on channel 2"]
pub type SBIT2_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL_SPEC, bool, O>;
#[doc = "Field `POLA2` reader - Channel 2 polarity inverted"]
pub type POLA2_R = crate::BitReader<bool>;
#[doc = "Field `POLA2` writer - Channel 2 polarity inverted"]
pub type POLA2_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL_SPEC, bool, O>;
#[doc = "Field `USEF2` reader - Use FIFO for channel 2"]
pub type USEF2_R = crate::BitReader<bool>;
#[doc = "Field `USEF2` writer - Use FIFO for channel 2"]
pub type USEF2_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL_SPEC, bool, O>;
#[doc = "Field `MSEN2` reader - M/S mode for channel 2"]
pub type MSEN2_R = crate::BitReader<bool>;
#[doc = "Field `MSEN2` writer - M/S mode for channel 2"]
pub type MSEN2_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTL_SPEC, bool, O>;
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
impl W {
    #[doc = "Bit 0 - Enable channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn pwen1(&mut self) -> PWEN1_W<0> {
        PWEN1_W::new(self)
    }
    #[doc = "Bit 1 - Channel 1 mode"]
    #[inline(always)]
    #[must_use]
    pub fn mode1(&mut self) -> MODE1_W<1> {
        MODE1_W::new(self)
    }
    #[doc = "Bit 2 - Repeat last value from FIFO for channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn rptl1(&mut self) -> RPTL1_W<2> {
        RPTL1_W::new(self)
    }
    #[doc = "Bit 3 - State when not transmitting on channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn sbit1(&mut self) -> SBIT1_W<3> {
        SBIT1_W::new(self)
    }
    #[doc = "Bit 4 - Channel 1 polarity inverted"]
    #[inline(always)]
    #[must_use]
    pub fn pola1(&mut self) -> POLA1_W<4> {
        POLA1_W::new(self)
    }
    #[doc = "Bit 5 - Use FIFO for channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn usef1(&mut self) -> USEF1_W<5> {
        USEF1_W::new(self)
    }
    #[doc = "Bit 6 - Clear FIFO"]
    #[inline(always)]
    #[must_use]
    pub fn clrf1(&mut self) -> CLRF1_W<6> {
        CLRF1_W::new(self)
    }
    #[doc = "Bit 7 - M/S mode for channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn msen1(&mut self) -> MSEN1_W<7> {
        MSEN1_W::new(self)
    }
    #[doc = "Bit 8 - Enable channel 2"]
    #[inline(always)]
    #[must_use]
    pub fn pwen2(&mut self) -> PWEN2_W<8> {
        PWEN2_W::new(self)
    }
    #[doc = "Bit 9 - Channel 2 mode"]
    #[inline(always)]
    #[must_use]
    pub fn mode2(&mut self) -> MODE2_W<9> {
        MODE2_W::new(self)
    }
    #[doc = "Bit 10 - Repeat last value from FIFO for channel 2"]
    #[inline(always)]
    #[must_use]
    pub fn rptl2(&mut self) -> RPTL2_W<10> {
        RPTL2_W::new(self)
    }
    #[doc = "Bit 11 - State when not transmitting on channel 2"]
    #[inline(always)]
    #[must_use]
    pub fn sbit2(&mut self) -> SBIT2_W<11> {
        SBIT2_W::new(self)
    }
    #[doc = "Bit 12 - Channel 2 polarity inverted"]
    #[inline(always)]
    #[must_use]
    pub fn pola2(&mut self) -> POLA2_W<12> {
        POLA2_W::new(self)
    }
    #[doc = "Bit 13 - Use FIFO for channel 2"]
    #[inline(always)]
    #[must_use]
    pub fn usef2(&mut self) -> USEF2_W<13> {
        USEF2_W::new(self)
    }
    #[doc = "Bit 15 - M/S mode for channel 2"]
    #[inline(always)]
    #[must_use]
    pub fn msen2(&mut self) -> MSEN2_W<15> {
        MSEN2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctl](index.html) module"]
pub struct CTL_SPEC;
impl crate::RegisterSpec for CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctl::R](R) reader structure"]
impl crate::Readable for CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctl::W](W) writer structure"]
impl crate::Writable for CTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTL to value 0"]
impl crate::Resettable for CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
