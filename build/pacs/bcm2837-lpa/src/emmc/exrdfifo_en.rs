#[doc = "Register `EXRDFIFO_EN` reader"]
pub type R = crate::R<EXRDFIFO_EN_SPEC>;
#[doc = "Register `EXRDFIFO_EN` writer"]
pub type W = crate::W<EXRDFIFO_EN_SPEC>;
#[doc = "Field `ENABLE` reader - Enable the extension FIFO"]
pub type ENABLE_R = crate::BitReader;
#[doc = "Field `ENABLE` writer - Enable the extension FIFO"]
pub type ENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable the extension FIFO"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EXRDFIFO_EN")
            .field("enable", &format_args!("{}", self.enable().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<EXRDFIFO_EN_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - Enable the extension FIFO"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> ENABLE_W<EXRDFIFO_EN_SPEC> {
        ENABLE_W::new(self, 0)
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
#[doc = "Enable the extension data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exrdfifo_en::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`exrdfifo_en::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EXRDFIFO_EN_SPEC;
impl crate::RegisterSpec for EXRDFIFO_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`exrdfifo_en::R`](R) reader structure"]
impl crate::Readable for EXRDFIFO_EN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`exrdfifo_en::W`](W) writer structure"]
impl crate::Writable for EXRDFIFO_EN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EXRDFIFO_EN to value 0"]
impl crate::Resettable for EXRDFIFO_EN_SPEC {
    const RESET_VALUE: u32 = 0;
}
