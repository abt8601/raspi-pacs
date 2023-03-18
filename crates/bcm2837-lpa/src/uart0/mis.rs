#[doc = "Register `MIS` reader"]
pub struct R(crate::R<MIS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MIS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MIS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MIS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RIMMIS` reader - RIMMIS"]
pub type RIMMIS_R = crate::BitReader<bool>;
#[doc = "Field `CTSMMIS` reader - CTSMMIS"]
pub type CTSMMIS_R = crate::BitReader<bool>;
#[doc = "Field `DCDMMIS` reader - DCDMMIS"]
pub type DCDMMIS_R = crate::BitReader<bool>;
#[doc = "Field `DSRMMIS` reader - DSRMMIS"]
pub type DSRMMIS_R = crate::BitReader<bool>;
#[doc = "Field `RXMIS` reader - RXMIS"]
pub type RXMIS_R = crate::BitReader<bool>;
#[doc = "Field `TXMIS` reader - TXMIS"]
pub type TXMIS_R = crate::BitReader<bool>;
#[doc = "Field `RTMIS` reader - RTMIS"]
pub type RTMIS_R = crate::BitReader<bool>;
#[doc = "Field `FEMIS` reader - FEMIS"]
pub type FEMIS_R = crate::BitReader<bool>;
#[doc = "Field `PEMIS` reader - PEMIS"]
pub type PEMIS_R = crate::BitReader<bool>;
#[doc = "Field `BEMIS` reader - BEMIS"]
pub type BEMIS_R = crate::BitReader<bool>;
#[doc = "Field `OEMIS` reader - OEMIS"]
pub type OEMIS_R = crate::BitReader<bool>;
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
#[doc = "Masked Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mis](index.html) module"]
pub struct MIS_SPEC;
impl crate::RegisterSpec for MIS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mis::R](R) reader structure"]
impl crate::Readable for MIS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MIS to value 0"]
impl crate::Resettable for MIS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
