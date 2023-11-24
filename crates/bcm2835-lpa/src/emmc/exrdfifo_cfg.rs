#[doc = "Register `EXRDFIFO_CFG` reader"]
pub type R = crate::R<EXRDFIFO_CFG_SPEC>;
#[doc = "Register `EXRDFIFO_CFG` writer"]
pub type W = crate::W<EXRDFIFO_CFG_SPEC>;
#[doc = "Field `RD_THRSH` reader - Read threshold in 32 bit words"]
pub type RD_THRSH_R = crate::FieldReader;
#[doc = "Field `RD_THRSH` writer - Read threshold in 32 bit words"]
pub type RD_THRSH_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - Read threshold in 32 bit words"]
    #[inline(always)]
    pub fn rd_thrsh(&self) -> RD_THRSH_R {
        RD_THRSH_R::new((self.bits & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EXRDFIFO_CFG")
            .field("rd_thrsh", &format_args!("{}", self.rd_thrsh().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<EXRDFIFO_CFG_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:2 - Read threshold in 32 bit words"]
    #[inline(always)]
    #[must_use]
    pub fn rd_thrsh(&mut self) -> RD_THRSH_W<EXRDFIFO_CFG_SPEC> {
        RD_THRSH_W::new(self, 0)
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
#[doc = "Fine tune DMA request generation\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exrdfifo_cfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`exrdfifo_cfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EXRDFIFO_CFG_SPEC;
impl crate::RegisterSpec for EXRDFIFO_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`exrdfifo_cfg::R`](R) reader structure"]
impl crate::Readable for EXRDFIFO_CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`exrdfifo_cfg::W`](W) writer structure"]
impl crate::Writable for EXRDFIFO_CFG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EXRDFIFO_CFG to value 0"]
impl crate::Resettable for EXRDFIFO_CFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
