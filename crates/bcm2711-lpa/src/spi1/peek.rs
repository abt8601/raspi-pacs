#[doc = "Register `PEEK` reader"]
pub struct R(crate::R<PEEK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PEEK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PEEK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PEEK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DATA` reader - FIFO data access"]
pub type DATA_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - FIFO data access"]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Read the RXFIFO without removing an entry\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [peek](index.html) module"]
pub struct PEEK_SPEC;
impl crate::RegisterSpec for PEEK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [peek::R](R) reader structure"]
impl crate::Readable for PEEK_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PEEK to value 0"]
impl crate::Resettable for PEEK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
