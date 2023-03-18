#[doc = "Register `DEL` reader"]
pub struct R(crate::R<DEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DEL` writer"]
pub struct W(crate::W<DEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DEL_SPEC>;
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
impl From<crate::W<DEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `REDL` reader - Delay before reading after a rising edge"]
pub type REDL_R = crate::FieldReader<u16, u16>;
#[doc = "Field `REDL` writer - Delay before reading after a rising edge"]
pub type REDL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DEL_SPEC, u16, u16, 16, O>;
#[doc = "Field `FEDL` reader - Delay before reading after a falling edge"]
pub type FEDL_R = crate::FieldReader<u16, u16>;
#[doc = "Field `FEDL` writer - Delay before reading after a falling edge"]
pub type FEDL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DEL_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Delay before reading after a rising edge"]
    #[inline(always)]
    pub fn redl(&self) -> REDL_R {
        REDL_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Delay before reading after a falling edge"]
    #[inline(always)]
    pub fn fedl(&self) -> FEDL_R {
        FEDL_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Delay before reading after a rising edge"]
    #[inline(always)]
    #[must_use]
    pub fn redl(&mut self) -> REDL_W<0> {
        REDL_W::new(self)
    }
    #[doc = "Bits 16:31 - Delay before reading after a falling edge"]
    #[inline(always)]
    #[must_use]
    pub fn fedl(&mut self) -> FEDL_W<16> {
        FEDL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Data delay (Values must be under CDIV / 2)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [del](index.html) module"]
pub struct DEL_SPEC;
impl crate::RegisterSpec for DEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [del::R](R) reader structure"]
impl crate::Readable for DEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [del::W](W) writer structure"]
impl crate::Writable for DEL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DEL to value 0x0030_0030"]
impl crate::Resettable for DEL_SPEC {
    const RESET_VALUE: Self::Ux = 0x0030_0030;
}
