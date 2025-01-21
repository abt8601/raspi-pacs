#[doc = "Register `RIS` reader"]
pub type R = crate::R<RIS_SPEC>;
#[doc = "Field `RIRMIS` reader - RIRMIS"]
pub type RIRMIS_R = crate::BitReader;
#[doc = "Field `CTSRMIS` reader - CTSRMIS"]
pub type CTSRMIS_R = crate::BitReader;
#[doc = "Field `DCDRMIS` reader - DCDRMIS"]
pub type DCDRMIS_R = crate::BitReader;
#[doc = "Field `DSRRMIS` reader - DSRRMIS"]
pub type DSRRMIS_R = crate::BitReader;
#[doc = "Field `RXRIS` reader - RXRIS"]
pub type RXRIS_R = crate::BitReader;
#[doc = "Field `TXRIS` reader - TXRIS"]
pub type TXRIS_R = crate::BitReader;
#[doc = "Field `RTRIS` reader - RTRIS"]
pub type RTRIS_R = crate::BitReader;
#[doc = "Field `FERIS` reader - FERIS"]
pub type FERIS_R = crate::BitReader;
#[doc = "Field `PERIS` reader - PERIS"]
pub type PERIS_R = crate::BitReader;
#[doc = "Field `BERIS` reader - BERIS"]
pub type BERIS_R = crate::BitReader;
#[doc = "Field `OERIS` reader - OERIS"]
pub type OERIS_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - RIRMIS"]
    #[inline(always)]
    pub fn rirmis(&self) -> RIRMIS_R {
        RIRMIS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CTSRMIS"]
    #[inline(always)]
    pub fn ctsrmis(&self) -> CTSRMIS_R {
        CTSRMIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DCDRMIS"]
    #[inline(always)]
    pub fn dcdrmis(&self) -> DCDRMIS_R {
        DCDRMIS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DSRRMIS"]
    #[inline(always)]
    pub fn dsrrmis(&self) -> DSRRMIS_R {
        DSRRMIS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - RXRIS"]
    #[inline(always)]
    pub fn rxris(&self) -> RXRIS_R {
        RXRIS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TXRIS"]
    #[inline(always)]
    pub fn txris(&self) -> TXRIS_R {
        TXRIS_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - RTRIS"]
    #[inline(always)]
    pub fn rtris(&self) -> RTRIS_R {
        RTRIS_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - FERIS"]
    #[inline(always)]
    pub fn feris(&self) -> FERIS_R {
        FERIS_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - PERIS"]
    #[inline(always)]
    pub fn peris(&self) -> PERIS_R {
        PERIS_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - BERIS"]
    #[inline(always)]
    pub fn beris(&self) -> BERIS_R {
        BERIS_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - OERIS"]
    #[inline(always)]
    pub fn oeris(&self) -> OERIS_R {
        OERIS_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RIS")
            .field("rirmis", &format_args!("{}", self.rirmis().bit()))
            .field("ctsrmis", &format_args!("{}", self.ctsrmis().bit()))
            .field("dcdrmis", &format_args!("{}", self.dcdrmis().bit()))
            .field("dsrrmis", &format_args!("{}", self.dsrrmis().bit()))
            .field("rxris", &format_args!("{}", self.rxris().bit()))
            .field("txris", &format_args!("{}", self.txris().bit()))
            .field("rtris", &format_args!("{}", self.rtris().bit()))
            .field("feris", &format_args!("{}", self.feris().bit()))
            .field("peris", &format_args!("{}", self.peris().bit()))
            .field("beris", &format_args!("{}", self.beris().bit()))
            .field("oeris", &format_args!("{}", self.oeris().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<RIS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "Raw Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ris::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RIS_SPEC;
impl crate::RegisterSpec for RIS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ris::R`](R) reader structure"]
impl crate::Readable for RIS_SPEC {}
#[doc = "`reset()` method sets RIS to value 0"]
impl crate::Resettable for RIS_SPEC {
    const RESET_VALUE: u32 = 0;
}
