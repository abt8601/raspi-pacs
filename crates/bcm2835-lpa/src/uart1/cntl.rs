#[doc = "Register `CNTL` reader"]
pub type R = crate::R<CNTL_SPEC>;
#[doc = "Register `CNTL` writer"]
pub type W = crate::W<CNTL_SPEC>;
#[doc = "Field `RX_ENABLE` reader - Enable receive"]
pub type RX_ENABLE_R = crate::BitReader;
#[doc = "Field `RX_ENABLE` writer - Enable receive"]
pub type RX_ENABLE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TX_ENABLE` reader - Enable transmit"]
pub type TX_ENABLE_R = crate::BitReader;
#[doc = "Field `TX_ENABLE` writer - Enable transmit"]
pub type TX_ENABLE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RTS_ENABLE` reader - Enable auto receive flow control with RTS"]
pub type RTS_ENABLE_R = crate::BitReader;
#[doc = "Field `RTS_ENABLE` writer - Enable auto receive flow control with RTS"]
pub type RTS_ENABLE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CTS_ENABLE` reader - Enable auto transmit flow control with CTS"]
pub type CTS_ENABLE_R = crate::BitReader;
#[doc = "Field `CTS_ENABLE` writer - Enable auto transmit flow control with CTS"]
pub type CTS_ENABLE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RTS_FIFO_LEVEL` reader - FIFO level to de-assert RTS"]
pub type RTS_FIFO_LEVEL_R = crate::FieldReader<FIFO_LEVEL_A>;
#[doc = "FIFO level to de-assert RTS\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FIFO_LEVEL_A {
    #[doc = "0: 3 empty spaces"]
    _3EMPTY = 0,
    #[doc = "1: 2 empty spaces"]
    _2EMPTY = 1,
    #[doc = "2: 1 empty spaces"]
    _1EMPTY = 2,
    #[doc = "3: 4 empty spaces"]
    _4EMPTY = 3,
}
impl From<FIFO_LEVEL_A> for u8 {
    #[inline(always)]
    fn from(variant: FIFO_LEVEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for FIFO_LEVEL_A {
    type Ux = u8;
}
impl RTS_FIFO_LEVEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FIFO_LEVEL_A {
        match self.bits {
            0 => FIFO_LEVEL_A::_3EMPTY,
            1 => FIFO_LEVEL_A::_2EMPTY,
            2 => FIFO_LEVEL_A::_1EMPTY,
            3 => FIFO_LEVEL_A::_4EMPTY,
            _ => unreachable!(),
        }
    }
    #[doc = "3 empty spaces"]
    #[inline(always)]
    pub fn is_3empty(&self) -> bool {
        *self == FIFO_LEVEL_A::_3EMPTY
    }
    #[doc = "2 empty spaces"]
    #[inline(always)]
    pub fn is_2empty(&self) -> bool {
        *self == FIFO_LEVEL_A::_2EMPTY
    }
    #[doc = "1 empty spaces"]
    #[inline(always)]
    pub fn is_1empty(&self) -> bool {
        *self == FIFO_LEVEL_A::_1EMPTY
    }
    #[doc = "4 empty spaces"]
    #[inline(always)]
    pub fn is_4empty(&self) -> bool {
        *self == FIFO_LEVEL_A::_4EMPTY
    }
}
#[doc = "Field `RTS_FIFO_LEVEL` writer - FIFO level to de-assert RTS"]
pub type RTS_FIFO_LEVEL_W<'a, REG, const O: u8> =
    crate::FieldWriterSafe<'a, REG, 2, O, FIFO_LEVEL_A>;
impl<'a, REG, const O: u8> RTS_FIFO_LEVEL_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "3 empty spaces"]
    #[inline(always)]
    pub fn _3empty(self) -> &'a mut crate::W<REG> {
        self.variant(FIFO_LEVEL_A::_3EMPTY)
    }
    #[doc = "2 empty spaces"]
    #[inline(always)]
    pub fn _2empty(self) -> &'a mut crate::W<REG> {
        self.variant(FIFO_LEVEL_A::_2EMPTY)
    }
    #[doc = "1 empty spaces"]
    #[inline(always)]
    pub fn _1empty(self) -> &'a mut crate::W<REG> {
        self.variant(FIFO_LEVEL_A::_1EMPTY)
    }
    #[doc = "4 empty spaces"]
    #[inline(always)]
    pub fn _4empty(self) -> &'a mut crate::W<REG> {
        self.variant(FIFO_LEVEL_A::_4EMPTY)
    }
}
#[doc = "Field `RTS_ASSERT` reader - RTS assert level"]
pub use CTS_ASSERT_R as RTS_ASSERT_R;
#[doc = "Field `RTS_ASSERT` writer - RTS assert level"]
pub use CTS_ASSERT_W as RTS_ASSERT_W;
#[doc = "Field `CTS_ASSERT` reader - CTS assert level"]
pub type CTS_ASSERT_R = crate::BitReader<ASSERT_LEVEL_A>;
#[doc = "CTS assert level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ASSERT_LEVEL_A {
    #[doc = "0: Assert high"]
    HIGH = 0,
    #[doc = "1: Assert low"]
    LOW = 1,
}
impl From<ASSERT_LEVEL_A> for bool {
    #[inline(always)]
    fn from(variant: ASSERT_LEVEL_A) -> Self {
        variant as u8 != 0
    }
}
impl CTS_ASSERT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ASSERT_LEVEL_A {
        match self.bits {
            false => ASSERT_LEVEL_A::HIGH,
            true => ASSERT_LEVEL_A::LOW,
        }
    }
    #[doc = "Assert high"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == ASSERT_LEVEL_A::HIGH
    }
    #[doc = "Assert low"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == ASSERT_LEVEL_A::LOW
    }
}
#[doc = "Field `CTS_ASSERT` writer - CTS assert level"]
pub type CTS_ASSERT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, ASSERT_LEVEL_A>;
impl<'a, REG, const O: u8> CTS_ASSERT_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Assert high"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(ASSERT_LEVEL_A::HIGH)
    }
    #[doc = "Assert low"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(ASSERT_LEVEL_A::LOW)
    }
}
impl R {
    #[doc = "Bit 0 - Enable receive"]
    #[inline(always)]
    pub fn rx_enable(&self) -> RX_ENABLE_R {
        RX_ENABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable transmit"]
    #[inline(always)]
    pub fn tx_enable(&self) -> TX_ENABLE_R {
        TX_ENABLE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable auto receive flow control with RTS"]
    #[inline(always)]
    pub fn rts_enable(&self) -> RTS_ENABLE_R {
        RTS_ENABLE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable auto transmit flow control with CTS"]
    #[inline(always)]
    pub fn cts_enable(&self) -> CTS_ENABLE_R {
        CTS_ENABLE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - FIFO level to de-assert RTS"]
    #[inline(always)]
    pub fn rts_fifo_level(&self) -> RTS_FIFO_LEVEL_R {
        RTS_FIFO_LEVEL_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - RTS assert level"]
    #[inline(always)]
    pub fn rts_assert(&self) -> RTS_ASSERT_R {
        RTS_ASSERT_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - CTS assert level"]
    #[inline(always)]
    pub fn cts_assert(&self) -> CTS_ASSERT_R {
        CTS_ASSERT_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CNTL")
            .field("cts_assert", &format_args!("{}", self.cts_assert().bit()))
            .field("rts_assert", &format_args!("{}", self.rts_assert().bit()))
            .field(
                "rts_fifo_level",
                &format_args!("{}", self.rts_fifo_level().bits()),
            )
            .field("cts_enable", &format_args!("{}", self.cts_enable().bit()))
            .field("rts_enable", &format_args!("{}", self.rts_enable().bit()))
            .field("tx_enable", &format_args!("{}", self.tx_enable().bit()))
            .field("rx_enable", &format_args!("{}", self.rx_enable().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<CNTL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Enable receive"]
    #[inline(always)]
    #[must_use]
    pub fn rx_enable(&mut self) -> RX_ENABLE_W<CNTL_SPEC, 0> {
        RX_ENABLE_W::new(self)
    }
    #[doc = "Bit 1 - Enable transmit"]
    #[inline(always)]
    #[must_use]
    pub fn tx_enable(&mut self) -> TX_ENABLE_W<CNTL_SPEC, 1> {
        TX_ENABLE_W::new(self)
    }
    #[doc = "Bit 2 - Enable auto receive flow control with RTS"]
    #[inline(always)]
    #[must_use]
    pub fn rts_enable(&mut self) -> RTS_ENABLE_W<CNTL_SPEC, 2> {
        RTS_ENABLE_W::new(self)
    }
    #[doc = "Bit 3 - Enable auto transmit flow control with CTS"]
    #[inline(always)]
    #[must_use]
    pub fn cts_enable(&mut self) -> CTS_ENABLE_W<CNTL_SPEC, 3> {
        CTS_ENABLE_W::new(self)
    }
    #[doc = "Bits 4:5 - FIFO level to de-assert RTS"]
    #[inline(always)]
    #[must_use]
    pub fn rts_fifo_level(&mut self) -> RTS_FIFO_LEVEL_W<CNTL_SPEC, 4> {
        RTS_FIFO_LEVEL_W::new(self)
    }
    #[doc = "Bit 6 - RTS assert level"]
    #[inline(always)]
    #[must_use]
    pub fn rts_assert(&mut self) -> RTS_ASSERT_W<CNTL_SPEC, 6> {
        RTS_ASSERT_W::new(self)
    }
    #[doc = "Bit 7 - CTS assert level"]
    #[inline(always)]
    #[must_use]
    pub fn cts_assert(&mut self) -> CTS_ASSERT_W<CNTL_SPEC, 7> {
        CTS_ASSERT_W::new(self)
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
#[doc = "Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cntl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cntl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CNTL_SPEC;
impl crate::RegisterSpec for CNTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cntl::R`](R) reader structure"]
impl crate::Readable for CNTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cntl::W`](W) writer structure"]
impl crate::Writable for CNTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CNTL to value 0"]
impl crate::Resettable for CNTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
