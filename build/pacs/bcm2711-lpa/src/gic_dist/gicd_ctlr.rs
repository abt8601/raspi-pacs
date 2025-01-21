#[doc = "Register `GICD_CTLR` reader"]
pub type R = crate::R<GICD_CTLR_SPEC>;
#[doc = "Register `GICD_CTLR` writer"]
pub type W = crate::W<GICD_CTLR_SPEC>;
#[doc = "Field `ENABLE_GROUP0` reader - Enable group 0 interrupts"]
pub type ENABLE_GROUP0_R = crate::BitReader;
#[doc = "Field `ENABLE_GROUP0` writer - Enable group 0 interrupts"]
pub type ENABLE_GROUP0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENABLE_GROUP1` reader - Enable group 1 interrupts"]
pub type ENABLE_GROUP1_R = crate::BitReader;
#[doc = "Field `ENABLE_GROUP1` writer - Enable group 1 interrupts"]
pub type ENABLE_GROUP1_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable group 0 interrupts"]
    #[inline(always)]
    pub fn enable_group0(&self) -> ENABLE_GROUP0_R {
        ENABLE_GROUP0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable group 1 interrupts"]
    #[inline(always)]
    pub fn enable_group1(&self) -> ENABLE_GROUP1_R {
        ENABLE_GROUP1_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICD_CTLR")
            .field(
                "enable_group0",
                &format_args!("{}", self.enable_group0().bit()),
            )
            .field(
                "enable_group1",
                &format_args!("{}", self.enable_group1().bit()),
            )
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<GICD_CTLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - Enable group 0 interrupts"]
    #[inline(always)]
    #[must_use]
    pub fn enable_group0(&mut self) -> ENABLE_GROUP0_W<GICD_CTLR_SPEC> {
        ENABLE_GROUP0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Enable group 1 interrupts"]
    #[inline(always)]
    #[must_use]
    pub fn enable_group1(&mut self) -> ENABLE_GROUP1_W<GICD_CTLR_SPEC> {
        ENABLE_GROUP1_W::new(self, 1)
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
#[doc = "Distributor Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_ctlr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_ctlr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GICD_CTLR_SPEC;
impl crate::RegisterSpec for GICD_CTLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_ctlr::R`](R) reader structure"]
impl crate::Readable for GICD_CTLR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gicd_ctlr::W`](W) writer structure"]
impl crate::Writable for GICD_CTLR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GICD_CTLR to value 0"]
impl crate::Resettable for GICD_CTLR_SPEC {
    const RESET_VALUE: u32 = 0;
}
