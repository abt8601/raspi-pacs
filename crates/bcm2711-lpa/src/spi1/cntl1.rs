#[doc = "Register `CNTL1` reader"]
pub struct R(crate::R<CNTL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CNTL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CNTL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CNTL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CNTL1` writer"]
pub struct W(crate::W<CNTL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CNTL1_SPEC>;
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
impl From<crate::W<CNTL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CNTL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `KEEP_INPUT` reader - Don't clear the RX shift register before a new transaction"]
pub type KEEP_INPUT_R = crate::BitReader<bool>;
#[doc = "Field `KEEP_INPUT` writer - Don't clear the RX shift register before a new transaction"]
pub type KEEP_INPUT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CNTL1_SPEC, bool, O>;
#[doc = "Field `MSB_FIRST` reader - Shift the most significant bit first (MSB)"]
pub type MSB_FIRST_R = crate::BitReader<bool>;
#[doc = "Field `MSB_FIRST` writer - Shift the most significant bit first (MSB)"]
pub type MSB_FIRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, CNTL1_SPEC, bool, O>;
#[doc = "Field `DONE_ENABLE` reader - Enable DONE interrupt"]
pub type DONE_ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `DONE_ENABLE` writer - Enable DONE interrupt"]
pub type DONE_ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CNTL1_SPEC, bool, O>;
#[doc = "Field `TXE_ENABLE` reader - Enable TX empty interrupt"]
pub type TXE_ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `TXE_ENABLE` writer - Enable TX empty interrupt"]
pub type TXE_ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CNTL1_SPEC, bool, O>;
#[doc = "Field `CS_HIGH_TIME` reader - Additional SPI clock cycles where CS is high"]
pub type CS_HIGH_TIME_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CS_HIGH_TIME` writer - Additional SPI clock cycles where CS is high"]
pub type CS_HIGH_TIME_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CNTL1_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bit 0 - Don't clear the RX shift register before a new transaction"]
    #[inline(always)]
    pub fn keep_input(&self) -> KEEP_INPUT_R {
        KEEP_INPUT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Shift the most significant bit first (MSB)"]
    #[inline(always)]
    pub fn msb_first(&self) -> MSB_FIRST_R {
        MSB_FIRST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable DONE interrupt"]
    #[inline(always)]
    pub fn done_enable(&self) -> DONE_ENABLE_R {
        DONE_ENABLE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable TX empty interrupt"]
    #[inline(always)]
    pub fn txe_enable(&self) -> TXE_ENABLE_R {
        TXE_ENABLE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - Additional SPI clock cycles where CS is high"]
    #[inline(always)]
    pub fn cs_high_time(&self) -> CS_HIGH_TIME_R {
        CS_HIGH_TIME_R::new(((self.bits >> 8) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Don't clear the RX shift register before a new transaction"]
    #[inline(always)]
    #[must_use]
    pub fn keep_input(&mut self) -> KEEP_INPUT_W<0> {
        KEEP_INPUT_W::new(self)
    }
    #[doc = "Bit 1 - Shift the most significant bit first (MSB)"]
    #[inline(always)]
    #[must_use]
    pub fn msb_first(&mut self) -> MSB_FIRST_W<1> {
        MSB_FIRST_W::new(self)
    }
    #[doc = "Bit 6 - Enable DONE interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn done_enable(&mut self) -> DONE_ENABLE_W<6> {
        DONE_ENABLE_W::new(self)
    }
    #[doc = "Bit 7 - Enable TX empty interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn txe_enable(&mut self) -> TXE_ENABLE_W<7> {
        TXE_ENABLE_W::new(self)
    }
    #[doc = "Bits 8:10 - Additional SPI clock cycles where CS is high"]
    #[inline(always)]
    #[must_use]
    pub fn cs_high_time(&mut self) -> CS_HIGH_TIME_W<8> {
        CS_HIGH_TIME_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cntl1](index.html) module"]
pub struct CNTL1_SPEC;
impl crate::RegisterSpec for CNTL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cntl1::R](R) reader structure"]
impl crate::Readable for CNTL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cntl1::W](W) writer structure"]
impl crate::Writable for CNTL1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CNTL1 to value 0"]
impl crate::Resettable for CNTL1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
