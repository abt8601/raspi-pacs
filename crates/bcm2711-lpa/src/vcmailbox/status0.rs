#[doc = "Register `STATUS0` reader"]
pub struct R(crate::R<STATUS0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATUS0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STATUS0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STATUS0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `EMPTY` reader - "]
pub type EMPTY_R = crate::BitReader<bool>;
#[doc = "Field `FULL` reader - "]
pub type FULL_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn empty(&self) -> EMPTY_R {
        EMPTY_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn full(&self) -> FULL_R {
        FULL_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status0](index.html) module"]
pub struct STATUS0_SPEC;
impl crate::RegisterSpec for STATUS0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [status0::R](R) reader structure"]
impl crate::Readable for STATUS0_SPEC {
    type Reader = R;
}
