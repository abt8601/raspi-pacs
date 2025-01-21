#[doc = "Register `ECR` writer"]
pub type W = crate::W<ECR_SPEC>;
#[doc = "Field `FE` writer - FE"]
pub type FE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PE` writer - PE"]
pub type PE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BE` writer - BE"]
pub type BE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OE` writer - OE"]
pub type OE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<ECR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - FE"]
    #[inline(always)]
    #[must_use]
    pub fn fe(&mut self) -> FE_W<ECR_SPEC> {
        FE_W::new(self, 0)
    }
    #[doc = "Bit 1 - PE"]
    #[inline(always)]
    #[must_use]
    pub fn pe(&mut self) -> PE_W<ECR_SPEC> {
        PE_W::new(self, 1)
    }
    #[doc = "Bit 2 - BE"]
    #[inline(always)]
    #[must_use]
    pub fn be(&mut self) -> BE_W<ECR_SPEC> {
        BE_W::new(self, 2)
    }
    #[doc = "Bit 3 - OE"]
    #[inline(always)]
    #[must_use]
    pub fn oe(&mut self) -> OE_W<ECR_SPEC> {
        OE_W::new(self, 3)
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
#[doc = "Error Clear Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ecr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ECR_SPEC;
impl crate::RegisterSpec for ECR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ecr::W`](W) writer structure"]
impl crate::Writable for ECR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ECR to value 0"]
impl crate::Resettable for ECR_SPEC {
    const RESET_VALUE: u32 = 0;
}
