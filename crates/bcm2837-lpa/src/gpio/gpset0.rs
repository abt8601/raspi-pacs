#[doc = "Register `GPSET0` writer"]
pub type W = crate::W<GPSET0_SPEC>;
#[doc = "Field `SET0` writer - Set 0"]
pub type SET0_W<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `SET1` writer - Set 1"]
pub type SET1_W<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `SET2` writer - Set 2"]
pub type SET2_W<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `SET3` writer - Set 3"]
pub type SET3_W<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `SET4` writer - Set 4"]
pub type SET4_W<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `SET5` writer - Set 5"]
pub type SET5_W<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `SET6` writer - Set 6"]
pub type SET6_W<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `SET7` writer - Set 7"]
pub type SET7_W<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `SET8` writer - Set 8"]
pub type SET8_W<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `SET9` writer - Set 9"]
pub type SET9_W<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `SET10` writer - Set 10"]
pub type SET10_W<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `SET11` writer - Set 11"]
pub type SET11_W<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `SET12` writer - Set 12"]
pub type SET12_W<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `SET13` writer - Set 13"]
pub type SET13_W<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `SET14` writer - Set 14"]
pub type SET14_W<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `SET15` writer - Set 15"]
pub type SET15_W<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `SET16` writer - Set 16"]
pub type SET16_W<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `SET17` writer - Set 17"]
pub type SET17_W<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `SET18` writer - Set 18"]
pub type SET18_W<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `SET19` writer - Set 19"]
pub type SET19_W<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `SET20` writer - Set 20"]
pub type SET20_W<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `SET21` writer - Set 21"]
pub type SET21_W<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `SET22` writer - Set 22"]
pub type SET22_W<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `SET23` writer - Set 23"]
pub type SET23_W<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `SET24` writer - Set 24"]
pub type SET24_W<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `SET25` writer - Set 25"]
pub type SET25_W<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `SET26` writer - Set 26"]
pub type SET26_W<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `SET27` writer - Set 27"]
pub type SET27_W<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `SET28` writer - Set 28"]
pub type SET28_W<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `SET29` writer - Set 29"]
pub type SET29_W<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `SET30` writer - Set 30"]
pub type SET30_W<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `SET31` writer - Set 31"]
pub type SET31_W<'a, REG> = crate::BitWriter1S<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<GPSET0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Set 0"]
    #[inline(always)]
    #[must_use]
    pub fn set0(&mut self) -> SET0_W<GPSET0_SPEC> {
        SET0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Set 1"]
    #[inline(always)]
    #[must_use]
    pub fn set1(&mut self) -> SET1_W<GPSET0_SPEC> {
        SET1_W::new(self, 1)
    }
    #[doc = "Bit 2 - Set 2"]
    #[inline(always)]
    #[must_use]
    pub fn set2(&mut self) -> SET2_W<GPSET0_SPEC> {
        SET2_W::new(self, 2)
    }
    #[doc = "Bit 3 - Set 3"]
    #[inline(always)]
    #[must_use]
    pub fn set3(&mut self) -> SET3_W<GPSET0_SPEC> {
        SET3_W::new(self, 3)
    }
    #[doc = "Bit 4 - Set 4"]
    #[inline(always)]
    #[must_use]
    pub fn set4(&mut self) -> SET4_W<GPSET0_SPEC> {
        SET4_W::new(self, 4)
    }
    #[doc = "Bit 5 - Set 5"]
    #[inline(always)]
    #[must_use]
    pub fn set5(&mut self) -> SET5_W<GPSET0_SPEC> {
        SET5_W::new(self, 5)
    }
    #[doc = "Bit 6 - Set 6"]
    #[inline(always)]
    #[must_use]
    pub fn set6(&mut self) -> SET6_W<GPSET0_SPEC> {
        SET6_W::new(self, 6)
    }
    #[doc = "Bit 7 - Set 7"]
    #[inline(always)]
    #[must_use]
    pub fn set7(&mut self) -> SET7_W<GPSET0_SPEC> {
        SET7_W::new(self, 7)
    }
    #[doc = "Bit 8 - Set 8"]
    #[inline(always)]
    #[must_use]
    pub fn set8(&mut self) -> SET8_W<GPSET0_SPEC> {
        SET8_W::new(self, 8)
    }
    #[doc = "Bit 9 - Set 9"]
    #[inline(always)]
    #[must_use]
    pub fn set9(&mut self) -> SET9_W<GPSET0_SPEC> {
        SET9_W::new(self, 9)
    }
    #[doc = "Bit 10 - Set 10"]
    #[inline(always)]
    #[must_use]
    pub fn set10(&mut self) -> SET10_W<GPSET0_SPEC> {
        SET10_W::new(self, 10)
    }
    #[doc = "Bit 11 - Set 11"]
    #[inline(always)]
    #[must_use]
    pub fn set11(&mut self) -> SET11_W<GPSET0_SPEC> {
        SET11_W::new(self, 11)
    }
    #[doc = "Bit 12 - Set 12"]
    #[inline(always)]
    #[must_use]
    pub fn set12(&mut self) -> SET12_W<GPSET0_SPEC> {
        SET12_W::new(self, 12)
    }
    #[doc = "Bit 13 - Set 13"]
    #[inline(always)]
    #[must_use]
    pub fn set13(&mut self) -> SET13_W<GPSET0_SPEC> {
        SET13_W::new(self, 13)
    }
    #[doc = "Bit 14 - Set 14"]
    #[inline(always)]
    #[must_use]
    pub fn set14(&mut self) -> SET14_W<GPSET0_SPEC> {
        SET14_W::new(self, 14)
    }
    #[doc = "Bit 15 - Set 15"]
    #[inline(always)]
    #[must_use]
    pub fn set15(&mut self) -> SET15_W<GPSET0_SPEC> {
        SET15_W::new(self, 15)
    }
    #[doc = "Bit 16 - Set 16"]
    #[inline(always)]
    #[must_use]
    pub fn set16(&mut self) -> SET16_W<GPSET0_SPEC> {
        SET16_W::new(self, 16)
    }
    #[doc = "Bit 17 - Set 17"]
    #[inline(always)]
    #[must_use]
    pub fn set17(&mut self) -> SET17_W<GPSET0_SPEC> {
        SET17_W::new(self, 17)
    }
    #[doc = "Bit 18 - Set 18"]
    #[inline(always)]
    #[must_use]
    pub fn set18(&mut self) -> SET18_W<GPSET0_SPEC> {
        SET18_W::new(self, 18)
    }
    #[doc = "Bit 19 - Set 19"]
    #[inline(always)]
    #[must_use]
    pub fn set19(&mut self) -> SET19_W<GPSET0_SPEC> {
        SET19_W::new(self, 19)
    }
    #[doc = "Bit 20 - Set 20"]
    #[inline(always)]
    #[must_use]
    pub fn set20(&mut self) -> SET20_W<GPSET0_SPEC> {
        SET20_W::new(self, 20)
    }
    #[doc = "Bit 21 - Set 21"]
    #[inline(always)]
    #[must_use]
    pub fn set21(&mut self) -> SET21_W<GPSET0_SPEC> {
        SET21_W::new(self, 21)
    }
    #[doc = "Bit 22 - Set 22"]
    #[inline(always)]
    #[must_use]
    pub fn set22(&mut self) -> SET22_W<GPSET0_SPEC> {
        SET22_W::new(self, 22)
    }
    #[doc = "Bit 23 - Set 23"]
    #[inline(always)]
    #[must_use]
    pub fn set23(&mut self) -> SET23_W<GPSET0_SPEC> {
        SET23_W::new(self, 23)
    }
    #[doc = "Bit 24 - Set 24"]
    #[inline(always)]
    #[must_use]
    pub fn set24(&mut self) -> SET24_W<GPSET0_SPEC> {
        SET24_W::new(self, 24)
    }
    #[doc = "Bit 25 - Set 25"]
    #[inline(always)]
    #[must_use]
    pub fn set25(&mut self) -> SET25_W<GPSET0_SPEC> {
        SET25_W::new(self, 25)
    }
    #[doc = "Bit 26 - Set 26"]
    #[inline(always)]
    #[must_use]
    pub fn set26(&mut self) -> SET26_W<GPSET0_SPEC> {
        SET26_W::new(self, 26)
    }
    #[doc = "Bit 27 - Set 27"]
    #[inline(always)]
    #[must_use]
    pub fn set27(&mut self) -> SET27_W<GPSET0_SPEC> {
        SET27_W::new(self, 27)
    }
    #[doc = "Bit 28 - Set 28"]
    #[inline(always)]
    #[must_use]
    pub fn set28(&mut self) -> SET28_W<GPSET0_SPEC> {
        SET28_W::new(self, 28)
    }
    #[doc = "Bit 29 - Set 29"]
    #[inline(always)]
    #[must_use]
    pub fn set29(&mut self) -> SET29_W<GPSET0_SPEC> {
        SET29_W::new(self, 29)
    }
    #[doc = "Bit 30 - Set 30"]
    #[inline(always)]
    #[must_use]
    pub fn set30(&mut self) -> SET30_W<GPSET0_SPEC> {
        SET30_W::new(self, 30)
    }
    #[doc = "Bit 31 - Set 31"]
    #[inline(always)]
    #[must_use]
    pub fn set31(&mut self) -> SET31_W<GPSET0_SPEC> {
        SET31_W::new(self, 31)
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
#[doc = "GPIO Pin Output Set 0\n\nYou can [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpset0::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPSET0_SPEC;
impl crate::RegisterSpec for GPSET0_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`gpset0::W`](W) writer structure"]
impl crate::Writable for GPSET0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0xffff_ffff;
}
