#[doc = "Register `GICD_ITARGETSR27` reader"]
pub type R = crate::R<GICD_ITARGETSR27_SPEC>;
#[doc = "Register `GICD_ITARGETSR27` writer"]
pub type W = crate::W<GICD_ITARGETSR27_SPEC>;
#[doc = "Field `MULTICORE_SYNC_0` reader - Multicore Sync 0"]
pub type MULTICORE_SYNC_0_R = crate::FieldReader;
#[doc = "Field `MULTICORE_SYNC_0` writer - Multicore Sync 0"]
pub type MULTICORE_SYNC_0_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `MULTICORE_SYNC_1` reader - Multicore Sync 1"]
pub type MULTICORE_SYNC_1_R = crate::FieldReader;
#[doc = "Field `MULTICORE_SYNC_1` writer - Multicore Sync 1"]
pub type MULTICORE_SYNC_1_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `MULTICORE_SYNC_2` reader - Multicore Sync 2"]
pub type MULTICORE_SYNC_2_R = crate::FieldReader;
#[doc = "Field `MULTICORE_SYNC_2` writer - Multicore Sync 2"]
pub type MULTICORE_SYNC_2_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `MULTICORE_SYNC_3` reader - Multicore Sync 3"]
pub type MULTICORE_SYNC_3_R = crate::FieldReader;
#[doc = "Field `MULTICORE_SYNC_3` writer - Multicore Sync 3"]
pub type MULTICORE_SYNC_3_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Multicore Sync 0"]
    #[inline(always)]
    pub fn multicore_sync_0(&self) -> MULTICORE_SYNC_0_R {
        MULTICORE_SYNC_0_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Multicore Sync 1"]
    #[inline(always)]
    pub fn multicore_sync_1(&self) -> MULTICORE_SYNC_1_R {
        MULTICORE_SYNC_1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Multicore Sync 2"]
    #[inline(always)]
    pub fn multicore_sync_2(&self) -> MULTICORE_SYNC_2_R {
        MULTICORE_SYNC_2_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Multicore Sync 3"]
    #[inline(always)]
    pub fn multicore_sync_3(&self) -> MULTICORE_SYNC_3_R {
        MULTICORE_SYNC_3_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICD_ITARGETSR27")
            .field(
                "multicore_sync_0",
                &format_args!("{}", self.multicore_sync_0().bits()),
            )
            .field(
                "multicore_sync_1",
                &format_args!("{}", self.multicore_sync_1().bits()),
            )
            .field(
                "multicore_sync_2",
                &format_args!("{}", self.multicore_sync_2().bits()),
            )
            .field(
                "multicore_sync_3",
                &format_args!("{}", self.multicore_sync_3().bits()),
            )
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<GICD_ITARGETSR27_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:7 - Multicore Sync 0"]
    #[inline(always)]
    #[must_use]
    pub fn multicore_sync_0(&mut self) -> MULTICORE_SYNC_0_W<GICD_ITARGETSR27_SPEC> {
        MULTICORE_SYNC_0_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Multicore Sync 1"]
    #[inline(always)]
    #[must_use]
    pub fn multicore_sync_1(&mut self) -> MULTICORE_SYNC_1_W<GICD_ITARGETSR27_SPEC> {
        MULTICORE_SYNC_1_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Multicore Sync 2"]
    #[inline(always)]
    #[must_use]
    pub fn multicore_sync_2(&mut self) -> MULTICORE_SYNC_2_W<GICD_ITARGETSR27_SPEC> {
        MULTICORE_SYNC_2_W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Multicore Sync 3"]
    #[inline(always)]
    #[must_use]
    pub fn multicore_sync_3(&mut self) -> MULTICORE_SYNC_3_W<GICD_ITARGETSR27_SPEC> {
        MULTICORE_SYNC_3_W::new(self, 24)
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
#[doc = "Interrupt Processor Target 108 - 111\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_itargetsr27::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_itargetsr27::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GICD_ITARGETSR27_SPEC;
impl crate::RegisterSpec for GICD_ITARGETSR27_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_itargetsr27::R`](R) reader structure"]
impl crate::Readable for GICD_ITARGETSR27_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gicd_itargetsr27::W`](W) writer structure"]
impl crate::Writable for GICD_ITARGETSR27_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GICD_ITARGETSR27 to value 0"]
impl crate::Resettable for GICD_ITARGETSR27_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
