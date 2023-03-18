#[doc = "Register `GICC_IIDR` reader"]
pub struct R(crate::R<GICC_IIDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GICC_IIDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GICC_IIDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GICC_IIDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GICC_IIDR` writer"]
pub struct W(crate::W<GICC_IIDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GICC_IIDR_SPEC>;
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
impl From<crate::W<GICC_IIDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GICC_IIDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ID` reader - ID"]
pub type ID_R = crate::FieldReader<u32, ID_A>;
#[doc = "ID\n\nValue on reset: 33690683"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum ID_A {
    #[doc = "33690683: ID is valid"]
    VALID = 33690683,
}
impl From<ID_A> for u32 {
    #[inline(always)]
    fn from(variant: ID_A) -> Self {
        variant as _
    }
}
impl ID_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ID_A> {
        match self.bits {
            33690683 => Some(ID_A::VALID),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALID`"]
    #[inline(always)]
    pub fn is_valid(&self) -> bool {
        *self == ID_A::VALID
    }
}
#[doc = "Field `ID` writer - ID"]
pub type ID_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GICC_IIDR_SPEC, u32, ID_A, 32, O>;
impl<'a, const O: u8> ID_W<'a, O> {
    #[doc = "ID is valid"]
    #[inline(always)]
    pub fn valid(self) -> &'a mut W {
        self.variant(ID_A::VALID)
    }
}
impl R {
    #[doc = "Bits 0:31 - ID"]
    #[inline(always)]
    pub fn id(&self) -> ID_R {
        ID_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - ID"]
    #[inline(always)]
    #[must_use]
    pub fn id(&mut self) -> ID_W<0> {
        ID_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CPU Interface Identification Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicc_iidr](index.html) module"]
pub struct GICC_IIDR_SPEC;
impl crate::RegisterSpec for GICC_IIDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gicc_iidr::R](R) reader structure"]
impl crate::Readable for GICC_IIDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gicc_iidr::W](W) writer structure"]
impl crate::Writable for GICC_IIDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GICC_IIDR to value 0x0202_143b"]
impl crate::Resettable for GICC_IIDR_SPEC {
    const RESET_VALUE: Self::Ux = 0x0202_143b;
}
