#[doc = "Register `GICD_IPRIORITYR17` reader"]
pub struct R(crate::R<GICD_IPRIORITYR17_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GICD_IPRIORITYR17_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GICD_IPRIORITYR17_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GICD_IPRIORITYR17_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GICD_IPRIORITYR17` writer"]
pub struct W(crate::W<GICD_IPRIORITYR17_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GICD_IPRIORITYR17_SPEC>;
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
impl From<crate::W<GICD_IPRIORITYR17_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GICD_IPRIORITYR17_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VPU0_HALTED` reader - VPU0 halted"]
pub type VPU0_HALTED_R = crate::FieldReader<u8, u8>;
#[doc = "Field `VPU0_HALTED` writer - VPU0 halted"]
pub type VPU0_HALTED_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GICD_IPRIORITYR17_SPEC, u8, u8, 8, O>;
#[doc = "Field `VPU1_HALTED` reader - VPU1 halted"]
pub type VPU1_HALTED_R = crate::FieldReader<u8, u8>;
#[doc = "Field `VPU1_HALTED` writer - VPU1 halted"]
pub type VPU1_HALTED_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GICD_IPRIORITYR17_SPEC, u8, u8, 8, O>;
#[doc = "Field `ARM_ADDRESS_ERROR` reader - ARM address error"]
pub type ARM_ADDRESS_ERROR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ARM_ADDRESS_ERROR` writer - ARM address error"]
pub type ARM_ADDRESS_ERROR_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GICD_IPRIORITYR17_SPEC, u8, u8, 8, O>;
#[doc = "Field `ARM_AXI_ERROR` reader - ARM AXI error"]
pub type ARM_AXI_ERROR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ARM_AXI_ERROR` writer - ARM AXI error"]
pub type ARM_AXI_ERROR_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GICD_IPRIORITYR17_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - VPU0 halted"]
    #[inline(always)]
    pub fn vpu0_halted(&self) -> VPU0_HALTED_R {
        VPU0_HALTED_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - VPU1 halted"]
    #[inline(always)]
    pub fn vpu1_halted(&self) -> VPU1_HALTED_R {
        VPU1_HALTED_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - ARM address error"]
    #[inline(always)]
    pub fn arm_address_error(&self) -> ARM_ADDRESS_ERROR_R {
        ARM_ADDRESS_ERROR_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - ARM AXI error"]
    #[inline(always)]
    pub fn arm_axi_error(&self) -> ARM_AXI_ERROR_R {
        ARM_AXI_ERROR_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - VPU0 halted"]
    #[inline(always)]
    #[must_use]
    pub fn vpu0_halted(&mut self) -> VPU0_HALTED_W<0> {
        VPU0_HALTED_W::new(self)
    }
    #[doc = "Bits 8:15 - VPU1 halted"]
    #[inline(always)]
    #[must_use]
    pub fn vpu1_halted(&mut self) -> VPU1_HALTED_W<8> {
        VPU1_HALTED_W::new(self)
    }
    #[doc = "Bits 16:23 - ARM address error"]
    #[inline(always)]
    #[must_use]
    pub fn arm_address_error(&mut self) -> ARM_ADDRESS_ERROR_W<16> {
        ARM_ADDRESS_ERROR_W::new(self)
    }
    #[doc = "Bits 24:31 - ARM AXI error"]
    #[inline(always)]
    #[must_use]
    pub fn arm_axi_error(&mut self) -> ARM_AXI_ERROR_W<24> {
        ARM_AXI_ERROR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Priority 68 - 71 (Lower is first)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_ipriorityr17](index.html) module"]
pub struct GICD_IPRIORITYR17_SPEC;
impl crate::RegisterSpec for GICD_IPRIORITYR17_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gicd_ipriorityr17::R](R) reader structure"]
impl crate::Readable for GICD_IPRIORITYR17_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gicd_ipriorityr17::W](W) writer structure"]
impl crate::Writable for GICD_IPRIORITYR17_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GICD_IPRIORITYR17 to value 0"]
impl crate::Resettable for GICD_IPRIORITYR17_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
