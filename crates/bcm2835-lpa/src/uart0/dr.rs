#[doc = "Register `DR` reader"]
pub struct R(crate::R<DR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DR` writer"]
pub struct W(crate::W<DR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DR_SPEC>;
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
impl From<crate::W<DR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATA` reader - DATA"]
pub type DATA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DATA` writer - DATA"]
pub type DATA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DR_SPEC, u8, u8, 8, O>;
#[doc = "Field `FE` reader - FE"]
pub type FE_R = crate::BitReader<bool>;
#[doc = "Field `FE` writer - FE"]
pub type FE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DR_SPEC, bool, O>;
#[doc = "Field `PE` reader - PE"]
pub type PE_R = crate::BitReader<bool>;
#[doc = "Field `PE` writer - PE"]
pub type PE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DR_SPEC, bool, O>;
#[doc = "Field `BE` reader - BE"]
pub type BE_R = crate::BitReader<bool>;
#[doc = "Field `BE` writer - BE"]
pub type BE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DR_SPEC, bool, O>;
#[doc = "Field `OE` reader - OE"]
pub type OE_R = crate::BitReader<bool>;
#[doc = "Field `OE` writer - OE"]
pub type OE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DR_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:7 - DATA"]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - FE"]
    #[inline(always)]
    pub fn fe(&self) -> FE_R {
        FE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - PE"]
    #[inline(always)]
    pub fn pe(&self) -> PE_R {
        PE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - BE"]
    #[inline(always)]
    pub fn be(&self) -> BE_R {
        BE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - OE"]
    #[inline(always)]
    pub fn oe(&self) -> OE_R {
        OE_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - DATA"]
    #[inline(always)]
    #[must_use]
    pub fn data(&mut self) -> DATA_W<0> {
        DATA_W::new(self)
    }
    #[doc = "Bit 8 - FE"]
    #[inline(always)]
    #[must_use]
    pub fn fe(&mut self) -> FE_W<8> {
        FE_W::new(self)
    }
    #[doc = "Bit 9 - PE"]
    #[inline(always)]
    #[must_use]
    pub fn pe(&mut self) -> PE_W<9> {
        PE_W::new(self)
    }
    #[doc = "Bit 10 - BE"]
    #[inline(always)]
    #[must_use]
    pub fn be(&mut self) -> BE_W<10> {
        BE_W::new(self)
    }
    #[doc = "Bit 11 - OE"]
    #[inline(always)]
    #[must_use]
    pub fn oe(&mut self) -> OE_W<11> {
        OE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Data Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dr](index.html) module"]
pub struct DR_SPEC;
impl crate::RegisterSpec for DR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dr::R](R) reader structure"]
impl crate::Readable for DR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dr::W](W) writer structure"]
impl crate::Writable for DR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DR to value 0"]
impl crate::Resettable for DR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
