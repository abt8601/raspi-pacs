#[doc = "Register `GPLEV1` reader"]
pub struct R(crate::R<GPLEV1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPLEV1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPLEV1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPLEV1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `LEV32` reader - Level 32"]
pub type LEV32_R = crate::BitReader<bool>;
#[doc = "Field `LEV33` reader - Level 33"]
pub type LEV33_R = crate::BitReader<bool>;
#[doc = "Field `LEV34` reader - Level 34"]
pub type LEV34_R = crate::BitReader<bool>;
#[doc = "Field `LEV35` reader - Level 35"]
pub type LEV35_R = crate::BitReader<bool>;
#[doc = "Field `LEV36` reader - Level 36"]
pub type LEV36_R = crate::BitReader<bool>;
#[doc = "Field `LEV37` reader - Level 37"]
pub type LEV37_R = crate::BitReader<bool>;
#[doc = "Field `LEV38` reader - Level 38"]
pub type LEV38_R = crate::BitReader<bool>;
#[doc = "Field `LEV39` reader - Level 39"]
pub type LEV39_R = crate::BitReader<bool>;
#[doc = "Field `LEV40` reader - Level 40"]
pub type LEV40_R = crate::BitReader<bool>;
#[doc = "Field `LEV41` reader - Level 41"]
pub type LEV41_R = crate::BitReader<bool>;
#[doc = "Field `LEV42` reader - Level 42"]
pub type LEV42_R = crate::BitReader<bool>;
#[doc = "Field `LEV43` reader - Level 43"]
pub type LEV43_R = crate::BitReader<bool>;
#[doc = "Field `LEV44` reader - Level 44"]
pub type LEV44_R = crate::BitReader<bool>;
#[doc = "Field `LEV45` reader - Level 45"]
pub type LEV45_R = crate::BitReader<bool>;
#[doc = "Field `LEV46` reader - Level 46"]
pub type LEV46_R = crate::BitReader<bool>;
#[doc = "Field `LEV47` reader - Level 47"]
pub type LEV47_R = crate::BitReader<bool>;
#[doc = "Field `LEV48` reader - Level 48"]
pub type LEV48_R = crate::BitReader<bool>;
#[doc = "Field `LEV49` reader - Level 49"]
pub type LEV49_R = crate::BitReader<bool>;
#[doc = "Field `LEV50` reader - Level 50"]
pub type LEV50_R = crate::BitReader<bool>;
#[doc = "Field `LEV51` reader - Level 51"]
pub type LEV51_R = crate::BitReader<bool>;
#[doc = "Field `LEV52` reader - Level 52"]
pub type LEV52_R = crate::BitReader<bool>;
#[doc = "Field `LEV53` reader - Level 53"]
pub type LEV53_R = crate::BitReader<bool>;
#[doc = "Field `LEV54` reader - Level 54"]
pub type LEV54_R = crate::BitReader<bool>;
#[doc = "Field `LEV55` reader - Level 55"]
pub type LEV55_R = crate::BitReader<bool>;
#[doc = "Field `LEV56` reader - Level 56"]
pub type LEV56_R = crate::BitReader<bool>;
#[doc = "Field `LEV57` reader - Level 57"]
pub type LEV57_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Level 32"]
    #[inline(always)]
    pub fn lev32(&self) -> LEV32_R {
        LEV32_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Level 33"]
    #[inline(always)]
    pub fn lev33(&self) -> LEV33_R {
        LEV33_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Level 34"]
    #[inline(always)]
    pub fn lev34(&self) -> LEV34_R {
        LEV34_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Level 35"]
    #[inline(always)]
    pub fn lev35(&self) -> LEV35_R {
        LEV35_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Level 36"]
    #[inline(always)]
    pub fn lev36(&self) -> LEV36_R {
        LEV36_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Level 37"]
    #[inline(always)]
    pub fn lev37(&self) -> LEV37_R {
        LEV37_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Level 38"]
    #[inline(always)]
    pub fn lev38(&self) -> LEV38_R {
        LEV38_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Level 39"]
    #[inline(always)]
    pub fn lev39(&self) -> LEV39_R {
        LEV39_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Level 40"]
    #[inline(always)]
    pub fn lev40(&self) -> LEV40_R {
        LEV40_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Level 41"]
    #[inline(always)]
    pub fn lev41(&self) -> LEV41_R {
        LEV41_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Level 42"]
    #[inline(always)]
    pub fn lev42(&self) -> LEV42_R {
        LEV42_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Level 43"]
    #[inline(always)]
    pub fn lev43(&self) -> LEV43_R {
        LEV43_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Level 44"]
    #[inline(always)]
    pub fn lev44(&self) -> LEV44_R {
        LEV44_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Level 45"]
    #[inline(always)]
    pub fn lev45(&self) -> LEV45_R {
        LEV45_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Level 46"]
    #[inline(always)]
    pub fn lev46(&self) -> LEV46_R {
        LEV46_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Level 47"]
    #[inline(always)]
    pub fn lev47(&self) -> LEV47_R {
        LEV47_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Level 48"]
    #[inline(always)]
    pub fn lev48(&self) -> LEV48_R {
        LEV48_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Level 49"]
    #[inline(always)]
    pub fn lev49(&self) -> LEV49_R {
        LEV49_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Level 50"]
    #[inline(always)]
    pub fn lev50(&self) -> LEV50_R {
        LEV50_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Level 51"]
    #[inline(always)]
    pub fn lev51(&self) -> LEV51_R {
        LEV51_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Level 52"]
    #[inline(always)]
    pub fn lev52(&self) -> LEV52_R {
        LEV52_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Level 53"]
    #[inline(always)]
    pub fn lev53(&self) -> LEV53_R {
        LEV53_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Level 54"]
    #[inline(always)]
    pub fn lev54(&self) -> LEV54_R {
        LEV54_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Level 55"]
    #[inline(always)]
    pub fn lev55(&self) -> LEV55_R {
        LEV55_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Level 56"]
    #[inline(always)]
    pub fn lev56(&self) -> LEV56_R {
        LEV56_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Level 57"]
    #[inline(always)]
    pub fn lev57(&self) -> LEV57_R {
        LEV57_R::new(((self.bits >> 25) & 1) != 0)
    }
}
#[doc = "GPIO Pin Level 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gplev1](index.html) module"]
pub struct GPLEV1_SPEC;
impl crate::RegisterSpec for GPLEV1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gplev1::R](R) reader structure"]
impl crate::Readable for GPLEV1_SPEC {
    type Reader = R;
}
