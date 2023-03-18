#[doc = "Register `ECR` writer"]
pub struct W(crate::W<ECR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ECR_SPEC>;
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
impl From<crate::W<ECR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ECR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FE` writer - FE"]
pub type FE_W<'a, const O: u8> = crate::BitWriter<'a, u32, ECR_SPEC, bool, O>;
#[doc = "Field `PE` writer - PE"]
pub type PE_W<'a, const O: u8> = crate::BitWriter<'a, u32, ECR_SPEC, bool, O>;
#[doc = "Field `BE` writer - BE"]
pub type BE_W<'a, const O: u8> = crate::BitWriter<'a, u32, ECR_SPEC, bool, O>;
#[doc = "Field `OE` writer - OE"]
pub type OE_W<'a, const O: u8> = crate::BitWriter<'a, u32, ECR_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - FE"]
    #[inline(always)]
    #[must_use]
    pub fn fe(&mut self) -> FE_W<0> {
        FE_W::new(self)
    }
    #[doc = "Bit 1 - PE"]
    #[inline(always)]
    #[must_use]
    pub fn pe(&mut self) -> PE_W<1> {
        PE_W::new(self)
    }
    #[doc = "Bit 2 - BE"]
    #[inline(always)]
    #[must_use]
    pub fn be(&mut self) -> BE_W<2> {
        BE_W::new(self)
    }
    #[doc = "Bit 3 - OE"]
    #[inline(always)]
    #[must_use]
    pub fn oe(&mut self) -> OE_W<3> {
        OE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Error Clear Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ecr](index.html) module"]
pub struct ECR_SPEC;
impl crate::RegisterSpec for ECR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [ecr::W](W) writer structure"]
impl crate::Writable for ECR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ECR to value 0"]
impl crate::Resettable for ECR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
