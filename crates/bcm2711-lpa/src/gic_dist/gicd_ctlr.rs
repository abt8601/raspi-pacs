#[doc = "Register `GICD_CTLR` reader"]
pub struct R(crate::R<GICD_CTLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GICD_CTLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GICD_CTLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GICD_CTLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GICD_CTLR` writer"]
pub struct W(crate::W<GICD_CTLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GICD_CTLR_SPEC>;
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
impl From<crate::W<GICD_CTLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GICD_CTLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENABLE_GROUP0` reader - Enable group 0 interrupts"]
pub type ENABLE_GROUP0_R = crate::BitReader<bool>;
#[doc = "Field `ENABLE_GROUP0` writer - Enable group 0 interrupts"]
pub type ENABLE_GROUP0_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_CTLR_SPEC, bool, O>;
#[doc = "Field `ENABLE_GROUP1` reader - Enable group 1 interrupts"]
pub type ENABLE_GROUP1_R = crate::BitReader<bool>;
#[doc = "Field `ENABLE_GROUP1` writer - Enable group 1 interrupts"]
pub type ENABLE_GROUP1_W<'a, const O: u8> = crate::BitWriter<'a, u32, GICD_CTLR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Enable group 0 interrupts"]
    #[inline(always)]
    pub fn enable_group0(&self) -> ENABLE_GROUP0_R {
        ENABLE_GROUP0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable group 1 interrupts"]
    #[inline(always)]
    pub fn enable_group1(&self) -> ENABLE_GROUP1_R {
        ENABLE_GROUP1_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable group 0 interrupts"]
    #[inline(always)]
    #[must_use]
    pub fn enable_group0(&mut self) -> ENABLE_GROUP0_W<0> {
        ENABLE_GROUP0_W::new(self)
    }
    #[doc = "Bit 1 - Enable group 1 interrupts"]
    #[inline(always)]
    #[must_use]
    pub fn enable_group1(&mut self) -> ENABLE_GROUP1_W<1> {
        ENABLE_GROUP1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Distributor Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_ctlr](index.html) module"]
pub struct GICD_CTLR_SPEC;
impl crate::RegisterSpec for GICD_CTLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gicd_ctlr::R](R) reader structure"]
impl crate::Readable for GICD_CTLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gicd_ctlr::W](W) writer structure"]
impl crate::Writable for GICD_CTLR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GICD_CTLR to value 0"]
impl crate::Resettable for GICD_CTLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
