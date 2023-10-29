#[doc = "Register `GICD_ITARGETSR17` reader"]
pub type R = crate::R<GICD_ITARGETSR17_SPEC>;
#[doc = "Register `GICD_ITARGETSR17` writer"]
pub type W = crate::W<GICD_ITARGETSR17_SPEC>;
#[doc = "Field `VPU0_HALTED` reader - VPU0 halted"]
pub type VPU0_HALTED_R = crate::FieldReader;
#[doc = "Field `VPU0_HALTED` writer - VPU0 halted"]
pub type VPU0_HALTED_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `VPU1_HALTED` reader - VPU1 halted"]
pub type VPU1_HALTED_R = crate::FieldReader;
#[doc = "Field `VPU1_HALTED` writer - VPU1 halted"]
pub type VPU1_HALTED_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `ARM_ADDRESS_ERROR` reader - ARM address error"]
pub type ARM_ADDRESS_ERROR_R = crate::FieldReader;
#[doc = "Field `ARM_ADDRESS_ERROR` writer - ARM address error"]
pub type ARM_ADDRESS_ERROR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `ARM_AXI_ERROR` reader - ARM AXI error"]
pub type ARM_AXI_ERROR_R = crate::FieldReader;
#[doc = "Field `ARM_AXI_ERROR` writer - ARM AXI error"]
pub type ARM_AXI_ERROR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
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
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICD_ITARGETSR17")
            .field(
                "vpu0_halted",
                &format_args!("{}", self.vpu0_halted().bits()),
            )
            .field(
                "vpu1_halted",
                &format_args!("{}", self.vpu1_halted().bits()),
            )
            .field(
                "arm_address_error",
                &format_args!("{}", self.arm_address_error().bits()),
            )
            .field(
                "arm_axi_error",
                &format_args!("{}", self.arm_axi_error().bits()),
            )
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<GICD_ITARGETSR17_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:7 - VPU0 halted"]
    #[inline(always)]
    #[must_use]
    pub fn vpu0_halted(&mut self) -> VPU0_HALTED_W<GICD_ITARGETSR17_SPEC, 0> {
        VPU0_HALTED_W::new(self)
    }
    #[doc = "Bits 8:15 - VPU1 halted"]
    #[inline(always)]
    #[must_use]
    pub fn vpu1_halted(&mut self) -> VPU1_HALTED_W<GICD_ITARGETSR17_SPEC, 8> {
        VPU1_HALTED_W::new(self)
    }
    #[doc = "Bits 16:23 - ARM address error"]
    #[inline(always)]
    #[must_use]
    pub fn arm_address_error(&mut self) -> ARM_ADDRESS_ERROR_W<GICD_ITARGETSR17_SPEC, 16> {
        ARM_ADDRESS_ERROR_W::new(self)
    }
    #[doc = "Bits 24:31 - ARM AXI error"]
    #[inline(always)]
    #[must_use]
    pub fn arm_axi_error(&mut self) -> ARM_AXI_ERROR_W<GICD_ITARGETSR17_SPEC, 24> {
        ARM_AXI_ERROR_W::new(self)
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
#[doc = "Interrupt Processor Target 68 - 71\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_itargetsr17::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_itargetsr17::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GICD_ITARGETSR17_SPEC;
impl crate::RegisterSpec for GICD_ITARGETSR17_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_itargetsr17::R`](R) reader structure"]
impl crate::Readable for GICD_ITARGETSR17_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gicd_itargetsr17::W`](W) writer structure"]
impl crate::Writable for GICD_ITARGETSR17_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GICD_ITARGETSR17 to value 0"]
impl crate::Resettable for GICD_ITARGETSR17_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
