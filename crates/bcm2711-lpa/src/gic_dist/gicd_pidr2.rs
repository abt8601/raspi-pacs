#[doc = "Register `GICD_PIDR2` reader"]
pub type R = crate::R<GICD_PIDR2_SPEC>;
#[doc = "Field `GICD_PIDR2` reader - Peripheral ID 2"]
pub type GICD_PIDR2_R = crate::FieldReader<GICD_PIDR2_A>;
#[doc = "Peripheral ID 2\n\nValue on reset: 43"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum GICD_PIDR2_A {
    #[doc = "43: Valid"]
    VALID = 43,
}
impl From<GICD_PIDR2_A> for u32 {
    #[inline(always)]
    fn from(variant: GICD_PIDR2_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for GICD_PIDR2_A {
    type Ux = u32;
}
impl GICD_PIDR2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<GICD_PIDR2_A> {
        match self.bits {
            43 => Some(GICD_PIDR2_A::VALID),
            _ => None,
        }
    }
    #[doc = "Valid"]
    #[inline(always)]
    pub fn is_valid(&self) -> bool {
        *self == GICD_PIDR2_A::VALID
    }
}
impl R {
    #[doc = "Bits 0:31 - Peripheral ID 2"]
    #[inline(always)]
    pub fn gicd_pidr2(&self) -> GICD_PIDR2_R {
        GICD_PIDR2_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICD_PIDR2")
            .field("gicd_pidr2", &format_args!("{}", self.gicd_pidr2().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<GICD_PIDR2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Peripheral ID 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_pidr2::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GICD_PIDR2_SPEC;
impl crate::RegisterSpec for GICD_PIDR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_pidr2::R`](R) reader structure"]
impl crate::Readable for GICD_PIDR2_SPEC {}
#[doc = "`reset()` method sets GICD_PIDR2 to value 0x2b"]
impl crate::Resettable for GICD_PIDR2_SPEC {
    const RESET_VALUE: Self::Ux = 0x2b;
}
