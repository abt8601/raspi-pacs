#[doc = "Register `DSTS` reader"]
pub type R = crate::R<DSTS_SPEC>;
#[doc = "Field `SUSPSTS` reader - Suspend status"]
pub type SUSPSTS_R = crate::BitReader;
#[doc = "Field `ENUMSPD` reader - Enumerated speed"]
pub type ENUMSPD_R = crate::FieldReader;
#[doc = "Field `EERR` reader - Erratic error"]
pub type EERR_R = crate::BitReader;
#[doc = "Field `FNSOF` reader - Frame number of the received SOF"]
pub type FNSOF_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bit 0 - Suspend status"]
    #[inline(always)]
    pub fn suspsts(&self) -> SUSPSTS_R {
        SUSPSTS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Enumerated speed"]
    #[inline(always)]
    pub fn enumspd(&self) -> ENUMSPD_R {
        ENUMSPD_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 3 - Erratic error"]
    #[inline(always)]
    pub fn eerr(&self) -> EERR_R {
        EERR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 8:21 - Frame number of the received SOF"]
    #[inline(always)]
    pub fn fnsof(&self) -> FNSOF_R {
        FNSOF_R::new(((self.bits >> 8) & 0x3fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DSTS")
            .field("suspsts", &format_args!("{}", self.suspsts().bit()))
            .field("enumspd", &format_args!("{}", self.enumspd().bits()))
            .field("eerr", &format_args!("{}", self.eerr().bit()))
            .field("fnsof", &format_args!("{}", self.fnsof().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<DSTS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "OTG_HS device status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsts::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DSTS_SPEC;
impl crate::RegisterSpec for DSTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsts::R`](R) reader structure"]
impl crate::Readable for DSTS_SPEC {}
#[doc = "`reset()` method sets DSTS to value 0x10"]
impl crate::Resettable for DSTS_SPEC {
    const RESET_VALUE: u32 = 0x10;
}
