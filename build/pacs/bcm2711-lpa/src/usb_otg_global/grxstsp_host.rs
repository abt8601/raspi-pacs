#[doc = "Register `GRXSTSP_Host` reader"]
pub type R = crate::R<GRXSTSP_HOST_SPEC>;
#[doc = "Field `CHNUM` reader - Channel number"]
pub type CHNUM_R = crate::FieldReader;
#[doc = "Field `BCNT` reader - Byte count"]
pub type BCNT_R = crate::FieldReader<u16>;
#[doc = "Field `DPID` reader - Data PID"]
pub type DPID_R = crate::FieldReader;
#[doc = "Field `PKTSTS` reader - Packet status"]
pub type PKTSTS_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - Channel number"]
    #[inline(always)]
    pub fn chnum(&self) -> CHNUM_R {
        CHNUM_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:14 - Byte count"]
    #[inline(always)]
    pub fn bcnt(&self) -> BCNT_R {
        BCNT_R::new(((self.bits >> 4) & 0x07ff) as u16)
    }
    #[doc = "Bits 15:16 - Data PID"]
    #[inline(always)]
    pub fn dpid(&self) -> DPID_R {
        DPID_R::new(((self.bits >> 15) & 3) as u8)
    }
    #[doc = "Bits 17:20 - Packet status"]
    #[inline(always)]
    pub fn pktsts(&self) -> PKTSTS_R {
        PKTSTS_R::new(((self.bits >> 17) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GRXSTSP_Host")
            .field("chnum", &format_args!("{}", self.chnum().bits()))
            .field("bcnt", &format_args!("{}", self.bcnt().bits()))
            .field("dpid", &format_args!("{}", self.dpid().bits()))
            .field("pktsts", &format_args!("{}", self.pktsts().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<GRXSTSP_HOST_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "OTG_HS status read and pop register (host mode)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grxstsp_host::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GRXSTSP_HOST_SPEC;
impl crate::RegisterSpec for GRXSTSP_HOST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`grxstsp_host::R`](R) reader structure"]
impl crate::Readable for GRXSTSP_HOST_SPEC {}
#[doc = "`reset()` method sets GRXSTSP_Host to value 0"]
impl crate::Resettable for GRXSTSP_HOST_SPEC {
    const RESET_VALUE: u32 = 0;
}
