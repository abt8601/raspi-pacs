#[doc = "Register `GICD_IPRIORITYR32` reader"]
pub type R = crate::R<GICD_IPRIORITYR32_SPEC>;
#[doc = "Register `GICD_IPRIORITYR32` writer"]
pub type W = crate::W<GICD_IPRIORITYR32_SPEC>;
#[doc = "Field `HDMI_CEC` reader - HDMI CEC"]
pub type HDMI_CEC_R = crate::FieldReader;
#[doc = "Field `HDMI_CEC` writer - HDMI CEC"]
pub type HDMI_CEC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `HVS` reader - HVS"]
pub type HVS_R = crate::FieldReader;
#[doc = "Field `HVS` writer - HVS"]
pub type HVS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `RPIVID` reader - RPIVID"]
pub type RPIVID_R = crate::FieldReader;
#[doc = "Field `RPIVID` writer - RPIVID"]
pub type RPIVID_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `SDC` reader - SDC"]
pub type SDC_R = crate::FieldReader;
#[doc = "Field `SDC` writer - SDC"]
pub type SDC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - HDMI CEC"]
    #[inline(always)]
    pub fn hdmi_cec(&self) -> HDMI_CEC_R {
        HDMI_CEC_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - HVS"]
    #[inline(always)]
    pub fn hvs(&self) -> HVS_R {
        HVS_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - RPIVID"]
    #[inline(always)]
    pub fn rpivid(&self) -> RPIVID_R {
        RPIVID_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - SDC"]
    #[inline(always)]
    pub fn sdc(&self) -> SDC_R {
        SDC_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICD_IPRIORITYR32")
            .field("hdmi_cec", &format_args!("{}", self.hdmi_cec().bits()))
            .field("hvs", &format_args!("{}", self.hvs().bits()))
            .field("rpivid", &format_args!("{}", self.rpivid().bits()))
            .field("sdc", &format_args!("{}", self.sdc().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<GICD_IPRIORITYR32_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:7 - HDMI CEC"]
    #[inline(always)]
    #[must_use]
    pub fn hdmi_cec(&mut self) -> HDMI_CEC_W<GICD_IPRIORITYR32_SPEC, 0> {
        HDMI_CEC_W::new(self)
    }
    #[doc = "Bits 8:15 - HVS"]
    #[inline(always)]
    #[must_use]
    pub fn hvs(&mut self) -> HVS_W<GICD_IPRIORITYR32_SPEC, 8> {
        HVS_W::new(self)
    }
    #[doc = "Bits 16:23 - RPIVID"]
    #[inline(always)]
    #[must_use]
    pub fn rpivid(&mut self) -> RPIVID_W<GICD_IPRIORITYR32_SPEC, 16> {
        RPIVID_W::new(self)
    }
    #[doc = "Bits 24:31 - SDC"]
    #[inline(always)]
    #[must_use]
    pub fn sdc(&mut self) -> SDC_W<GICD_IPRIORITYR32_SPEC, 24> {
        SDC_W::new(self)
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
#[doc = "Interrupt Priority 128 - 131 (Lower is first)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_ipriorityr32::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_ipriorityr32::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GICD_IPRIORITYR32_SPEC;
impl crate::RegisterSpec for GICD_IPRIORITYR32_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_ipriorityr32::R`](R) reader structure"]
impl crate::Readable for GICD_IPRIORITYR32_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gicd_ipriorityr32::W`](W) writer structure"]
impl crate::Writable for GICD_IPRIORITYR32_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GICD_IPRIORITYR32 to value 0"]
impl crate::Resettable for GICD_IPRIORITYR32_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
