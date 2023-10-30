#[doc = "Register `READ` reader"]
pub type R = crate::R<READ_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<READ_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Read messages from the VideoCore\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`read::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct READ_SPEC;
impl crate::RegisterSpec for READ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`read::R`](R) reader structure"]
impl crate::Readable for READ_SPEC {}
