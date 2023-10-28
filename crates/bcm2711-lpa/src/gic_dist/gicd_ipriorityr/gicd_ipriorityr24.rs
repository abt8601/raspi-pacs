#[doc = "Register `GICD_IPRIORITYR24` reader"]
pub type R = crate::R<GICD_IPRIORITYR24_SPEC>;
#[doc = "Register `GICD_IPRIORITYR24` writer"]
pub type W = crate::W<GICD_IPRIORITYR24_SPEC>;
#[doc = "Field `TIMER_0` reader - Timer 0"]
pub type TIMER_0_R = crate::FieldReader;
#[doc = "Field `TIMER_0` writer - Timer 0"]
pub type TIMER_0_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `TIMER_1` reader - Timer 1"]
pub type TIMER_1_R = crate::FieldReader;
#[doc = "Field `TIMER_1` writer - Timer 1"]
pub type TIMER_1_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `TIMER_2` reader - Timer 2"]
pub type TIMER_2_R = crate::FieldReader;
#[doc = "Field `TIMER_2` writer - Timer 2"]
pub type TIMER_2_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `TIMER_3` reader - Timer 3"]
pub type TIMER_3_R = crate::FieldReader;
#[doc = "Field `TIMER_3` writer - Timer 3"]
pub type TIMER_3_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Timer 0"]
    #[inline(always)]
    pub fn timer_0(&self) -> TIMER_0_R {
        TIMER_0_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Timer 1"]
    #[inline(always)]
    pub fn timer_1(&self) -> TIMER_1_R {
        TIMER_1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Timer 2"]
    #[inline(always)]
    pub fn timer_2(&self) -> TIMER_2_R {
        TIMER_2_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Timer 3"]
    #[inline(always)]
    pub fn timer_3(&self) -> TIMER_3_R {
        TIMER_3_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICD_IPRIORITYR24")
            .field("timer_0", &format_args!("{}", self.timer_0().bits()))
            .field("timer_1", &format_args!("{}", self.timer_1().bits()))
            .field("timer_2", &format_args!("{}", self.timer_2().bits()))
            .field("timer_3", &format_args!("{}", self.timer_3().bits()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<GICD_IPRIORITYR24_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:7 - Timer 0"]
    #[inline(always)]
    #[must_use]
    pub fn timer_0(&mut self) -> TIMER_0_W<GICD_IPRIORITYR24_SPEC, 0> {
        TIMER_0_W::new(self)
    }
    #[doc = "Bits 8:15 - Timer 1"]
    #[inline(always)]
    #[must_use]
    pub fn timer_1(&mut self) -> TIMER_1_W<GICD_IPRIORITYR24_SPEC, 8> {
        TIMER_1_W::new(self)
    }
    #[doc = "Bits 16:23 - Timer 2"]
    #[inline(always)]
    #[must_use]
    pub fn timer_2(&mut self) -> TIMER_2_W<GICD_IPRIORITYR24_SPEC, 16> {
        TIMER_2_W::new(self)
    }
    #[doc = "Bits 24:31 - Timer 3"]
    #[inline(always)]
    #[must_use]
    pub fn timer_3(&mut self) -> TIMER_3_W<GICD_IPRIORITYR24_SPEC, 24> {
        TIMER_3_W::new(self)
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
#[doc = "Interrupt Priority 96 - 99 (Lower is first)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_ipriorityr24::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_ipriorityr24::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GICD_IPRIORITYR24_SPEC;
impl crate::RegisterSpec for GICD_IPRIORITYR24_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_ipriorityr24::R`](R) reader structure"]
impl crate::Readable for GICD_IPRIORITYR24_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gicd_ipriorityr24::W`](W) writer structure"]
impl crate::Writable for GICD_IPRIORITYR24_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GICD_IPRIORITYR24 to value 0"]
impl crate::Resettable for GICD_IPRIORITYR24_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
