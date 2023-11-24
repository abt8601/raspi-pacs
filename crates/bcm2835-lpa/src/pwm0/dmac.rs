#[doc = "Register `DMAC` reader"]
pub type R = crate::R<DMAC_SPEC>;
#[doc = "Register `DMAC` writer"]
pub type W = crate::W<DMAC_SPEC>;
#[doc = "Field `DREQ` reader - DMA threshold for DREQ signal"]
pub type DREQ_R = crate::FieldReader;
#[doc = "Field `DREQ` writer - DMA threshold for DREQ signal"]
pub type DREQ_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PANIC` reader - DMA threshold for panic signal"]
pub type PANIC_R = crate::FieldReader;
#[doc = "Field `PANIC` writer - DMA threshold for panic signal"]
pub type PANIC_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `ENAB` reader - DMA enabled"]
pub type ENAB_R = crate::BitReader;
#[doc = "Field `ENAB` writer - DMA enabled"]
pub type ENAB_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - DMA threshold for DREQ signal"]
    #[inline(always)]
    pub fn dreq(&self) -> DREQ_R {
        DREQ_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - DMA threshold for panic signal"]
    #[inline(always)]
    pub fn panic(&self) -> PANIC_R {
        PANIC_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 31 - DMA enabled"]
    #[inline(always)]
    pub fn enab(&self) -> ENAB_R {
        ENAB_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMAC")
            .field("enab", &format_args!("{}", self.enab().bit()))
            .field("panic", &format_args!("{}", self.panic().bits()))
            .field("dreq", &format_args!("{}", self.dreq().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<DMAC_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:7 - DMA threshold for DREQ signal"]
    #[inline(always)]
    #[must_use]
    pub fn dreq(&mut self) -> DREQ_W<DMAC_SPEC> {
        DREQ_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - DMA threshold for panic signal"]
    #[inline(always)]
    #[must_use]
    pub fn panic(&mut self) -> PANIC_W<DMAC_SPEC> {
        PANIC_W::new(self, 8)
    }
    #[doc = "Bit 31 - DMA enabled"]
    #[inline(always)]
    #[must_use]
    pub fn enab(&mut self) -> ENAB_W<DMAC_SPEC> {
        ENAB_W::new(self, 31)
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
#[doc = "DMA control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmac::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmac::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMAC_SPEC;
impl crate::RegisterSpec for DMAC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmac::R`](R) reader structure"]
impl crate::Readable for DMAC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dmac::W`](W) writer structure"]
impl crate::Writable for DMAC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMAC to value 0"]
impl crate::Resettable for DMAC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
