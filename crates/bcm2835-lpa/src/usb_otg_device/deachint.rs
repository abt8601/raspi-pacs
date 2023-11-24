#[doc = "Register `DEACHINT` reader"]
pub type R = crate::R<DEACHINT_SPEC>;
#[doc = "Register `DEACHINT` writer"]
pub type W = crate::W<DEACHINT_SPEC>;
#[doc = "Field `IEP1INT` reader - IN endpoint 1interrupt bit"]
pub type IEP1INT_R = crate::BitReader;
#[doc = "Field `IEP1INT` writer - IN endpoint 1interrupt bit"]
pub type IEP1INT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OEP1INT` reader - OUT endpoint 1 interrupt bit"]
pub type OEP1INT_R = crate::BitReader;
#[doc = "Field `OEP1INT` writer - OUT endpoint 1 interrupt bit"]
pub type OEP1INT_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - IN endpoint 1interrupt bit"]
    #[inline(always)]
    pub fn iep1int(&self) -> IEP1INT_R {
        IEP1INT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 17 - OUT endpoint 1 interrupt bit"]
    #[inline(always)]
    pub fn oep1int(&self) -> OEP1INT_R {
        OEP1INT_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DEACHINT")
            .field("iep1int", &format_args!("{}", self.iep1int().bit()))
            .field("oep1int", &format_args!("{}", self.oep1int().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<DEACHINT_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 1 - IN endpoint 1interrupt bit"]
    #[inline(always)]
    #[must_use]
    pub fn iep1int(&mut self) -> IEP1INT_W<DEACHINT_SPEC> {
        IEP1INT_W::new(self, 1)
    }
    #[doc = "Bit 17 - OUT endpoint 1 interrupt bit"]
    #[inline(always)]
    #[must_use]
    pub fn oep1int(&mut self) -> OEP1INT_W<DEACHINT_SPEC> {
        OEP1INT_W::new(self, 17)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "OTG_HS device each endpoint interrupt register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`deachint::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`deachint::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DEACHINT_SPEC;
impl crate::RegisterSpec for DEACHINT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`deachint::R`](R) reader structure"]
impl crate::Readable for DEACHINT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`deachint::W`](W) writer structure"]
impl crate::Writable for DEACHINT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DEACHINT to value 0"]
impl crate::Resettable for DEACHINT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
