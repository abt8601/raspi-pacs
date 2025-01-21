#[doc = "Register `GPLEV0` reader"]
pub type R = crate::R<GPLEV0_SPEC>;
#[doc = "Field `LEV0` reader - Level 0"]
pub type LEV0_R = crate::BitReader;
#[doc = "Field `LEV1` reader - Level 1"]
pub type LEV1_R = crate::BitReader;
#[doc = "Field `LEV2` reader - Level 2"]
pub type LEV2_R = crate::BitReader;
#[doc = "Field `LEV3` reader - Level 3"]
pub type LEV3_R = crate::BitReader;
#[doc = "Field `LEV4` reader - Level 4"]
pub type LEV4_R = crate::BitReader;
#[doc = "Field `LEV5` reader - Level 5"]
pub type LEV5_R = crate::BitReader;
#[doc = "Field `LEV6` reader - Level 6"]
pub type LEV6_R = crate::BitReader;
#[doc = "Field `LEV7` reader - Level 7"]
pub type LEV7_R = crate::BitReader;
#[doc = "Field `LEV8` reader - Level 8"]
pub type LEV8_R = crate::BitReader;
#[doc = "Field `LEV9` reader - Level 9"]
pub type LEV9_R = crate::BitReader;
#[doc = "Field `LEV10` reader - Level 10"]
pub type LEV10_R = crate::BitReader;
#[doc = "Field `LEV11` reader - Level 11"]
pub type LEV11_R = crate::BitReader;
#[doc = "Field `LEV12` reader - Level 12"]
pub type LEV12_R = crate::BitReader;
#[doc = "Field `LEV13` reader - Level 13"]
pub type LEV13_R = crate::BitReader;
#[doc = "Field `LEV14` reader - Level 14"]
pub type LEV14_R = crate::BitReader;
#[doc = "Field `LEV15` reader - Level 15"]
pub type LEV15_R = crate::BitReader;
#[doc = "Field `LEV16` reader - Level 16"]
pub type LEV16_R = crate::BitReader;
#[doc = "Field `LEV17` reader - Level 17"]
pub type LEV17_R = crate::BitReader;
#[doc = "Field `LEV18` reader - Level 18"]
pub type LEV18_R = crate::BitReader;
#[doc = "Field `LEV19` reader - Level 19"]
pub type LEV19_R = crate::BitReader;
#[doc = "Field `LEV20` reader - Level 20"]
pub type LEV20_R = crate::BitReader;
#[doc = "Field `LEV21` reader - Level 21"]
pub type LEV21_R = crate::BitReader;
#[doc = "Field `LEV22` reader - Level 22"]
pub type LEV22_R = crate::BitReader;
#[doc = "Field `LEV23` reader - Level 23"]
pub type LEV23_R = crate::BitReader;
#[doc = "Field `LEV24` reader - Level 24"]
pub type LEV24_R = crate::BitReader;
#[doc = "Field `LEV25` reader - Level 25"]
pub type LEV25_R = crate::BitReader;
#[doc = "Field `LEV26` reader - Level 26"]
pub type LEV26_R = crate::BitReader;
#[doc = "Field `LEV27` reader - Level 27"]
pub type LEV27_R = crate::BitReader;
#[doc = "Field `LEV28` reader - Level 28"]
pub type LEV28_R = crate::BitReader;
#[doc = "Field `LEV29` reader - Level 29"]
pub type LEV29_R = crate::BitReader;
#[doc = "Field `LEV30` reader - Level 30"]
pub type LEV30_R = crate::BitReader;
#[doc = "Field `LEV31` reader - Level 31"]
pub type LEV31_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Level 0"]
    #[inline(always)]
    pub fn lev0(&self) -> LEV0_R {
        LEV0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Level 1"]
    #[inline(always)]
    pub fn lev1(&self) -> LEV1_R {
        LEV1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Level 2"]
    #[inline(always)]
    pub fn lev2(&self) -> LEV2_R {
        LEV2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Level 3"]
    #[inline(always)]
    pub fn lev3(&self) -> LEV3_R {
        LEV3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Level 4"]
    #[inline(always)]
    pub fn lev4(&self) -> LEV4_R {
        LEV4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Level 5"]
    #[inline(always)]
    pub fn lev5(&self) -> LEV5_R {
        LEV5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Level 6"]
    #[inline(always)]
    pub fn lev6(&self) -> LEV6_R {
        LEV6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Level 7"]
    #[inline(always)]
    pub fn lev7(&self) -> LEV7_R {
        LEV7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Level 8"]
    #[inline(always)]
    pub fn lev8(&self) -> LEV8_R {
        LEV8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Level 9"]
    #[inline(always)]
    pub fn lev9(&self) -> LEV9_R {
        LEV9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Level 10"]
    #[inline(always)]
    pub fn lev10(&self) -> LEV10_R {
        LEV10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Level 11"]
    #[inline(always)]
    pub fn lev11(&self) -> LEV11_R {
        LEV11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Level 12"]
    #[inline(always)]
    pub fn lev12(&self) -> LEV12_R {
        LEV12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Level 13"]
    #[inline(always)]
    pub fn lev13(&self) -> LEV13_R {
        LEV13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Level 14"]
    #[inline(always)]
    pub fn lev14(&self) -> LEV14_R {
        LEV14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Level 15"]
    #[inline(always)]
    pub fn lev15(&self) -> LEV15_R {
        LEV15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Level 16"]
    #[inline(always)]
    pub fn lev16(&self) -> LEV16_R {
        LEV16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Level 17"]
    #[inline(always)]
    pub fn lev17(&self) -> LEV17_R {
        LEV17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Level 18"]
    #[inline(always)]
    pub fn lev18(&self) -> LEV18_R {
        LEV18_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Level 19"]
    #[inline(always)]
    pub fn lev19(&self) -> LEV19_R {
        LEV19_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Level 20"]
    #[inline(always)]
    pub fn lev20(&self) -> LEV20_R {
        LEV20_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Level 21"]
    #[inline(always)]
    pub fn lev21(&self) -> LEV21_R {
        LEV21_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Level 22"]
    #[inline(always)]
    pub fn lev22(&self) -> LEV22_R {
        LEV22_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Level 23"]
    #[inline(always)]
    pub fn lev23(&self) -> LEV23_R {
        LEV23_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Level 24"]
    #[inline(always)]
    pub fn lev24(&self) -> LEV24_R {
        LEV24_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Level 25"]
    #[inline(always)]
    pub fn lev25(&self) -> LEV25_R {
        LEV25_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Level 26"]
    #[inline(always)]
    pub fn lev26(&self) -> LEV26_R {
        LEV26_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Level 27"]
    #[inline(always)]
    pub fn lev27(&self) -> LEV27_R {
        LEV27_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Level 28"]
    #[inline(always)]
    pub fn lev28(&self) -> LEV28_R {
        LEV28_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Level 29"]
    #[inline(always)]
    pub fn lev29(&self) -> LEV29_R {
        LEV29_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Level 30"]
    #[inline(always)]
    pub fn lev30(&self) -> LEV30_R {
        LEV30_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Level 31"]
    #[inline(always)]
    pub fn lev31(&self) -> LEV31_R {
        LEV31_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPLEV0")
            .field("lev0", &format_args!("{}", self.lev0().bit()))
            .field("lev1", &format_args!("{}", self.lev1().bit()))
            .field("lev2", &format_args!("{}", self.lev2().bit()))
            .field("lev3", &format_args!("{}", self.lev3().bit()))
            .field("lev4", &format_args!("{}", self.lev4().bit()))
            .field("lev5", &format_args!("{}", self.lev5().bit()))
            .field("lev6", &format_args!("{}", self.lev6().bit()))
            .field("lev7", &format_args!("{}", self.lev7().bit()))
            .field("lev8", &format_args!("{}", self.lev8().bit()))
            .field("lev9", &format_args!("{}", self.lev9().bit()))
            .field("lev10", &format_args!("{}", self.lev10().bit()))
            .field("lev11", &format_args!("{}", self.lev11().bit()))
            .field("lev12", &format_args!("{}", self.lev12().bit()))
            .field("lev13", &format_args!("{}", self.lev13().bit()))
            .field("lev14", &format_args!("{}", self.lev14().bit()))
            .field("lev15", &format_args!("{}", self.lev15().bit()))
            .field("lev16", &format_args!("{}", self.lev16().bit()))
            .field("lev17", &format_args!("{}", self.lev17().bit()))
            .field("lev18", &format_args!("{}", self.lev18().bit()))
            .field("lev19", &format_args!("{}", self.lev19().bit()))
            .field("lev20", &format_args!("{}", self.lev20().bit()))
            .field("lev21", &format_args!("{}", self.lev21().bit()))
            .field("lev22", &format_args!("{}", self.lev22().bit()))
            .field("lev23", &format_args!("{}", self.lev23().bit()))
            .field("lev24", &format_args!("{}", self.lev24().bit()))
            .field("lev25", &format_args!("{}", self.lev25().bit()))
            .field("lev26", &format_args!("{}", self.lev26().bit()))
            .field("lev27", &format_args!("{}", self.lev27().bit()))
            .field("lev28", &format_args!("{}", self.lev28().bit()))
            .field("lev29", &format_args!("{}", self.lev29().bit()))
            .field("lev30", &format_args!("{}", self.lev30().bit()))
            .field("lev31", &format_args!("{}", self.lev31().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<GPLEV0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "GPIO Pin Level 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gplev0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPLEV0_SPEC;
impl crate::RegisterSpec for GPLEV0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gplev0::R`](R) reader structure"]
impl crate::Readable for GPLEV0_SPEC {}
