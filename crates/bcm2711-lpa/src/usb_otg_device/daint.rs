#[doc = "Register `DAINT` reader"]
pub type R = crate::R<DAINT_SPEC>;
#[doc = "Field `IEPINT` reader - IN endpoint interrupt bits"]
pub type IEPINT_R = crate::FieldReader<u16>;
#[doc = "Field `OEPINT` reader - OUT endpoint interrupt bits"]
pub type OEPINT_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - IN endpoint interrupt bits"]
    #[inline(always)]
    pub fn iepint(&self) -> IEPINT_R {
        IEPINT_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - OUT endpoint interrupt bits"]
    #[inline(always)]
    pub fn oepint(&self) -> OEPINT_R {
        OEPINT_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DAINT")
            .field("iepint", &format_args!("{}", self.iepint().bits()))
            .field("oepint", &format_args!("{}", self.oepint().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<DAINT_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "OTG_HS device all endpoints interrupt register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`daint::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DAINT_SPEC;
impl crate::RegisterSpec for DAINT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`daint::R`](R) reader structure"]
impl crate::Readable for DAINT_SPEC {}
#[doc = "`reset()` method sets DAINT to value 0"]
impl crate::Resettable for DAINT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
