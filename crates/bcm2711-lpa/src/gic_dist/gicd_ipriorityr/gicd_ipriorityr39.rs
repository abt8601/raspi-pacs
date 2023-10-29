#[doc = "Register `GICD_IPRIORITYR39` reader"]
pub type R = crate::R<GICD_IPRIORITYR39_SPEC>;
#[doc = "Register `GICD_IPRIORITYR39` writer"]
pub type W = crate::W<GICD_IPRIORITYR39_SPEC>;
#[doc = "Field `CPG` reader - CPG"]
pub type CPG_R = crate::FieldReader;
#[doc = "Field `CPG` writer - CPG"]
pub type CPG_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `RNG` reader - RNG"]
pub type RNG_R = crate::FieldReader;
#[doc = "Field `RNG` writer - RNG"]
pub type RNG_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `EMMC` reader - OR of EMMC and EMMC2"]
pub type EMMC_R = crate::FieldReader;
#[doc = "Field `EMMC` writer - OR of EMMC and EMMC2"]
pub type EMMC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `ETH_PCIE_SECURE` reader - ETH_PCIe secure"]
pub type ETH_PCIE_SECURE_R = crate::FieldReader;
#[doc = "Field `ETH_PCIE_SECURE` writer - ETH_PCIe secure"]
pub type ETH_PCIE_SECURE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
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
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICD_IPRIORITYR39")
            .field("cpg", &format_args!("{}", self.cpg().bits()))
            .field("rng", &format_args!("{}", self.rng().bits()))
            .field("emmc", &format_args!("{}", self.emmc().bits()))
            .field(
                "eth_pcie_secure",
                &format_args!("{}", self.eth_pcie_secure().bits()),
            )
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<GICD_IPRIORITYR39_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:7 - CPG"]
    #[inline(always)]
    #[must_use]
    pub fn cpg(&mut self) -> CPG_W<GICD_IPRIORITYR39_SPEC, 0> {
        CPG_W::new(self)
    }
    #[doc = "Bits 8:15 - RNG"]
    #[inline(always)]
    #[must_use]
    pub fn rng(&mut self) -> RNG_W<GICD_IPRIORITYR39_SPEC, 8> {
        RNG_W::new(self)
    }
    #[doc = "Bits 16:23 - OR of EMMC and EMMC2"]
    #[inline(always)]
    #[must_use]
    pub fn emmc(&mut self) -> EMMC_W<GICD_IPRIORITYR39_SPEC, 16> {
        EMMC_W::new(self)
    }
    #[doc = "Bits 24:31 - ETH_PCIe secure"]
    #[inline(always)]
    #[must_use]
    pub fn eth_pcie_secure(&mut self) -> ETH_PCIE_SECURE_W<GICD_IPRIORITYR39_SPEC, 24> {
        ETH_PCIE_SECURE_W::new(self)
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
#[doc = "Interrupt Priority 156 - 159 (Lower is first)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_ipriorityr39::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_ipriorityr39::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GICD_IPRIORITYR39_SPEC;
impl crate::RegisterSpec for GICD_IPRIORITYR39_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_ipriorityr39::R`](R) reader structure"]
impl crate::Readable for GICD_IPRIORITYR39_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gicd_ipriorityr39::W`](W) writer structure"]
impl crate::Writable for GICD_IPRIORITYR39_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GICD_IPRIORITYR39 to value 0"]
impl crate::Resettable for GICD_IPRIORITYR39_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
