#[doc = "Register `CS` reader"]
pub type R = crate::R<CS_SPEC>;
#[doc = "Register `CS` writer"]
pub type W = crate::W<CS_SPEC>;
#[doc = "Field `M0` reader - System timer match 0"]
pub type M0_R = crate::BitReader;
#[doc = "Field `M0` writer - System timer match 0"]
pub type M0_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `M1` reader - System timer match 1"]
pub type M1_R = crate::BitReader;
#[doc = "Field `M1` writer - System timer match 1"]
pub type M1_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `M2` reader - System timer match 2"]
pub type M2_R = crate::BitReader;
#[doc = "Field `M2` writer - System timer match 2"]
pub type M2_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `M3` reader - System timer match 3"]
pub type M3_R = crate::BitReader;
#[doc = "Field `M3` writer - System timer match 3"]
pub type M3_W<'a, REG> = crate::BitWriter1C<'a, REG>;
impl R {
    #[doc = "Bit 0 - System timer match 0"]
    #[inline(always)]
    pub fn m0(&self) -> M0_R {
        M0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - System timer match 1"]
    #[inline(always)]
    pub fn m1(&self) -> M1_R {
        M1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - System timer match 2"]
    #[inline(always)]
    pub fn m2(&self) -> M2_R {
        M2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - System timer match 3"]
    #[inline(always)]
    pub fn m3(&self) -> M3_R {
        M3_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CS")
            .field("m3", &format_args!("{}", self.m3().bit()))
            .field("m2", &format_args!("{}", self.m2().bit()))
            .field("m1", &format_args!("{}", self.m1().bit()))
            .field("m0", &format_args!("{}", self.m0().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<CS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - System timer match 0"]
    #[inline(always)]
    #[must_use]
    pub fn m0(&mut self) -> M0_W<CS_SPEC> {
        M0_W::new(self, 0)
    }
    #[doc = "Bit 1 - System timer match 1"]
    #[inline(always)]
    #[must_use]
    pub fn m1(&mut self) -> M1_W<CS_SPEC> {
        M1_W::new(self, 1)
    }
    #[doc = "Bit 2 - System timer match 2"]
    #[inline(always)]
    #[must_use]
    pub fn m2(&mut self) -> M2_W<CS_SPEC> {
        M2_W::new(self, 2)
    }
    #[doc = "Bit 3 - System timer match 3"]
    #[inline(always)]
    #[must_use]
    pub fn m3(&mut self) -> M3_W<CS_SPEC> {
        M3_W::new(self, 3)
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
#[doc = "Control / Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cs::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cs::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CS_SPEC;
impl crate::RegisterSpec for CS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cs::R`](R) reader structure"]
impl crate::Readable for CS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cs::W`](W) writer structure"]
impl crate::Writable for CS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x0f;
}
#[doc = "`reset()` method sets CS to value 0"]
impl crate::Resettable for CS_SPEC {
    const RESET_VALUE: u32 = 0;
}
