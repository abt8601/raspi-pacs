#[doc = "Register `GICC_HPPIR` reader"]
pub struct R(crate::R<GICC_HPPIR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GICC_HPPIR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GICC_HPPIR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GICC_HPPIR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GICC_HPPIR` writer"]
pub struct W(crate::W<GICC_HPPIR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GICC_HPPIR_SPEC>;
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
impl From<crate::W<GICC_HPPIR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GICC_HPPIR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INTERRUPT_ID` reader - Pending Interrupt ID"]
pub type INTERRUPT_ID_R = crate::FieldReader<u16, u16>;
#[doc = "Field `INTERRUPT_ID` writer - Pending Interrupt ID"]
pub type INTERRUPT_ID_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GICC_HPPIR_SPEC, u16, u16, 10, O>;
#[doc = "Field `CPUID` reader - CPUID that requested a software interrupt, 0 otherwise"]
pub type CPUID_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CPUID` writer - CPUID that requested a software interrupt, 0 otherwise"]
pub type CPUID_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GICC_HPPIR_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bits 0:9 - Pending Interrupt ID"]
    #[inline(always)]
    pub fn interrupt_id(&self) -> INTERRUPT_ID_R {
        INTERRUPT_ID_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 10:12 - CPUID that requested a software interrupt, 0 otherwise"]
    #[inline(always)]
    pub fn cpuid(&self) -> CPUID_R {
        CPUID_R::new(((self.bits >> 10) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:9 - Pending Interrupt ID"]
    #[inline(always)]
    #[must_use]
    pub fn interrupt_id(&mut self) -> INTERRUPT_ID_W<0> {
        INTERRUPT_ID_W::new(self)
    }
    #[doc = "Bits 10:12 - CPUID that requested a software interrupt, 0 otherwise"]
    #[inline(always)]
    #[must_use]
    pub fn cpuid(&mut self) -> CPUID_W<10> {
        CPUID_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Highest Priority Pending Interrupt\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicc_hppir](index.html) module"]
pub struct GICC_HPPIR_SPEC;
impl crate::RegisterSpec for GICC_HPPIR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gicc_hppir::R](R) reader structure"]
impl crate::Readable for GICC_HPPIR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gicc_hppir::W](W) writer structure"]
impl crate::Writable for GICC_HPPIR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GICC_HPPIR to value 0"]
impl crate::Resettable for GICC_HPPIR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
