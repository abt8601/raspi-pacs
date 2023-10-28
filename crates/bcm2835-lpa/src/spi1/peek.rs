#[doc = "Register `PEEK` reader"]
pub type R = crate::R<PEEK_SPEC>;
#[doc = "Field `DATA` reader - FIFO data access"]
pub type DATA_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - FIFO data access"]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PEEK")
            .field("data", &format_args!("{}", self.data().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<PEEK_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Read the RXFIFO without removing an entry\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`peek::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PEEK_SPEC;
impl crate::RegisterSpec for PEEK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`peek::R`](R) reader structure"]
impl crate::Readable for PEEK_SPEC {}
#[doc = "`reset()` method sets PEEK to value 0"]
impl crate::Resettable for PEEK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
