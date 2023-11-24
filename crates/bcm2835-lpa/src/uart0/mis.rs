#[doc = "Register `MIS` reader"]
pub type R = crate::R<MIS_SPEC>;
#[doc = "Field `RIMMIS` reader - RIMMIS"]
pub type RIMMIS_R = crate::BitReader;
#[doc = "Field `CTSMMIS` reader - CTSMMIS"]
pub type CTSMMIS_R = crate::BitReader;
#[doc = "Field `DCDMMIS` reader - DCDMMIS"]
pub type DCDMMIS_R = crate::BitReader;
#[doc = "Field `DSRMMIS` reader - DSRMMIS"]
pub type DSRMMIS_R = crate::BitReader;
#[doc = "Field `RXMIS` reader - RXMIS"]
pub type RXMIS_R = crate::BitReader;
#[doc = "Field `TXMIS` reader - TXMIS"]
pub type TXMIS_R = crate::BitReader;
#[doc = "Field `RTMIS` reader - RTMIS"]
pub type RTMIS_R = crate::BitReader;
#[doc = "Field `FEMIS` reader - FEMIS"]
pub type FEMIS_R = crate::BitReader;
#[doc = "Field `PEMIS` reader - PEMIS"]
pub type PEMIS_R = crate::BitReader;
#[doc = "Field `BEMIS` reader - BEMIS"]
pub type BEMIS_R = crate::BitReader;
#[doc = "Field `OEMIS` reader - OEMIS"]
pub type OEMIS_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - RIMMIS"]
    #[inline(always)]
    pub fn rimmis(&self) -> RIMMIS_R {
        RIMMIS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CTSMMIS"]
    #[inline(always)]
    pub fn ctsmmis(&self) -> CTSMMIS_R {
        CTSMMIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DCDMMIS"]
    #[inline(always)]
    pub fn dcdmmis(&self) -> DCDMMIS_R {
        DCDMMIS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DSRMMIS"]
    #[inline(always)]
    pub fn dsrmmis(&self) -> DSRMMIS_R {
        DSRMMIS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - RXMIS"]
    #[inline(always)]
    pub fn rxmis(&self) -> RXMIS_R {
        RXMIS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TXMIS"]
    #[inline(always)]
    pub fn txmis(&self) -> TXMIS_R {
        TXMIS_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - RTMIS"]
    #[inline(always)]
    pub fn rtmis(&self) -> RTMIS_R {
        RTMIS_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - FEMIS"]
    #[inline(always)]
    pub fn femis(&self) -> FEMIS_R {
        FEMIS_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - PEMIS"]
    #[inline(always)]
    pub fn pemis(&self) -> PEMIS_R {
        PEMIS_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - BEMIS"]
    #[inline(always)]
    pub fn bemis(&self) -> BEMIS_R {
        BEMIS_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - OEMIS"]
    #[inline(always)]
    pub fn oemis(&self) -> OEMIS_R {
        OEMIS_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MIS")
            .field("rimmis", &format_args!("{}", self.rimmis().bit()))
            .field("ctsmmis", &format_args!("{}", self.ctsmmis().bit()))
            .field("dcdmmis", &format_args!("{}", self.dcdmmis().bit()))
            .field("dsrmmis", &format_args!("{}", self.dsrmmis().bit()))
            .field("rxmis", &format_args!("{}", self.rxmis().bit()))
            .field("txmis", &format_args!("{}", self.txmis().bit()))
            .field("rtmis", &format_args!("{}", self.rtmis().bit()))
            .field("femis", &format_args!("{}", self.femis().bit()))
            .field("pemis", &format_args!("{}", self.pemis().bit()))
            .field("bemis", &format_args!("{}", self.bemis().bit()))
            .field("oemis", &format_args!("{}", self.oemis().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<MIS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "Masked Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mis::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MIS_SPEC;
impl crate::RegisterSpec for MIS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mis::R`](R) reader structure"]
impl crate::Readable for MIS_SPEC {}
#[doc = "`reset()` method sets MIS to value 0"]
impl crate::Resettable for MIS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
