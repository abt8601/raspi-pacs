#[doc = "Register `GICD_ITARGETSR27` reader"]
pub struct R(crate::R<GICD_ITARGETSR27_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GICD_ITARGETSR27_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GICD_ITARGETSR27_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GICD_ITARGETSR27_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GICD_ITARGETSR27` writer"]
pub struct W(crate::W<GICD_ITARGETSR27_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GICD_ITARGETSR27_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<GICD_ITARGETSR27_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GICD_ITARGETSR27_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MULTICORE_SYNC_0` reader - Multicore Sync 0"]
pub type MULTICORE_SYNC_0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MULTICORE_SYNC_0` writer - Multicore Sync 0"]
pub type MULTICORE_SYNC_0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GICD_ITARGETSR27_SPEC, u8, u8, 8, O>;
#[doc = "Field `MULTICORE_SYNC_1` reader - Multicore Sync 1"]
pub type MULTICORE_SYNC_1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MULTICORE_SYNC_1` writer - Multicore Sync 1"]
pub type MULTICORE_SYNC_1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GICD_ITARGETSR27_SPEC, u8, u8, 8, O>;
#[doc = "Field `MULTICORE_SYNC_2` reader - Multicore Sync 2"]
pub type MULTICORE_SYNC_2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MULTICORE_SYNC_2` writer - Multicore Sync 2"]
pub type MULTICORE_SYNC_2_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GICD_ITARGETSR27_SPEC, u8, u8, 8, O>;
#[doc = "Field `MULTICORE_SYNC_3` reader - Multicore Sync 3"]
pub type MULTICORE_SYNC_3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MULTICORE_SYNC_3` writer - Multicore Sync 3"]
pub type MULTICORE_SYNC_3_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GICD_ITARGETSR27_SPEC, u8, u8, 8, O>;
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
impl W {
    #[doc = "Bits 0:7 - Multicore Sync 0"]
    #[inline(always)]
    #[must_use]
    pub fn multicore_sync_0(&mut self) -> MULTICORE_SYNC_0_W<0> {
        MULTICORE_SYNC_0_W::new(self)
    }
    #[doc = "Bits 8:15 - Multicore Sync 1"]
    #[inline(always)]
    #[must_use]
    pub fn multicore_sync_1(&mut self) -> MULTICORE_SYNC_1_W<8> {
        MULTICORE_SYNC_1_W::new(self)
    }
    #[doc = "Bits 16:23 - Multicore Sync 2"]
    #[inline(always)]
    #[must_use]
    pub fn multicore_sync_2(&mut self) -> MULTICORE_SYNC_2_W<16> {
        MULTICORE_SYNC_2_W::new(self)
    }
    #[doc = "Bits 24:31 - Multicore Sync 3"]
    #[inline(always)]
    #[must_use]
    pub fn multicore_sync_3(&mut self) -> MULTICORE_SYNC_3_W<24> {
        MULTICORE_SYNC_3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Processor Target 108 - 111\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_itargetsr27](index.html) module"]
pub struct GICD_ITARGETSR27_SPEC;
impl crate::RegisterSpec for GICD_ITARGETSR27_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gicd_itargetsr27::R](R) reader structure"]
impl crate::Readable for GICD_ITARGETSR27_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gicd_itargetsr27::W](W) writer structure"]
impl crate::Writable for GICD_ITARGETSR27_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GICD_ITARGETSR27 to value 0"]
impl crate::Resettable for GICD_ITARGETSR27_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
