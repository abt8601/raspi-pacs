#[doc = "Register `RSR` reader"]
pub type R = crate::R<RSR_SPEC>;
#[doc = "Field `FE` reader - FE"]
pub type FE_R = crate::BitReader;
#[doc = "Field `PE` reader - PE"]
pub type PE_R = crate::BitReader;
#[doc = "Field `BE` reader - BE"]
pub type BE_R = crate::BitReader;
#[doc = "Field `OE` reader - OE"]
pub type OE_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - FE"]
    #[inline(always)]
    pub fn fe(&self) -> FE_R {
        FE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PE"]
    #[inline(always)]
    pub fn pe(&self) -> PE_R {
        PE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - BE"]
    #[inline(always)]
    pub fn be(&self) -> BE_R {
        BE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - OE"]
    #[inline(always)]
    pub fn oe(&self) -> OE_R {
        OE_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RSR")
            .field("fe", &format_args!("{}", self.fe().bit()))
            .field("pe", &format_args!("{}", self.pe().bit()))
            .field("be", &format_args!("{}", self.be().bit()))
            .field("oe", &format_args!("{}", self.oe().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<RSR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Receive Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rsr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RSR_SPEC;
impl crate::RegisterSpec for RSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rsr::R`](R) reader structure"]
impl crate::Readable for RSR_SPEC {}
#[doc = "`reset()` method sets RSR to value 0"]
impl crate::Resettable for RSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
