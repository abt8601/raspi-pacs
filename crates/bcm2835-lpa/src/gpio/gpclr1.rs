#[doc = "Register `GPCLR1` writer"]
pub struct W(crate::W<GPCLR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPCLR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<GPCLR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPCLR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLR32` writer - Clear 32"]
pub type CLR32_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, GPCLR1_SPEC, bool, O>;
#[doc = "Field `CLR33` writer - Clear 33"]
pub type CLR33_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, GPCLR1_SPEC, bool, O>;
#[doc = "Field `CLR34` writer - Clear 34"]
pub type CLR34_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, GPCLR1_SPEC, bool, O>;
#[doc = "Field `CLR35` writer - Clear 35"]
pub type CLR35_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, GPCLR1_SPEC, bool, O>;
#[doc = "Field `CLR36` writer - Clear 36"]
pub type CLR36_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, GPCLR1_SPEC, bool, O>;
#[doc = "Field `CLR37` writer - Clear 37"]
pub type CLR37_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, GPCLR1_SPEC, bool, O>;
#[doc = "Field `CLR38` writer - Clear 38"]
pub type CLR38_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, GPCLR1_SPEC, bool, O>;
#[doc = "Field `CLR39` writer - Clear 39"]
pub type CLR39_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, GPCLR1_SPEC, bool, O>;
#[doc = "Field `CLR40` writer - Clear 40"]
pub type CLR40_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, GPCLR1_SPEC, bool, O>;
#[doc = "Field `CLR41` writer - Clear 41"]
pub type CLR41_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, GPCLR1_SPEC, bool, O>;
#[doc = "Field `CLR42` writer - Clear 42"]
pub type CLR42_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, GPCLR1_SPEC, bool, O>;
#[doc = "Field `CLR43` writer - Clear 43"]
pub type CLR43_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, GPCLR1_SPEC, bool, O>;
#[doc = "Field `CLR44` writer - Clear 44"]
pub type CLR44_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, GPCLR1_SPEC, bool, O>;
#[doc = "Field `CLR45` writer - Clear 45"]
pub type CLR45_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, GPCLR1_SPEC, bool, O>;
#[doc = "Field `CLR46` writer - Clear 46"]
pub type CLR46_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, GPCLR1_SPEC, bool, O>;
#[doc = "Field `CLR47` writer - Clear 47"]
pub type CLR47_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, GPCLR1_SPEC, bool, O>;
#[doc = "Field `CLR48` writer - Clear 48"]
pub type CLR48_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, GPCLR1_SPEC, bool, O>;
#[doc = "Field `CLR49` writer - Clear 49"]
pub type CLR49_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, GPCLR1_SPEC, bool, O>;
#[doc = "Field `CLR50` writer - Clear 50"]
pub type CLR50_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, GPCLR1_SPEC, bool, O>;
#[doc = "Field `CLR51` writer - Clear 51"]
pub type CLR51_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, GPCLR1_SPEC, bool, O>;
#[doc = "Field `CLR52` writer - Clear 52"]
pub type CLR52_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, GPCLR1_SPEC, bool, O>;
#[doc = "Field `CLR53` writer - Clear 53"]
pub type CLR53_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, GPCLR1_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - Clear 32"]
    #[inline(always)]
    #[must_use]
    pub fn clr32(&mut self) -> CLR32_W<0> {
        CLR32_W::new(self)
    }
    #[doc = "Bit 1 - Clear 33"]
    #[inline(always)]
    #[must_use]
    pub fn clr33(&mut self) -> CLR33_W<1> {
        CLR33_W::new(self)
    }
    #[doc = "Bit 2 - Clear 34"]
    #[inline(always)]
    #[must_use]
    pub fn clr34(&mut self) -> CLR34_W<2> {
        CLR34_W::new(self)
    }
    #[doc = "Bit 3 - Clear 35"]
    #[inline(always)]
    #[must_use]
    pub fn clr35(&mut self) -> CLR35_W<3> {
        CLR35_W::new(self)
    }
    #[doc = "Bit 4 - Clear 36"]
    #[inline(always)]
    #[must_use]
    pub fn clr36(&mut self) -> CLR36_W<4> {
        CLR36_W::new(self)
    }
    #[doc = "Bit 5 - Clear 37"]
    #[inline(always)]
    #[must_use]
    pub fn clr37(&mut self) -> CLR37_W<5> {
        CLR37_W::new(self)
    }
    #[doc = "Bit 6 - Clear 38"]
    #[inline(always)]
    #[must_use]
    pub fn clr38(&mut self) -> CLR38_W<6> {
        CLR38_W::new(self)
    }
    #[doc = "Bit 7 - Clear 39"]
    #[inline(always)]
    #[must_use]
    pub fn clr39(&mut self) -> CLR39_W<7> {
        CLR39_W::new(self)
    }
    #[doc = "Bit 8 - Clear 40"]
    #[inline(always)]
    #[must_use]
    pub fn clr40(&mut self) -> CLR40_W<8> {
        CLR40_W::new(self)
    }
    #[doc = "Bit 9 - Clear 41"]
    #[inline(always)]
    #[must_use]
    pub fn clr41(&mut self) -> CLR41_W<9> {
        CLR41_W::new(self)
    }
    #[doc = "Bit 10 - Clear 42"]
    #[inline(always)]
    #[must_use]
    pub fn clr42(&mut self) -> CLR42_W<10> {
        CLR42_W::new(self)
    }
    #[doc = "Bit 11 - Clear 43"]
    #[inline(always)]
    #[must_use]
    pub fn clr43(&mut self) -> CLR43_W<11> {
        CLR43_W::new(self)
    }
    #[doc = "Bit 12 - Clear 44"]
    #[inline(always)]
    #[must_use]
    pub fn clr44(&mut self) -> CLR44_W<12> {
        CLR44_W::new(self)
    }
    #[doc = "Bit 13 - Clear 45"]
    #[inline(always)]
    #[must_use]
    pub fn clr45(&mut self) -> CLR45_W<13> {
        CLR45_W::new(self)
    }
    #[doc = "Bit 14 - Clear 46"]
    #[inline(always)]
    #[must_use]
    pub fn clr46(&mut self) -> CLR46_W<14> {
        CLR46_W::new(self)
    }
    #[doc = "Bit 15 - Clear 47"]
    #[inline(always)]
    #[must_use]
    pub fn clr47(&mut self) -> CLR47_W<15> {
        CLR47_W::new(self)
    }
    #[doc = "Bit 16 - Clear 48"]
    #[inline(always)]
    #[must_use]
    pub fn clr48(&mut self) -> CLR48_W<16> {
        CLR48_W::new(self)
    }
    #[doc = "Bit 17 - Clear 49"]
    #[inline(always)]
    #[must_use]
    pub fn clr49(&mut self) -> CLR49_W<17> {
        CLR49_W::new(self)
    }
    #[doc = "Bit 18 - Clear 50"]
    #[inline(always)]
    #[must_use]
    pub fn clr50(&mut self) -> CLR50_W<18> {
        CLR50_W::new(self)
    }
    #[doc = "Bit 19 - Clear 51"]
    #[inline(always)]
    #[must_use]
    pub fn clr51(&mut self) -> CLR51_W<19> {
        CLR51_W::new(self)
    }
    #[doc = "Bit 20 - Clear 52"]
    #[inline(always)]
    #[must_use]
    pub fn clr52(&mut self) -> CLR52_W<20> {
        CLR52_W::new(self)
    }
    #[doc = "Bit 21 - Clear 53"]
    #[inline(always)]
    #[must_use]
    pub fn clr53(&mut self) -> CLR53_W<21> {
        CLR53_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO Pin Output Clear 1\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpclr1](index.html) module"]
pub struct GPCLR1_SPEC;
impl crate::RegisterSpec for GPCLR1_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [gpclr1::W](W) writer structure"]
impl crate::Writable for GPCLR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x003f_ffff;
}
