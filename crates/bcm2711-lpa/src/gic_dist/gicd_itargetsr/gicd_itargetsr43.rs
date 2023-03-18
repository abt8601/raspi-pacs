#[doc = "Register `GICD_ITARGETSR43` reader"]
pub struct R(crate::R<GICD_ITARGETSR43_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GICD_ITARGETSR43_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GICD_ITARGETSR43_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GICD_ITARGETSR43_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GICD_ITARGETSR43` writer"]
pub struct W(crate::W<GICD_ITARGETSR43_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GICD_ITARGETSR43_SPEC>;
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
impl From<crate::W<GICD_ITARGETSR43_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GICD_ITARGETSR43_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INT172` reader - Interrupt 172"]
pub type INT172_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INT172` writer - Interrupt 172"]
pub type INT172_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GICD_ITARGETSR43_SPEC, u8, u8, 8, O>;
#[doc = "Field `INT173` reader - Interrupt 173"]
pub type INT173_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INT173` writer - Interrupt 173"]
pub type INT173_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GICD_ITARGETSR43_SPEC, u8, u8, 8, O>;
#[doc = "Field `INT174` reader - Interrupt 174"]
pub type INT174_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INT174` writer - Interrupt 174"]
pub type INT174_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GICD_ITARGETSR43_SPEC, u8, u8, 8, O>;
#[doc = "Field `INT175` reader - Interrupt 175"]
pub type INT175_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INT175` writer - Interrupt 175"]
pub type INT175_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GICD_ITARGETSR43_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Interrupt 172"]
    #[inline(always)]
    pub fn int172(&self) -> INT172_R {
        INT172_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Interrupt 173"]
    #[inline(always)]
    pub fn int173(&self) -> INT173_R {
        INT173_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Interrupt 174"]
    #[inline(always)]
    pub fn int174(&self) -> INT174_R {
        INT174_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Interrupt 175"]
    #[inline(always)]
    pub fn int175(&self) -> INT175_R {
        INT175_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Interrupt 172"]
    #[inline(always)]
    #[must_use]
    pub fn int172(&mut self) -> INT172_W<0> {
        INT172_W::new(self)
    }
    #[doc = "Bits 8:15 - Interrupt 173"]
    #[inline(always)]
    #[must_use]
    pub fn int173(&mut self) -> INT173_W<8> {
        INT173_W::new(self)
    }
    #[doc = "Bits 16:23 - Interrupt 174"]
    #[inline(always)]
    #[must_use]
    pub fn int174(&mut self) -> INT174_W<16> {
        INT174_W::new(self)
    }
    #[doc = "Bits 24:31 - Interrupt 175"]
    #[inline(always)]
    #[must_use]
    pub fn int175(&mut self) -> INT175_W<24> {
        INT175_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Processor Target 172 - 175\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_itargetsr43](index.html) module"]
pub struct GICD_ITARGETSR43_SPEC;
impl crate::RegisterSpec for GICD_ITARGETSR43_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gicd_itargetsr43::R](R) reader structure"]
impl crate::Readable for GICD_ITARGETSR43_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gicd_itargetsr43::W](W) writer structure"]
impl crate::Writable for GICD_ITARGETSR43_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GICD_ITARGETSR43 to value 0"]
impl crate::Resettable for GICD_ITARGETSR43_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
