#[doc = "Register `GICC_ABPR` reader"]
pub struct R(crate::R<GICC_ABPR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GICC_ABPR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GICC_ABPR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GICC_ABPR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GICC_ABPR` writer"]
pub struct W(crate::W<GICC_ABPR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GICC_ABPR_SPEC>;
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
impl From<crate::W<GICC_ABPR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GICC_ABPR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BINARY_POINT` reader - Split point between group priority and subpriority"]
pub type BINARY_POINT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BINARY_POINT` writer - Split point between group priority and subpriority"]
pub type BINARY_POINT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GICC_ABPR_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bits 0:2 - Split point between group priority and subpriority"]
    #[inline(always)]
    pub fn binary_point(&self) -> BINARY_POINT_R {
        BINARY_POINT_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Split point between group priority and subpriority"]
    #[inline(always)]
    #[must_use]
    pub fn binary_point(&mut self) -> BINARY_POINT_W<0> {
        BINARY_POINT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Aliased Binary Point\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicc_abpr](index.html) module"]
pub struct GICC_ABPR_SPEC;
impl crate::RegisterSpec for GICC_ABPR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gicc_abpr::R](R) reader structure"]
impl crate::Readable for GICC_ABPR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gicc_abpr::W](W) writer structure"]
impl crate::Writable for GICC_ABPR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GICC_ABPR to value 0"]
impl crate::Resettable for GICC_ABPR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
