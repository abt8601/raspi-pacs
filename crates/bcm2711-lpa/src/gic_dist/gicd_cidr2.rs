#[doc = "Register `GICD_CIDR2` reader"]
pub type R = crate::R<GICD_CIDR2_SPEC>;
#[doc = "Field `GICD_CIDR2` reader - Component ID 2"]
pub type GICD_CIDR2_R = crate::FieldReader<GICD_CIDR2_A>;
#[doc = "Component ID 2\n\nValue on reset: 5"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum GICD_CIDR2_A {
    #[doc = "5: Valid"]
    VALID = 5,
}
impl From<GICD_CIDR2_A> for u32 {
    #[inline(always)]
    fn from(variant: GICD_CIDR2_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for GICD_CIDR2_A {
    type Ux = u32;
}
impl GICD_CIDR2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<GICD_CIDR2_A> {
        match self.bits {
            5 => Some(GICD_CIDR2_A::VALID),
            _ => None,
        }
    }
    #[doc = "Valid"]
    #[inline(always)]
    pub fn is_valid(&self) -> bool {
        *self == GICD_CIDR2_A::VALID
    }
}
impl R {
    #[doc = "Bits 0:31 - Component ID 2"]
    #[inline(always)]
    pub fn gicd_cidr2(&self) -> GICD_CIDR2_R {
        GICD_CIDR2_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICD_CIDR2")
            .field("gicd_cidr2", &format_args!("{}", self.gicd_cidr2().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<GICD_CIDR2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Component ID 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_cidr2::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GICD_CIDR2_SPEC;
impl crate::RegisterSpec for GICD_CIDR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_cidr2::R`](R) reader structure"]
impl crate::Readable for GICD_CIDR2_SPEC {}
#[doc = "`reset()` method sets GICD_CIDR2 to value 0x05"]
impl crate::Resettable for GICD_CIDR2_SPEC {
    const RESET_VALUE: Self::Ux = 0x05;
}
