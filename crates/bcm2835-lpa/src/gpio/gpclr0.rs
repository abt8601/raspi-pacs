#[doc = "Register `GPCLR0` writer"]
pub type W = crate::W<GPCLR0_SPEC>;
#[doc = "Field `CLR0` writer - Clear 0"]
pub type CLR0_W<'a, REG, const O: u8> = crate::BitWriter1C<'a, REG, O>;
#[doc = "Field `CLR1` writer - Clear 1"]
pub type CLR1_W<'a, REG, const O: u8> = crate::BitWriter1C<'a, REG, O>;
#[doc = "Field `CLR2` writer - Clear 2"]
pub type CLR2_W<'a, REG, const O: u8> = crate::BitWriter1C<'a, REG, O>;
#[doc = "Field `CLR3` writer - Clear 3"]
pub type CLR3_W<'a, REG, const O: u8> = crate::BitWriter1C<'a, REG, O>;
#[doc = "Field `CLR4` writer - Clear 4"]
pub type CLR4_W<'a, REG, const O: u8> = crate::BitWriter1C<'a, REG, O>;
#[doc = "Field `CLR5` writer - Clear 5"]
pub type CLR5_W<'a, REG, const O: u8> = crate::BitWriter1C<'a, REG, O>;
#[doc = "Field `CLR6` writer - Clear 6"]
pub type CLR6_W<'a, REG, const O: u8> = crate::BitWriter1C<'a, REG, O>;
#[doc = "Field `CLR7` writer - Clear 7"]
pub type CLR7_W<'a, REG, const O: u8> = crate::BitWriter1C<'a, REG, O>;
#[doc = "Field `CLR8` writer - Clear 8"]
pub type CLR8_W<'a, REG, const O: u8> = crate::BitWriter1C<'a, REG, O>;
#[doc = "Field `CLR9` writer - Clear 9"]
pub type CLR9_W<'a, REG, const O: u8> = crate::BitWriter1C<'a, REG, O>;
#[doc = "Field `CLR10` writer - Clear 10"]
pub type CLR10_W<'a, REG, const O: u8> = crate::BitWriter1C<'a, REG, O>;
#[doc = "Field `CLR11` writer - Clear 11"]
pub type CLR11_W<'a, REG, const O: u8> = crate::BitWriter1C<'a, REG, O>;
#[doc = "Field `CLR12` writer - Clear 12"]
pub type CLR12_W<'a, REG, const O: u8> = crate::BitWriter1C<'a, REG, O>;
#[doc = "Field `CLR13` writer - Clear 13"]
pub type CLR13_W<'a, REG, const O: u8> = crate::BitWriter1C<'a, REG, O>;
#[doc = "Field `CLR14` writer - Clear 14"]
pub type CLR14_W<'a, REG, const O: u8> = crate::BitWriter1C<'a, REG, O>;
#[doc = "Field `CLR15` writer - Clear 15"]
pub type CLR15_W<'a, REG, const O: u8> = crate::BitWriter1C<'a, REG, O>;
#[doc = "Field `CLR16` writer - Clear 16"]
pub type CLR16_W<'a, REG, const O: u8> = crate::BitWriter1C<'a, REG, O>;
#[doc = "Field `CLR17` writer - Clear 17"]
pub type CLR17_W<'a, REG, const O: u8> = crate::BitWriter1C<'a, REG, O>;
#[doc = "Field `CLR18` writer - Clear 18"]
pub type CLR18_W<'a, REG, const O: u8> = crate::BitWriter1C<'a, REG, O>;
#[doc = "Field `CLR19` writer - Clear 19"]
pub type CLR19_W<'a, REG, const O: u8> = crate::BitWriter1C<'a, REG, O>;
#[doc = "Field `CLR20` writer - Clear 20"]
pub type CLR20_W<'a, REG, const O: u8> = crate::BitWriter1C<'a, REG, O>;
#[doc = "Field `CLR21` writer - Clear 21"]
pub type CLR21_W<'a, REG, const O: u8> = crate::BitWriter1C<'a, REG, O>;
#[doc = "Field `CLR22` writer - Clear 22"]
pub type CLR22_W<'a, REG, const O: u8> = crate::BitWriter1C<'a, REG, O>;
#[doc = "Field `CLR23` writer - Clear 23"]
pub type CLR23_W<'a, REG, const O: u8> = crate::BitWriter1C<'a, REG, O>;
#[doc = "Field `CLR24` writer - Clear 24"]
pub type CLR24_W<'a, REG, const O: u8> = crate::BitWriter1C<'a, REG, O>;
#[doc = "Field `CLR25` writer - Clear 25"]
pub type CLR25_W<'a, REG, const O: u8> = crate::BitWriter1C<'a, REG, O>;
#[doc = "Field `CLR26` writer - Clear 26"]
pub type CLR26_W<'a, REG, const O: u8> = crate::BitWriter1C<'a, REG, O>;
#[doc = "Field `CLR27` writer - Clear 27"]
pub type CLR27_W<'a, REG, const O: u8> = crate::BitWriter1C<'a, REG, O>;
#[doc = "Field `CLR28` writer - Clear 28"]
pub type CLR28_W<'a, REG, const O: u8> = crate::BitWriter1C<'a, REG, O>;
#[doc = "Field `CLR29` writer - Clear 29"]
pub type CLR29_W<'a, REG, const O: u8> = crate::BitWriter1C<'a, REG, O>;
#[doc = "Field `CLR30` writer - Clear 30"]
pub type CLR30_W<'a, REG, const O: u8> = crate::BitWriter1C<'a, REG, O>;
#[doc = "Field `CLR31` writer - Clear 31"]
pub type CLR31_W<'a, REG, const O: u8> = crate::BitWriter1C<'a, REG, O>;
impl core::fmt::Debug for crate::generic::Reg<GPCLR0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Clear 0"]
    #[inline(always)]
    #[must_use]
    pub fn clr0(&mut self) -> CLR0_W<GPCLR0_SPEC, 0> {
        CLR0_W::new(self)
    }
    #[doc = "Bit 1 - Clear 1"]
    #[inline(always)]
    #[must_use]
    pub fn clr1(&mut self) -> CLR1_W<GPCLR0_SPEC, 1> {
        CLR1_W::new(self)
    }
    #[doc = "Bit 2 - Clear 2"]
    #[inline(always)]
    #[must_use]
    pub fn clr2(&mut self) -> CLR2_W<GPCLR0_SPEC, 2> {
        CLR2_W::new(self)
    }
    #[doc = "Bit 3 - Clear 3"]
    #[inline(always)]
    #[must_use]
    pub fn clr3(&mut self) -> CLR3_W<GPCLR0_SPEC, 3> {
        CLR3_W::new(self)
    }
    #[doc = "Bit 4 - Clear 4"]
    #[inline(always)]
    #[must_use]
    pub fn clr4(&mut self) -> CLR4_W<GPCLR0_SPEC, 4> {
        CLR4_W::new(self)
    }
    #[doc = "Bit 5 - Clear 5"]
    #[inline(always)]
    #[must_use]
    pub fn clr5(&mut self) -> CLR5_W<GPCLR0_SPEC, 5> {
        CLR5_W::new(self)
    }
    #[doc = "Bit 6 - Clear 6"]
    #[inline(always)]
    #[must_use]
    pub fn clr6(&mut self) -> CLR6_W<GPCLR0_SPEC, 6> {
        CLR6_W::new(self)
    }
    #[doc = "Bit 7 - Clear 7"]
    #[inline(always)]
    #[must_use]
    pub fn clr7(&mut self) -> CLR7_W<GPCLR0_SPEC, 7> {
        CLR7_W::new(self)
    }
    #[doc = "Bit 8 - Clear 8"]
    #[inline(always)]
    #[must_use]
    pub fn clr8(&mut self) -> CLR8_W<GPCLR0_SPEC, 8> {
        CLR8_W::new(self)
    }
    #[doc = "Bit 9 - Clear 9"]
    #[inline(always)]
    #[must_use]
    pub fn clr9(&mut self) -> CLR9_W<GPCLR0_SPEC, 9> {
        CLR9_W::new(self)
    }
    #[doc = "Bit 10 - Clear 10"]
    #[inline(always)]
    #[must_use]
    pub fn clr10(&mut self) -> CLR10_W<GPCLR0_SPEC, 10> {
        CLR10_W::new(self)
    }
    #[doc = "Bit 11 - Clear 11"]
    #[inline(always)]
    #[must_use]
    pub fn clr11(&mut self) -> CLR11_W<GPCLR0_SPEC, 11> {
        CLR11_W::new(self)
    }
    #[doc = "Bit 12 - Clear 12"]
    #[inline(always)]
    #[must_use]
    pub fn clr12(&mut self) -> CLR12_W<GPCLR0_SPEC, 12> {
        CLR12_W::new(self)
    }
    #[doc = "Bit 13 - Clear 13"]
    #[inline(always)]
    #[must_use]
    pub fn clr13(&mut self) -> CLR13_W<GPCLR0_SPEC, 13> {
        CLR13_W::new(self)
    }
    #[doc = "Bit 14 - Clear 14"]
    #[inline(always)]
    #[must_use]
    pub fn clr14(&mut self) -> CLR14_W<GPCLR0_SPEC, 14> {
        CLR14_W::new(self)
    }
    #[doc = "Bit 15 - Clear 15"]
    #[inline(always)]
    #[must_use]
    pub fn clr15(&mut self) -> CLR15_W<GPCLR0_SPEC, 15> {
        CLR15_W::new(self)
    }
    #[doc = "Bit 16 - Clear 16"]
    #[inline(always)]
    #[must_use]
    pub fn clr16(&mut self) -> CLR16_W<GPCLR0_SPEC, 16> {
        CLR16_W::new(self)
    }
    #[doc = "Bit 17 - Clear 17"]
    #[inline(always)]
    #[must_use]
    pub fn clr17(&mut self) -> CLR17_W<GPCLR0_SPEC, 17> {
        CLR17_W::new(self)
    }
    #[doc = "Bit 18 - Clear 18"]
    #[inline(always)]
    #[must_use]
    pub fn clr18(&mut self) -> CLR18_W<GPCLR0_SPEC, 18> {
        CLR18_W::new(self)
    }
    #[doc = "Bit 19 - Clear 19"]
    #[inline(always)]
    #[must_use]
    pub fn clr19(&mut self) -> CLR19_W<GPCLR0_SPEC, 19> {
        CLR19_W::new(self)
    }
    #[doc = "Bit 20 - Clear 20"]
    #[inline(always)]
    #[must_use]
    pub fn clr20(&mut self) -> CLR20_W<GPCLR0_SPEC, 20> {
        CLR20_W::new(self)
    }
    #[doc = "Bit 21 - Clear 21"]
    #[inline(always)]
    #[must_use]
    pub fn clr21(&mut self) -> CLR21_W<GPCLR0_SPEC, 21> {
        CLR21_W::new(self)
    }
    #[doc = "Bit 22 - Clear 22"]
    #[inline(always)]
    #[must_use]
    pub fn clr22(&mut self) -> CLR22_W<GPCLR0_SPEC, 22> {
        CLR22_W::new(self)
    }
    #[doc = "Bit 23 - Clear 23"]
    #[inline(always)]
    #[must_use]
    pub fn clr23(&mut self) -> CLR23_W<GPCLR0_SPEC, 23> {
        CLR23_W::new(self)
    }
    #[doc = "Bit 24 - Clear 24"]
    #[inline(always)]
    #[must_use]
    pub fn clr24(&mut self) -> CLR24_W<GPCLR0_SPEC, 24> {
        CLR24_W::new(self)
    }
    #[doc = "Bit 25 - Clear 25"]
    #[inline(always)]
    #[must_use]
    pub fn clr25(&mut self) -> CLR25_W<GPCLR0_SPEC, 25> {
        CLR25_W::new(self)
    }
    #[doc = "Bit 26 - Clear 26"]
    #[inline(always)]
    #[must_use]
    pub fn clr26(&mut self) -> CLR26_W<GPCLR0_SPEC, 26> {
        CLR26_W::new(self)
    }
    #[doc = "Bit 27 - Clear 27"]
    #[inline(always)]
    #[must_use]
    pub fn clr27(&mut self) -> CLR27_W<GPCLR0_SPEC, 27> {
        CLR27_W::new(self)
    }
    #[doc = "Bit 28 - Clear 28"]
    #[inline(always)]
    #[must_use]
    pub fn clr28(&mut self) -> CLR28_W<GPCLR0_SPEC, 28> {
        CLR28_W::new(self)
    }
    #[doc = "Bit 29 - Clear 29"]
    #[inline(always)]
    #[must_use]
    pub fn clr29(&mut self) -> CLR29_W<GPCLR0_SPEC, 29> {
        CLR29_W::new(self)
    }
    #[doc = "Bit 30 - Clear 30"]
    #[inline(always)]
    #[must_use]
    pub fn clr30(&mut self) -> CLR30_W<GPCLR0_SPEC, 30> {
        CLR30_W::new(self)
    }
    #[doc = "Bit 31 - Clear 31"]
    #[inline(always)]
    #[must_use]
    pub fn clr31(&mut self) -> CLR31_W<GPCLR0_SPEC, 31> {
        CLR31_W::new(self)
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
#[doc = "GPIO Pin Output Clear 0\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpclr0::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPCLR0_SPEC;
impl crate::RegisterSpec for GPCLR0_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`gpclr0::W`](W) writer structure"]
impl crate::Writable for GPCLR0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0xffff_ffff;
}
