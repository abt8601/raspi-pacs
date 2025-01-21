#[doc = "Register `GICD_CIDR3` reader"]
pub type R = crate::R<GICD_CIDR3_SPEC>;
#[doc = "Field `GICD_CIDR3` reader - Component ID 3"]
pub type GICD_CIDR3_R = crate::FieldReader<GICD_CIDR3_A>;
#[doc = "Component ID 3\n\nValue on reset: 177"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum GICD_CIDR3_A {
    #[doc = "177: Valid"]
    VALID = 177,
    #[doc = "0: Invalid ID"]
    INVALID = 0,
}
impl From<GICD_CIDR3_A> for u32 {
    #[inline(always)]
    fn from(variant: GICD_CIDR3_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for GICD_CIDR3_A {
    type Ux = u32;
}
impl GICD_CIDR3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> GICD_CIDR3_A {
        match self.bits {
            177 => GICD_CIDR3_A::VALID,
            _ => GICD_CIDR3_A::INVALID,
        }
    }
    #[doc = "Valid"]
    #[inline(always)]
    pub fn is_valid(&self) -> bool {
        *self == GICD_CIDR3_A::VALID
    }
    #[doc = "Invalid ID"]
    #[inline(always)]
    pub fn is_invalid(&self) -> bool {
        matches!(self.variant(), GICD_CIDR3_A::INVALID)
    }
}
impl R {
    #[doc = "Bits 0:31 - Component ID 3"]
    #[inline(always)]
    pub fn gicd_cidr3(&self) -> GICD_CIDR3_R {
        GICD_CIDR3_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICD_CIDR3")
            .field("gicd_cidr3", &format_args!("{}", self.gicd_cidr3().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<GICD_CIDR3_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "Component ID 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_cidr3::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GICD_CIDR3_SPEC;
impl crate::RegisterSpec for GICD_CIDR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_cidr3::R`](R) reader structure"]
impl crate::Readable for GICD_CIDR3_SPEC {}
#[doc = "`reset()` method sets GICD_CIDR3 to value 0xb1"]
impl crate::Resettable for GICD_CIDR3_SPEC {
    const RESET_VALUE: u32 = 0xb1;
}
