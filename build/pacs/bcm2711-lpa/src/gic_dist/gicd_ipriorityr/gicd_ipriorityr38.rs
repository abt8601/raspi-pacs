#[doc = "Register `GICD_IPRIORITYR38` reader"]
pub type R = crate::R<GICD_IPRIORITYR38_SPEC>;
#[doc = "Register `GICD_IPRIORITYR38` writer"]
pub type W = crate::W<GICD_IPRIORITYR38_SPEC>;
#[doc = "Field `SDHOST` reader - SDHOST"]
pub type SDHOST_R = crate::FieldReader;
#[doc = "Field `SDHOST` writer - SDHOST"]
pub type SDHOST_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `UART` reader - OR of all PL011 UARTs"]
pub type UART_R = crate::FieldReader;
#[doc = "Field `UART` writer - OR of all PL011 UARTs"]
pub type UART_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `ETH_PCIE` reader - OR of all ETH_PCIe L2"]
pub type ETH_PCIE_R = crate::FieldReader;
#[doc = "Field `ETH_PCIE` writer - OR of all ETH_PCIe L2"]
pub type ETH_PCIE_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `VEC` reader - VEC"]
pub type VEC_R = crate::FieldReader;
#[doc = "Field `VEC` writer - VEC"]
pub type VEC_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - SDHOST"]
    #[inline(always)]
    pub fn sdhost(&self) -> SDHOST_R {
        SDHOST_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - OR of all PL011 UARTs"]
    #[inline(always)]
    pub fn uart(&self) -> UART_R {
        UART_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - OR of all ETH_PCIe L2"]
    #[inline(always)]
    pub fn eth_pcie(&self) -> ETH_PCIE_R {
        ETH_PCIE_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - VEC"]
    #[inline(always)]
    pub fn vec(&self) -> VEC_R {
        VEC_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICD_IPRIORITYR38")
            .field("sdhost", &format_args!("{}", self.sdhost().bits()))
            .field("uart", &format_args!("{}", self.uart().bits()))
            .field("eth_pcie", &format_args!("{}", self.eth_pcie().bits()))
            .field("vec", &format_args!("{}", self.vec().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<GICD_IPRIORITYR38_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:7 - SDHOST"]
    #[inline(always)]
    #[must_use]
    pub fn sdhost(&mut self) -> SDHOST_W<GICD_IPRIORITYR38_SPEC> {
        SDHOST_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - OR of all PL011 UARTs"]
    #[inline(always)]
    #[must_use]
    pub fn uart(&mut self) -> UART_W<GICD_IPRIORITYR38_SPEC> {
        UART_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - OR of all ETH_PCIe L2"]
    #[inline(always)]
    #[must_use]
    pub fn eth_pcie(&mut self) -> ETH_PCIE_W<GICD_IPRIORITYR38_SPEC> {
        ETH_PCIE_W::new(self, 16)
    }
    #[doc = "Bits 24:31 - VEC"]
    #[inline(always)]
    #[must_use]
    pub fn vec(&mut self) -> VEC_W<GICD_IPRIORITYR38_SPEC> {
        VEC_W::new(self, 24)
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
#[doc = "Interrupt Priority 152 - 155 (Lower is first)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_ipriorityr38::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_ipriorityr38::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GICD_IPRIORITYR38_SPEC;
impl crate::RegisterSpec for GICD_IPRIORITYR38_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_ipriorityr38::R`](R) reader structure"]
impl crate::Readable for GICD_IPRIORITYR38_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gicd_ipriorityr38::W`](W) writer structure"]
impl crate::Writable for GICD_IPRIORITYR38_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GICD_IPRIORITYR38 to value 0"]
impl crate::Resettable for GICD_IPRIORITYR38_SPEC {
    const RESET_VALUE: u32 = 0;
}
