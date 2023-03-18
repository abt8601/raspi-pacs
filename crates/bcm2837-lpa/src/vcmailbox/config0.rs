#[doc = "Register `CONFIG0` reader"]
pub struct R(crate::R<CONFIG0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONFIG0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONFIG0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONFIG0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CONFIG0` writer"]
pub struct W(crate::W<CONFIG0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONFIG0_SPEC>;
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
impl From<crate::W<CONFIG0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONFIG0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IRQEN` reader - Enable the interrupt when data is available"]
pub type IRQEN_R = crate::BitReader<bool>;
#[doc = "Field `IRQEN` writer - Enable the interrupt when data is available"]
pub type IRQEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONFIG0_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Enable the interrupt when data is available"]
    #[inline(always)]
    pub fn irqen(&self) -> IRQEN_R {
        IRQEN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable the interrupt when data is available"]
    #[inline(always)]
    #[must_use]
    pub fn irqen(&mut self) -> IRQEN_W<0> {
        IRQEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [config0](index.html) module"]
pub struct CONFIG0_SPEC;
impl crate::RegisterSpec for CONFIG0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [config0::R](R) reader structure"]
impl crate::Readable for CONFIG0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [config0::W](W) writer structure"]
impl crate::Writable for CONFIG0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
