#[doc = "Register `GICD_PPISR` reader"]
pub struct R(crate::R<GICD_PPISR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GICD_PPISR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GICD_PPISR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GICD_PPISR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GICD_PPISR` writer"]
pub struct W(crate::W<GICD_PPISR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GICD_PPISR_SPEC>;
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
impl From<crate::W<GICD_PPISR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GICD_PPISR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ID25` reader - Virtual maintenance interrupt"]
pub type ID25_R = crate::BitReader<bool>;
#[doc = "Field `ID25` writer - Virtual maintenance interrupt"]
pub type ID25_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_PPISR_SPEC, bool, O>;
#[doc = "Field `ID26` reader - Hypervisor timer event"]
pub type ID26_R = crate::BitReader<bool>;
#[doc = "Field `ID26` writer - Hypervisor timer event"]
pub type ID26_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_PPISR_SPEC, bool, O>;
#[doc = "Field `ID27` reader - Virtual timer event"]
pub type ID27_R = crate::BitReader<bool>;
#[doc = "Field `ID27` writer - Virtual timer event"]
pub type ID27_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_PPISR_SPEC, bool, O>;
#[doc = "Field `ID28` reader - nLEGACYFIQ signal"]
pub type ID28_R = crate::BitReader<bool>;
#[doc = "Field `ID28` writer - nLEGACYFIQ signal"]
pub type ID28_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_PPISR_SPEC, bool, O>;
#[doc = "Field `ID29` reader - Secure physical timer event"]
pub type ID29_R = crate::BitReader<bool>;
#[doc = "Field `ID29` writer - Secure physical timer event"]
pub type ID29_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_PPISR_SPEC, bool, O>;
#[doc = "Field `ID30` reader - Non-secure physical timer event"]
pub type ID30_R = crate::BitReader<bool>;
#[doc = "Field `ID30` writer - Non-secure physical timer event"]
pub type ID30_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_PPISR_SPEC, bool, O>;
#[doc = "Field `ID31` reader - nLEGACYIRQ signal"]
pub type ID31_R = crate::BitReader<bool>;
#[doc = "Field `ID31` writer - nLEGACYIRQ signal"]
pub type ID31_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_PPISR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 9 - Virtual maintenance interrupt"]
    #[inline(always)]
    pub fn id25(&self) -> ID25_R {
        ID25_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Hypervisor timer event"]
    #[inline(always)]
    pub fn id26(&self) -> ID26_R {
        ID26_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Virtual timer event"]
    #[inline(always)]
    pub fn id27(&self) -> ID27_R {
        ID27_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - nLEGACYFIQ signal"]
    #[inline(always)]
    pub fn id28(&self) -> ID28_R {
        ID28_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Secure physical timer event"]
    #[inline(always)]
    pub fn id29(&self) -> ID29_R {
        ID29_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Non-secure physical timer event"]
    #[inline(always)]
    pub fn id30(&self) -> ID30_R {
        ID30_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - nLEGACYIRQ signal"]
    #[inline(always)]
    pub fn id31(&self) -> ID31_R {
        ID31_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 9 - Virtual maintenance interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn id25(&mut self) -> ID25_W<9> {
        ID25_W::new(self)
    }
    #[doc = "Bit 10 - Hypervisor timer event"]
    #[inline(always)]
    #[must_use]
    pub fn id26(&mut self) -> ID26_W<10> {
        ID26_W::new(self)
    }
    #[doc = "Bit 11 - Virtual timer event"]
    #[inline(always)]
    #[must_use]
    pub fn id27(&mut self) -> ID27_W<11> {
        ID27_W::new(self)
    }
    #[doc = "Bit 12 - nLEGACYFIQ signal"]
    #[inline(always)]
    #[must_use]
    pub fn id28(&mut self) -> ID28_W<12> {
        ID28_W::new(self)
    }
    #[doc = "Bit 13 - Secure physical timer event"]
    #[inline(always)]
    #[must_use]
    pub fn id29(&mut self) -> ID29_W<13> {
        ID29_W::new(self)
    }
    #[doc = "Bit 14 - Non-secure physical timer event"]
    #[inline(always)]
    #[must_use]
    pub fn id30(&mut self) -> ID30_W<14> {
        ID30_W::new(self)
    }
    #[doc = "Bit 15 - nLEGACYIRQ signal"]
    #[inline(always)]
    #[must_use]
    pub fn id31(&mut self) -> ID31_W<15> {
        ID31_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Private Peripheral Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_ppisr](index.html) module"]
pub struct GICD_PPISR_SPEC;
impl crate::RegisterSpec for GICD_PPISR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gicd_ppisr::R](R) reader structure"]
impl crate::Readable for GICD_PPISR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gicd_ppisr::W](W) writer structure"]
impl crate::Writable for GICD_PPISR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GICD_PPISR to value 0"]
impl crate::Resettable for GICD_PPISR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
