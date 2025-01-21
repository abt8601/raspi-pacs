#[doc = "Register `DOEPTSIZ` reader"]
pub type R = crate::R<DOEPTSIZ_SPEC>;
#[doc = "Register `DOEPTSIZ` writer"]
pub type W = crate::W<DOEPTSIZ_SPEC>;
#[doc = "Field `XFRSIZ` reader - Transfer size"]
pub type XFRSIZ_R = crate::FieldReader;
#[doc = "Field `XFRSIZ` writer - Transfer size"]
pub type XFRSIZ_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `PKTCNT` reader - Packet count"]
pub type PKTCNT_R = crate::BitReader;
#[doc = "Field `PKTCNT` writer - Packet count"]
pub type PKTCNT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STUPCNT` reader - SETUP packet count"]
pub type STUPCNT_R = crate::FieldReader;
#[doc = "Field `STUPCNT` writer - SETUP packet count"]
pub type STUPCNT_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:6 - Transfer size"]
    #[inline(always)]
    pub fn xfrsiz(&self) -> XFRSIZ_R {
        XFRSIZ_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 19 - Packet count"]
    #[inline(always)]
    pub fn pktcnt(&self) -> PKTCNT_R {
        PKTCNT_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 29:30 - SETUP packet count"]
    #[inline(always)]
    pub fn stupcnt(&self) -> STUPCNT_R {
        STUPCNT_R::new(((self.bits >> 29) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DOEPTSIZ")
            .field("xfrsiz", &format_args!("{}", self.xfrsiz().bits()))
            .field("pktcnt", &format_args!("{}", self.pktcnt().bit()))
            .field("stupcnt", &format_args!("{}", self.stupcnt().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<DOEPTSIZ_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:6 - Transfer size"]
    #[inline(always)]
    #[must_use]
    pub fn xfrsiz(&mut self) -> XFRSIZ_W<DOEPTSIZ_SPEC> {
        XFRSIZ_W::new(self, 0)
    }
    #[doc = "Bit 19 - Packet count"]
    #[inline(always)]
    #[must_use]
    pub fn pktcnt(&mut self) -> PKTCNT_W<DOEPTSIZ_SPEC> {
        PKTCNT_W::new(self, 19)
    }
    #[doc = "Bits 29:30 - SETUP packet count"]
    #[inline(always)]
    #[must_use]
    pub fn stupcnt(&mut self) -> STUPCNT_W<DOEPTSIZ_SPEC> {
        STUPCNT_W::new(self, 29)
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
#[doc = "Transfer size\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doeptsiz::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doeptsiz::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DOEPTSIZ_SPEC;
impl crate::RegisterSpec for DOEPTSIZ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`doeptsiz::R`](R) reader structure"]
impl crate::Readable for DOEPTSIZ_SPEC {}
#[doc = "`write(|w| ..)` method takes [`doeptsiz::W`](W) writer structure"]
impl crate::Writable for DOEPTSIZ_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DOEPTSIZ to value 0"]
impl crate::Resettable for DOEPTSIZ_SPEC {
    const RESET_VALUE: u32 = 0;
}
