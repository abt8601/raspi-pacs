#[doc = "Register `DIEPTXF3` reader"]
pub type R = crate::R<DIEPTXF3_SPEC>;
#[doc = "Register `DIEPTXF3` writer"]
pub type W = crate::W<DIEPTXF3_SPEC>;
#[doc = "Field `INEPTXSA` reader - IN endpoint FIFOx transmit RAM start address"]
pub type INEPTXSA_R = crate::FieldReader<u16>;
#[doc = "Field `INEPTXSA` writer - IN endpoint FIFOx transmit RAM start address"]
pub type INEPTXSA_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `INEPTXFD` reader - IN endpoint TxFIFO depth"]
pub type INEPTXFD_R = crate::FieldReader<u16>;
#[doc = "Field `INEPTXFD` writer - IN endpoint TxFIFO depth"]
pub type INEPTXFD_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - IN endpoint FIFOx transmit RAM start address"]
    #[inline(always)]
    pub fn ineptxsa(&self) -> INEPTXSA_R {
        INEPTXSA_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - IN endpoint TxFIFO depth"]
    #[inline(always)]
    pub fn ineptxfd(&self) -> INEPTXFD_R {
        INEPTXFD_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DIEPTXF3")
            .field("ineptxsa", &format_args!("{}", self.ineptxsa().bits()))
            .field("ineptxfd", &format_args!("{}", self.ineptxfd().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<DIEPTXF3_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:15 - IN endpoint FIFOx transmit RAM start address"]
    #[inline(always)]
    #[must_use]
    pub fn ineptxsa(&mut self) -> INEPTXSA_W<DIEPTXF3_SPEC> {
        INEPTXSA_W::new(self, 0)
    }
    #[doc = "Bits 16:31 - IN endpoint TxFIFO depth"]
    #[inline(always)]
    #[must_use]
    pub fn ineptxfd(&mut self) -> INEPTXFD_W<DIEPTXF3_SPEC> {
        INEPTXFD_W::new(self, 16)
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
#[doc = "OTG_HS device IN endpoint transmit FIFO size register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dieptxf3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dieptxf3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DIEPTXF3_SPEC;
impl crate::RegisterSpec for DIEPTXF3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dieptxf3::R`](R) reader structure"]
impl crate::Readable for DIEPTXF3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dieptxf3::W`](W) writer structure"]
impl crate::Writable for DIEPTXF3_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DIEPTXF3 to value 0x0200_0400"]
impl crate::Resettable for DIEPTXF3_SPEC {
    const RESET_VALUE: Self::Ux = 0x0200_0400;
}
