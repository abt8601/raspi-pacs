#[doc = "Register `DEL` reader"]
pub type R = crate::R<DEL_SPEC>;
#[doc = "Register `DEL` writer"]
pub type W = crate::W<DEL_SPEC>;
#[doc = "Field `REDL` reader - Delay before reading after a rising edge"]
pub type REDL_R = crate::FieldReader<u16>;
#[doc = "Field `REDL` writer - Delay before reading after a rising edge"]
pub type REDL_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `FEDL` reader - Delay before reading after a falling edge"]
pub type FEDL_R = crate::FieldReader<u16>;
#[doc = "Field `FEDL` writer - Delay before reading after a falling edge"]
pub type FEDL_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Delay before reading after a rising edge"]
    #[inline(always)]
    pub fn redl(&self) -> REDL_R {
        REDL_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Delay before reading after a falling edge"]
    #[inline(always)]
    pub fn fedl(&self) -> FEDL_R {
        FEDL_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DEL")
            .field("fedl", &format_args!("{}", self.fedl().bits()))
            .field("redl", &format_args!("{}", self.redl().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<DEL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:15 - Delay before reading after a rising edge"]
    #[inline(always)]
    #[must_use]
    pub fn redl(&mut self) -> REDL_W<DEL_SPEC> {
        REDL_W::new(self, 0)
    }
    #[doc = "Bits 16:31 - Delay before reading after a falling edge"]
    #[inline(always)]
    #[must_use]
    pub fn fedl(&mut self) -> FEDL_W<DEL_SPEC> {
        FEDL_W::new(self, 16)
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
#[doc = "Data delay (Values must be under CDIV / 2)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`del::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`del::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DEL_SPEC;
impl crate::RegisterSpec for DEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`del::R`](R) reader structure"]
impl crate::Readable for DEL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`del::W`](W) writer structure"]
impl crate::Writable for DEL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DEL to value 0x0030_0030"]
impl crate::Resettable for DEL_SPEC {
    const RESET_VALUE: Self::Ux = 0x0030_0030;
}
