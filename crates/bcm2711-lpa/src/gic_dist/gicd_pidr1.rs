#[doc = "Register `GICD_PIDR1` reader"]
pub type R = crate::R<GICD_PIDR1_SPEC>;
#[doc = "Field `GICD_PIDR1` reader - Peripheral ID 1"]
pub type GICD_PIDR1_R = crate::FieldReader<GICD_PIDR1_A>;
#[doc = "Peripheral ID 1\n\nValue on reset: 180"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum GICD_PIDR1_A {
    #[doc = "180: Valid"]
    VALID = 180,
}
impl From<GICD_PIDR1_A> for u32 {
    #[inline(always)]
    fn from(variant: GICD_PIDR1_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for GICD_PIDR1_A {
    type Ux = u32;
}
impl GICD_PIDR1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<GICD_PIDR1_A> {
        match self.bits {
            180 => Some(GICD_PIDR1_A::VALID),
            _ => None,
        }
    }
    #[doc = "Valid"]
    #[inline(always)]
    pub fn is_valid(&self) -> bool {
        *self == GICD_PIDR1_A::VALID
    }
}
impl R {
    #[doc = "Bits 0:31 - Peripheral ID 1"]
    #[inline(always)]
    pub fn gicd_pidr1(&self) -> GICD_PIDR1_R {
        GICD_PIDR1_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICD_PIDR1")
            .field("gicd_pidr1", &format_args!("{}", self.gicd_pidr1().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<GICD_PIDR1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Peripheral ID 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_pidr1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GICD_PIDR1_SPEC;
impl crate::RegisterSpec for GICD_PIDR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_pidr1::R`](R) reader structure"]
impl crate::Readable for GICD_PIDR1_SPEC {}
#[doc = "`reset()` method sets GICD_PIDR1 to value 0xb4"]
impl crate::Resettable for GICD_PIDR1_SPEC {
    const RESET_VALUE: Self::Ux = 0xb4;
}
