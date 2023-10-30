#[doc = "Register `STATUS0` reader"]
pub type R = crate::R<STATUS0_SPEC>;
#[doc = "Field `EMPTY` reader - "]
pub type EMPTY_R = crate::BitReader;
#[doc = "Field `FULL` reader - "]
pub type FULL_R = crate::BitReader;
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
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STATUS0")
            .field("full", &format_args!("{}", self.full().bit()))
            .field("empty", &format_args!("{}", self.empty().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<STATUS0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STATUS0_SPEC;
impl crate::RegisterSpec for STATUS0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status0::R`](R) reader structure"]
impl crate::Readable for STATUS0_SPEC {}
