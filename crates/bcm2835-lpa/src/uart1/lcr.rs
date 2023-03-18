#[doc = "Register `LCR` reader"]
pub struct R(crate::R<LCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LCR` writer"]
pub struct W(crate::W<LCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LCR_SPEC>;
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
impl From<crate::W<LCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATA_SIZE` reader - UART word size"]
pub type DATA_SIZE_R = crate::FieldReader<u8, MODE_A>;
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
impl DATA_SIZE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MODE_A> {
        match self.bits {
            0 => Some(MODE_A::_7BIT),
            3 => Some(MODE_A::_8BIT),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_7BIT`"]
    #[inline(always)]
    pub fn is_7bit(&self) -> bool {
        *self == MODE_A::_7BIT
    }
    #[doc = "Checks if the value of the field is `_8BIT`"]
    #[inline(always)]
    pub fn is_8bit(&self) -> bool {
        *self == MODE_A::_8BIT
    }
}
#[doc = "Field `DATA_SIZE` writer - UART word size"]
pub type DATA_SIZE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LCR_SPEC, u8, MODE_A, 2, O>;
impl<'a, const O: u8> DATA_SIZE_W<'a, O> {
    #[doc = "7 bit"]
    #[inline(always)]
    pub fn _7bit(self) -> &'a mut W {
        self.variant(MODE_A::_7BIT)
    }
    #[doc = "8 bit"]
    #[inline(always)]
    pub fn _8bit(self) -> &'a mut W {
        self.variant(MODE_A::_8BIT)
    }
}
#[doc = "Field `BREAK` reader - Pull TX low continuously to send break"]
pub type BREAK_R = crate::BitReader<bool>;
#[doc = "Field `BREAK` writer - Pull TX low continuously to send break"]
pub type BREAK_W<'a, const O: u8> = crate::BitWriter<'a, u32, LCR_SPEC, bool, O>;
#[doc = "Field `DLAB` reader - First two registers are baudrate"]
pub type DLAB_R = crate::BitReader<bool>;
#[doc = "Field `DLAB` writer - First two registers are baudrate"]
pub type DLAB_W<'a, const O: u8> = crate::BitWriter<'a, u32, LCR_SPEC, bool, O>;
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
impl W {
    #[doc = "Bits 0:1 - UART word size"]
    #[inline(always)]
    #[must_use]
    pub fn data_size(&mut self) -> DATA_SIZE_W<0> {
        DATA_SIZE_W::new(self)
    }
    #[doc = "Bit 6 - Pull TX low continuously to send break"]
    #[inline(always)]
    #[must_use]
    pub fn break_(&mut self) -> BREAK_W<6> {
        BREAK_W::new(self)
    }
    #[doc = "Bit 7 - First two registers are baudrate"]
    #[inline(always)]
    #[must_use]
    pub fn dlab(&mut self) -> DLAB_W<7> {
        DLAB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Line control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcr](index.html) module"]
pub struct LCR_SPEC;
impl crate::RegisterSpec for LCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lcr::R](R) reader structure"]
impl crate::Readable for LCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lcr::W](W) writer structure"]
impl crate::Writable for LCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LCR to value 0"]
impl crate::Resettable for LCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
