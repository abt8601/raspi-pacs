#[doc = "Register `A` reader"]
pub struct R(crate::R<A_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<A_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<A_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<A_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `A` writer"]
pub struct W(crate::W<A_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<A_SPEC>;
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
impl From<crate::W<A_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<A_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADDR` reader - Slave address"]
pub type ADDR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADDR` writer - Slave address"]
pub type ADDR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, A_SPEC, u8, u8, 7, O>;
impl R {
    #[doc = "Bits 0:6 - Slave address"]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Slave address"]
    #[inline(always)]
    #[must_use]
    pub fn addr(&mut self) -> ADDR_W<0> {
        ADDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Slave address\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [a](index.html) module"]
pub struct A_SPEC;
impl crate::RegisterSpec for A_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [a::R](R) reader structure"]
impl crate::Readable for A_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [a::W](W) writer structure"]
impl crate::Writable for A_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets A to value 0"]
impl crate::Resettable for A_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
