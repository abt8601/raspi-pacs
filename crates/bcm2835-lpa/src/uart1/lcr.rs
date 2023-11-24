#[doc = "Register `LCR` reader"]
pub type R = crate::R<LCR_SPEC>;
#[doc = "Register `LCR` writer"]
pub type W = crate::W<LCR_SPEC>;
#[doc = "Field `DATA_SIZE` reader - UART word size"]
pub type DATA_SIZE_R = crate::FieldReader<MODE_A>;
#[doc = "UART word size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODE_A {
    #[doc = "0: 7 bit"]
    _7BIT = 0,
    #[doc = "3: 8 bit"]
    _8BIT = 3,
}
impl From<MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MODE_A {
    type Ux = u8;
}
impl DATA_SIZE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<MODE_A> {
        match self.bits {
            0 => Some(MODE_A::_7BIT),
            3 => Some(MODE_A::_8BIT),
            _ => None,
        }
    }
    #[doc = "7 bit"]
    #[inline(always)]
    pub fn is_7bit(&self) -> bool {
        *self == MODE_A::_7BIT
    }
    #[doc = "8 bit"]
    #[inline(always)]
    pub fn is_8bit(&self) -> bool {
        *self == MODE_A::_8BIT
    }
}
#[doc = "Field `DATA_SIZE` writer - UART word size"]
pub type DATA_SIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 2, MODE_A>;
impl<'a, REG> DATA_SIZE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "7 bit"]
    #[inline(always)]
    pub fn _7bit(self) -> &'a mut crate::W<REG> {
        self.variant(MODE_A::_7BIT)
    }
    #[doc = "8 bit"]
    #[inline(always)]
    pub fn _8bit(self) -> &'a mut crate::W<REG> {
        self.variant(MODE_A::_8BIT)
    }
}
#[doc = "Field `BREAK` reader - Pull TX low continuously to send break"]
pub type BREAK_R = crate::BitReader;
#[doc = "Field `BREAK` writer - Pull TX low continuously to send break"]
pub type BREAK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DLAB` reader - First two registers are baudrate"]
pub type DLAB_R = crate::BitReader;
#[doc = "Field `DLAB` writer - First two registers are baudrate"]
pub type DLAB_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - UART word size"]
    #[inline(always)]
    pub fn data_size(&self) -> DATA_SIZE_R {
        DATA_SIZE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 6 - Pull TX low continuously to send break"]
    #[inline(always)]
    pub fn break_(&self) -> BREAK_R {
        BREAK_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - First two registers are baudrate"]
    #[inline(always)]
    pub fn dlab(&self) -> DLAB_R {
        DLAB_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LCR")
            .field("dlab", &format_args!("{}", self.dlab().bit()))
            .field("break_", &format_args!("{}", self.break_().bit()))
            .field("data_size", &format_args!("{}", self.data_size().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<LCR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:1 - UART word size"]
    #[inline(always)]
    #[must_use]
    pub fn data_size(&mut self) -> DATA_SIZE_W<LCR_SPEC> {
        DATA_SIZE_W::new(self, 0)
    }
    #[doc = "Bit 6 - Pull TX low continuously to send break"]
    #[inline(always)]
    #[must_use]
    pub fn break_(&mut self) -> BREAK_W<LCR_SPEC> {
        BREAK_W::new(self, 6)
    }
    #[doc = "Bit 7 - First two registers are baudrate"]
    #[inline(always)]
    #[must_use]
    pub fn dlab(&mut self) -> DLAB_W<LCR_SPEC> {
        DLAB_W::new(self, 7)
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
#[doc = "Line control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LCR_SPEC;
impl crate::RegisterSpec for LCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lcr::R`](R) reader structure"]
impl crate::Readable for LCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lcr::W`](W) writer structure"]
impl crate::Writable for LCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LCR to value 0"]
impl crate::Resettable for LCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
