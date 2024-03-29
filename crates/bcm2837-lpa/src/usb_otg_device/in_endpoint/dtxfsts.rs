#[doc = "Register `DTXFSTS` reader"]
pub type R = crate::R<DTXFSTS_SPEC>;
#[doc = "Field `INEPTFSAV` reader - IN endpoint TxFIFO space avail"]
pub type INEPTFSAV_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - IN endpoint TxFIFO space avail"]
    #[inline(always)]
    pub fn ineptfsav(&self) -> INEPTFSAV_R {
        INEPTFSAV_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DTXFSTS")
            .field("ineptfsav", &format_args!("{}", self.ineptfsav().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<DTXFSTS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "Transmit FIFO status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dtxfsts::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DTXFSTS_SPEC;
impl crate::RegisterSpec for DTXFSTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dtxfsts::R`](R) reader structure"]
impl crate::Readable for DTXFSTS_SPEC {}
#[doc = "`reset()` method sets DTXFSTS to value 0"]
impl crate::Resettable for DTXFSTS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
