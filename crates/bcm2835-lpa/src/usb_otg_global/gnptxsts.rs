#[doc = "Register `GNPTXSTS` reader"]
pub type R = crate::R<GNPTXSTS_SPEC>;
#[doc = "Field `NPTXFSAV` reader - Nonperiodic TxFIFO space available"]
pub type NPTXFSAV_R = crate::FieldReader<u16>;
#[doc = "Field `NPTQXSAV` reader - Nonperiodic transmit request queue space available"]
pub type NPTQXSAV_R = crate::FieldReader;
#[doc = "Field `NPTXQTOP` reader - Top of the nonperiodic transmit request queue"]
pub type NPTXQTOP_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:15 - Nonperiodic TxFIFO space available"]
    #[inline(always)]
    pub fn nptxfsav(&self) -> NPTXFSAV_R {
        NPTXFSAV_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:23 - Nonperiodic transmit request queue space available"]
    #[inline(always)]
    pub fn nptqxsav(&self) -> NPTQXSAV_R {
        NPTQXSAV_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:30 - Top of the nonperiodic transmit request queue"]
    #[inline(always)]
    pub fn nptxqtop(&self) -> NPTXQTOP_R {
        NPTXQTOP_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GNPTXSTS")
            .field("nptxfsav", &format_args!("{}", self.nptxfsav().bits()))
            .field("nptqxsav", &format_args!("{}", self.nptqxsav().bits()))
            .field("nptxqtop", &format_args!("{}", self.nptxqtop().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<GNPTXSTS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "OTG_HS nonperiodic transmit FIFO/queue status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gnptxsts::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GNPTXSTS_SPEC;
impl crate::RegisterSpec for GNPTXSTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gnptxsts::R`](R) reader structure"]
impl crate::Readable for GNPTXSTS_SPEC {}
#[doc = "`reset()` method sets GNPTXSTS to value 0x0008_0200"]
impl crate::Resettable for GNPTXSTS_SPEC {
    const RESET_VALUE: Self::Ux = 0x0008_0200;
}
