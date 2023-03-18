#[doc = "Register `GICD_IPRIORITYR39` reader"]
pub struct R(crate::R<GICD_IPRIORITYR39_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GICD_IPRIORITYR39_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GICD_IPRIORITYR39_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GICD_IPRIORITYR39_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GICD_IPRIORITYR39` writer"]
pub struct W(crate::W<GICD_IPRIORITYR39_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GICD_IPRIORITYR39_SPEC>;
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
impl From<crate::W<GICD_IPRIORITYR39_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GICD_IPRIORITYR39_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CPG` reader - CPG"]
pub type CPG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CPG` writer - CPG"]
pub type CPG_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GICD_IPRIORITYR39_SPEC, u8, u8, 8, O>;
#[doc = "Field `RNG` reader - RNG"]
pub type RNG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RNG` writer - RNG"]
pub type RNG_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GICD_IPRIORITYR39_SPEC, u8, u8, 8, O>;
#[doc = "Field `EMMC` reader - OR of EMMC and EMMC2"]
pub type EMMC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EMMC` writer - OR of EMMC and EMMC2"]
pub type EMMC_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GICD_IPRIORITYR39_SPEC, u8, u8, 8, O>;
#[doc = "Field `ETH_PCIE_SECURE` reader - ETH_PCIe secure"]
pub type ETH_PCIE_SECURE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ETH_PCIE_SECURE` writer - ETH_PCIe secure"]
pub type ETH_PCIE_SECURE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GICD_IPRIORITYR39_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - CPG"]
    #[inline(always)]
    pub fn cpg(&self) -> CPG_R {
        CPG_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - RNG"]
    #[inline(always)]
    pub fn rng(&self) -> RNG_R {
        RNG_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - OR of EMMC and EMMC2"]
    #[inline(always)]
    pub fn emmc(&self) -> EMMC_R {
        EMMC_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - ETH_PCIe secure"]
    #[inline(always)]
    pub fn eth_pcie_secure(&self) -> ETH_PCIE_SECURE_R {
        ETH_PCIE_SECURE_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - CPG"]
    #[inline(always)]
    #[must_use]
    pub fn cpg(&mut self) -> CPG_W<0> {
        CPG_W::new(self)
    }
    #[doc = "Bits 8:15 - RNG"]
    #[inline(always)]
    #[must_use]
    pub fn rng(&mut self) -> RNG_W<8> {
        RNG_W::new(self)
    }
    #[doc = "Bits 16:23 - OR of EMMC and EMMC2"]
    #[inline(always)]
    #[must_use]
    pub fn emmc(&mut self) -> EMMC_W<16> {
        EMMC_W::new(self)
    }
    #[doc = "Bits 24:31 - ETH_PCIe secure"]
    #[inline(always)]
    #[must_use]
    pub fn eth_pcie_secure(&mut self) -> ETH_PCIE_SECURE_W<24> {
        ETH_PCIE_SECURE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Priority 156 - 159 (Lower is first)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_ipriorityr39](index.html) module"]
pub struct GICD_IPRIORITYR39_SPEC;
impl crate::RegisterSpec for GICD_IPRIORITYR39_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gicd_ipriorityr39::R](R) reader structure"]
impl crate::Readable for GICD_IPRIORITYR39_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gicd_ipriorityr39::W](W) writer structure"]
impl crate::Writable for GICD_IPRIORITYR39_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GICD_IPRIORITYR39 to value 0"]
impl crate::Resettable for GICD_IPRIORITYR39_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
