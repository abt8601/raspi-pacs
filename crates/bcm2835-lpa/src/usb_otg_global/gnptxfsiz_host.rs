#[doc = "Register `GNPTXFSIZ_Host` reader"]
pub type R = crate::R<GNPTXFSIZ_HOST_SPEC>;
#[doc = "Register `GNPTXFSIZ_Host` writer"]
pub type W = crate::W<GNPTXFSIZ_HOST_SPEC>;
#[doc = "Field `NPTXFSA` reader - Nonperiodic transmit RAM start address"]
pub type NPTXFSA_R = crate::FieldReader<u16>;
#[doc = "Field `NPTXFSA` writer - Nonperiodic transmit RAM start address"]
pub type NPTXFSA_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `NPTXFD` reader - Nonperiodic TxFIFO depth"]
pub type NPTXFD_R = crate::FieldReader<u16>;
#[doc = "Field `NPTXFD` writer - Nonperiodic TxFIFO depth"]
pub type NPTXFD_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Nonperiodic transmit RAM start address"]
    #[inline(always)]
    pub fn nptxfsa(&self) -> NPTXFSA_R {
        NPTXFSA_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Nonperiodic TxFIFO depth"]
    #[inline(always)]
    pub fn nptxfd(&self) -> NPTXFD_R {
        NPTXFD_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GNPTXFSIZ_Host")
            .field("nptxfsa", &format_args!("{}", self.nptxfsa().bits()))
            .field("nptxfd", &format_args!("{}", self.nptxfd().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<GNPTXFSIZ_HOST_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:15 - Nonperiodic transmit RAM start address"]
    #[inline(always)]
    #[must_use]
    pub fn nptxfsa(&mut self) -> NPTXFSA_W<GNPTXFSIZ_HOST_SPEC> {
        NPTXFSA_W::new(self, 0)
    }
    #[doc = "Bits 16:31 - Nonperiodic TxFIFO depth"]
    #[inline(always)]
    #[must_use]
    pub fn nptxfd(&mut self) -> NPTXFD_W<GNPTXFSIZ_HOST_SPEC> {
        NPTXFD_W::new(self, 16)
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
#[doc = "OTG_HS nonperiodic transmit FIFO size register (host mode)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gnptxfsiz_host::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gnptxfsiz_host::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GNPTXFSIZ_HOST_SPEC;
impl crate::RegisterSpec for GNPTXFSIZ_HOST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gnptxfsiz_host::R`](R) reader structure"]
impl crate::Readable for GNPTXFSIZ_HOST_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gnptxfsiz_host::W`](W) writer structure"]
impl crate::Writable for GNPTXFSIZ_HOST_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GNPTXFSIZ_Host to value 0x0200"]
impl crate::Resettable for GNPTXFSIZ_HOST_SPEC {
    const RESET_VALUE: Self::Ux = 0x0200;
}
