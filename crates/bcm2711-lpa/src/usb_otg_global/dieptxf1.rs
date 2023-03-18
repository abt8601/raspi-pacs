#[doc = "Register `DIEPTXF1` reader"]
pub struct R(crate::R<DIEPTXF1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DIEPTXF1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DIEPTXF1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DIEPTXF1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DIEPTXF1` writer"]
pub struct W(crate::W<DIEPTXF1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DIEPTXF1_SPEC>;
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
impl From<crate::W<DIEPTXF1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DIEPTXF1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INEPTXSA` reader - IN endpoint FIFOx transmit RAM start address"]
pub type INEPTXSA_R = crate::FieldReader<u16, u16>;
#[doc = "Field `INEPTXSA` writer - IN endpoint FIFOx transmit RAM start address"]
pub type INEPTXSA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DIEPTXF1_SPEC, u16, u16, 16, O>;
#[doc = "Field `INEPTXFD` reader - IN endpoint TxFIFO depth"]
pub type INEPTXFD_R = crate::FieldReader<u16, u16>;
#[doc = "Field `INEPTXFD` writer - IN endpoint TxFIFO depth"]
pub type INEPTXFD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DIEPTXF1_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - IN endpoint FIFOx transmit RAM start address"]
    #[inline(always)]
    pub fn ineptxsa(&self) -> INEPTXSA_R {
        INEPTXSA_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - IN endpoint TxFIFO depth"]
    #[inline(always)]
    pub fn ineptxfd(&self) -> INEPTXFD_R {
        INEPTXFD_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - IN endpoint FIFOx transmit RAM start address"]
    #[inline(always)]
    #[must_use]
    pub fn ineptxsa(&mut self) -> INEPTXSA_W<0> {
        INEPTXSA_W::new(self)
    }
    #[doc = "Bits 16:31 - IN endpoint TxFIFO depth"]
    #[inline(always)]
    #[must_use]
    pub fn ineptxfd(&mut self) -> INEPTXFD_W<16> {
        INEPTXFD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OTG_HS device IN endpoint transmit FIFO size register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dieptxf1](index.html) module"]
pub struct DIEPTXF1_SPEC;
impl crate::RegisterSpec for DIEPTXF1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dieptxf1::R](R) reader structure"]
impl crate::Readable for DIEPTXF1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dieptxf1::W](W) writer structure"]
impl crate::Writable for DIEPTXF1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DIEPTXF1 to value 0x0200_0400"]
impl crate::Resettable for DIEPTXF1_SPEC {
    const RESET_VALUE: Self::Ux = 0x0200_0400;
}
