#[doc = "Register `GPCLR1` writer"]
pub type W = crate::W<GPCLR1_SPEC>;
#[doc = "Field `CLR32` writer - Clear 32"]
pub type CLR32_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `CLR33` writer - Clear 33"]
pub type CLR33_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `CLR34` writer - Clear 34"]
pub type CLR34_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `CLR35` writer - Clear 35"]
pub type CLR35_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `CLR36` writer - Clear 36"]
pub type CLR36_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `CLR37` writer - Clear 37"]
pub type CLR37_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `CLR38` writer - Clear 38"]
pub type CLR38_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `CLR39` writer - Clear 39"]
pub type CLR39_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `CLR40` writer - Clear 40"]
pub type CLR40_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `CLR41` writer - Clear 41"]
pub type CLR41_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `CLR42` writer - Clear 42"]
pub type CLR42_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `CLR43` writer - Clear 43"]
pub type CLR43_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `CLR44` writer - Clear 44"]
pub type CLR44_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `CLR45` writer - Clear 45"]
pub type CLR45_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `CLR46` writer - Clear 46"]
pub type CLR46_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `CLR47` writer - Clear 47"]
pub type CLR47_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `CLR48` writer - Clear 48"]
pub type CLR48_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `CLR49` writer - Clear 49"]
pub type CLR49_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `CLR50` writer - Clear 50"]
pub type CLR50_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `CLR51` writer - Clear 51"]
pub type CLR51_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `CLR52` writer - Clear 52"]
pub type CLR52_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `CLR53` writer - Clear 53"]
pub type CLR53_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `CLR54` writer - Clear 54"]
pub type CLR54_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `CLR55` writer - Clear 55"]
pub type CLR55_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `CLR56` writer - Clear 56"]
pub type CLR56_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `CLR57` writer - Clear 57"]
pub type CLR57_W<'a, REG> = crate::BitWriter1C<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<GPCLR1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Clear 32"]
    #[inline(always)]
    #[must_use]
    pub fn clr32(&mut self) -> CLR32_W<GPCLR1_SPEC> {
        CLR32_W::new(self, 0)
    }
    #[doc = "Bit 1 - Clear 33"]
    #[inline(always)]
    #[must_use]
    pub fn clr33(&mut self) -> CLR33_W<GPCLR1_SPEC> {
        CLR33_W::new(self, 1)
    }
    #[doc = "Bit 2 - Clear 34"]
    #[inline(always)]
    #[must_use]
    pub fn clr34(&mut self) -> CLR34_W<GPCLR1_SPEC> {
        CLR34_W::new(self, 2)
    }
    #[doc = "Bit 3 - Clear 35"]
    #[inline(always)]
    #[must_use]
    pub fn clr35(&mut self) -> CLR35_W<GPCLR1_SPEC> {
        CLR35_W::new(self, 3)
    }
    #[doc = "Bit 4 - Clear 36"]
    #[inline(always)]
    #[must_use]
    pub fn clr36(&mut self) -> CLR36_W<GPCLR1_SPEC> {
        CLR36_W::new(self, 4)
    }
    #[doc = "Bit 5 - Clear 37"]
    #[inline(always)]
    #[must_use]
    pub fn clr37(&mut self) -> CLR37_W<GPCLR1_SPEC> {
        CLR37_W::new(self, 5)
    }
    #[doc = "Bit 6 - Clear 38"]
    #[inline(always)]
    #[must_use]
    pub fn clr38(&mut self) -> CLR38_W<GPCLR1_SPEC> {
        CLR38_W::new(self, 6)
    }
    #[doc = "Bit 7 - Clear 39"]
    #[inline(always)]
    #[must_use]
    pub fn clr39(&mut self) -> CLR39_W<GPCLR1_SPEC> {
        CLR39_W::new(self, 7)
    }
    #[doc = "Bit 8 - Clear 40"]
    #[inline(always)]
    #[must_use]
    pub fn clr40(&mut self) -> CLR40_W<GPCLR1_SPEC> {
        CLR40_W::new(self, 8)
    }
    #[doc = "Bit 9 - Clear 41"]
    #[inline(always)]
    #[must_use]
    pub fn clr41(&mut self) -> CLR41_W<GPCLR1_SPEC> {
        CLR41_W::new(self, 9)
    }
    #[doc = "Bit 10 - Clear 42"]
    #[inline(always)]
    #[must_use]
    pub fn clr42(&mut self) -> CLR42_W<GPCLR1_SPEC> {
        CLR42_W::new(self, 10)
    }
    #[doc = "Bit 11 - Clear 43"]
    #[inline(always)]
    #[must_use]
    pub fn clr43(&mut self) -> CLR43_W<GPCLR1_SPEC> {
        CLR43_W::new(self, 11)
    }
    #[doc = "Bit 12 - Clear 44"]
    #[inline(always)]
    #[must_use]
    pub fn clr44(&mut self) -> CLR44_W<GPCLR1_SPEC> {
        CLR44_W::new(self, 12)
    }
    #[doc = "Bit 13 - Clear 45"]
    #[inline(always)]
    #[must_use]
    pub fn clr45(&mut self) -> CLR45_W<GPCLR1_SPEC> {
        CLR45_W::new(self, 13)
    }
    #[doc = "Bit 14 - Clear 46"]
    #[inline(always)]
    #[must_use]
    pub fn clr46(&mut self) -> CLR46_W<GPCLR1_SPEC> {
        CLR46_W::new(self, 14)
    }
    #[doc = "Bit 15 - Clear 47"]
    #[inline(always)]
    #[must_use]
    pub fn clr47(&mut self) -> CLR47_W<GPCLR1_SPEC> {
        CLR47_W::new(self, 15)
    }
    #[doc = "Bit 16 - Clear 48"]
    #[inline(always)]
    #[must_use]
    pub fn clr48(&mut self) -> CLR48_W<GPCLR1_SPEC> {
        CLR48_W::new(self, 16)
    }
    #[doc = "Bit 17 - Clear 49"]
    #[inline(always)]
    #[must_use]
    pub fn clr49(&mut self) -> CLR49_W<GPCLR1_SPEC> {
        CLR49_W::new(self, 17)
    }
    #[doc = "Bit 18 - Clear 50"]
    #[inline(always)]
    #[must_use]
    pub fn clr50(&mut self) -> CLR50_W<GPCLR1_SPEC> {
        CLR50_W::new(self, 18)
    }
    #[doc = "Bit 19 - Clear 51"]
    #[inline(always)]
    #[must_use]
    pub fn clr51(&mut self) -> CLR51_W<GPCLR1_SPEC> {
        CLR51_W::new(self, 19)
    }
    #[doc = "Bit 20 - Clear 52"]
    #[inline(always)]
    #[must_use]
    pub fn clr52(&mut self) -> CLR52_W<GPCLR1_SPEC> {
        CLR52_W::new(self, 20)
    }
    #[doc = "Bit 21 - Clear 53"]
    #[inline(always)]
    #[must_use]
    pub fn clr53(&mut self) -> CLR53_W<GPCLR1_SPEC> {
        CLR53_W::new(self, 21)
    }
    #[doc = "Bit 22 - Clear 54"]
    #[inline(always)]
    #[must_use]
    pub fn clr54(&mut self) -> CLR54_W<GPCLR1_SPEC> {
        CLR54_W::new(self, 22)
    }
    #[doc = "Bit 23 - Clear 55"]
    #[inline(always)]
    #[must_use]
    pub fn clr55(&mut self) -> CLR55_W<GPCLR1_SPEC> {
        CLR55_W::new(self, 23)
    }
    #[doc = "Bit 24 - Clear 56"]
    #[inline(always)]
    #[must_use]
    pub fn clr56(&mut self) -> CLR56_W<GPCLR1_SPEC> {
        CLR56_W::new(self, 24)
    }
    #[doc = "Bit 25 - Clear 57"]
    #[inline(always)]
    #[must_use]
    pub fn clr57(&mut self) -> CLR57_W<GPCLR1_SPEC> {
        CLR57_W::new(self, 25)
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
#[doc = "GPIO Pin Output Clear 1\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpclr1::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPCLR1_SPEC;
impl crate::RegisterSpec for GPCLR1_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`gpclr1::W`](W) writer structure"]
impl crate::Writable for GPCLR1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x03ff_ffff;
}
