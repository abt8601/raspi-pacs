#[doc = "Register `GICD_CPENDSGIRn` reader"]
pub struct R(crate::R<GICD_CPENDSGIRN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GICD_CPENDSGIRN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GICD_CPENDSGIRN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GICD_CPENDSGIRN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GICD_CPENDSGIRn` writer"]
pub struct W(crate::W<GICD_CPENDSGIRN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GICD_CPENDSGIRN_SPEC>;
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
impl From<crate::W<GICD_CPENDSGIRN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GICD_CPENDSGIRN_SPEC>) -> Self {
        W(writer)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SGI Clear-Pending Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_cpendsgirn](index.html) module"]
pub struct GICD_CPENDSGIRN_SPEC;
impl crate::RegisterSpec for GICD_CPENDSGIRN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gicd_cpendsgirn::R](R) reader structure"]
impl crate::Readable for GICD_CPENDSGIRN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gicd_cpendsgirn::W](W) writer structure"]
impl crate::Writable for GICD_CPENDSGIRN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GICD_CPENDSGIRn to value 0"]
impl crate::Resettable for GICD_CPENDSGIRN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
