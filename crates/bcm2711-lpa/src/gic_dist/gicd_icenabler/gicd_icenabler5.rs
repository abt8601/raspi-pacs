#[doc = "Register `GICD_ICENABLER5` reader"]
pub struct R(crate::R<GICD_ICENABLER5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GICD_ICENABLER5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GICD_ICENABLER5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GICD_ICENABLER5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GICD_ICENABLER5` writer"]
pub struct W(crate::W<GICD_ICENABLER5_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GICD_ICENABLER5_SPEC>;
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
impl From<crate::W<GICD_ICENABLER5_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GICD_ICENABLER5_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INT160` reader - Interrupt 160"]
pub type INT160_R = crate::BitReader<bool>;
#[doc = "Field `INT160` writer - Interrupt 160"]
pub type INT160_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, GICD_ICENABLER5_SPEC, bool, O>;
#[doc = "Field `INT161` reader - Interrupt 161"]
pub type INT161_R = crate::BitReader<bool>;
#[doc = "Field `INT161` writer - Interrupt 161"]
pub type INT161_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, GICD_ICENABLER5_SPEC, bool, O>;
#[doc = "Field `INT162` reader - Interrupt 162"]
pub type INT162_R = crate::BitReader<bool>;
#[doc = "Field `INT162` writer - Interrupt 162"]
pub type INT162_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, GICD_ICENABLER5_SPEC, bool, O>;
#[doc = "Field `INT163` reader - Interrupt 163"]
pub type INT163_R = crate::BitReader<bool>;
#[doc = "Field `INT163` writer - Interrupt 163"]
pub type INT163_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, GICD_ICENABLER5_SPEC, bool, O>;
#[doc = "Field `INT164` reader - Interrupt 164"]
pub type INT164_R = crate::BitReader<bool>;
#[doc = "Field `INT164` writer - Interrupt 164"]
pub type INT164_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, GICD_ICENABLER5_SPEC, bool, O>;
#[doc = "Field `INT165` reader - Interrupt 165"]
pub type INT165_R = crate::BitReader<bool>;
#[doc = "Field `INT165` writer - Interrupt 165"]
pub type INT165_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, GICD_ICENABLER5_SPEC, bool, O>;
#[doc = "Field `INT166` reader - Interrupt 166"]
pub type INT166_R = crate::BitReader<bool>;
#[doc = "Field `INT166` writer - Interrupt 166"]
pub type INT166_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, GICD_ICENABLER5_SPEC, bool, O>;
#[doc = "Field `INT167` reader - Interrupt 167"]
pub type INT167_R = crate::BitReader<bool>;
#[doc = "Field `INT167` writer - Interrupt 167"]
pub type INT167_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, GICD_ICENABLER5_SPEC, bool, O>;
#[doc = "Field `INT168` reader - Interrupt 168"]
pub type INT168_R = crate::BitReader<bool>;
#[doc = "Field `INT168` writer - Interrupt 168"]
pub type INT168_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, GICD_ICENABLER5_SPEC, bool, O>;
#[doc = "Field `INT169` reader - Interrupt 169"]
pub type INT169_R = crate::BitReader<bool>;
#[doc = "Field `INT169` writer - Interrupt 169"]
pub type INT169_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, GICD_ICENABLER5_SPEC, bool, O>;
#[doc = "Field `INT170` reader - Interrupt 170"]
pub type INT170_R = crate::BitReader<bool>;
#[doc = "Field `INT170` writer - Interrupt 170"]
pub type INT170_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, GICD_ICENABLER5_SPEC, bool, O>;
#[doc = "Field `INT171` reader - Interrupt 171"]
pub type INT171_R = crate::BitReader<bool>;
#[doc = "Field `INT171` writer - Interrupt 171"]
pub type INT171_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, GICD_ICENABLER5_SPEC, bool, O>;
#[doc = "Field `INT172` reader - Interrupt 172"]
pub type INT172_R = crate::BitReader<bool>;
#[doc = "Field `INT172` writer - Interrupt 172"]
pub type INT172_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, GICD_ICENABLER5_SPEC, bool, O>;
#[doc = "Field `INT173` reader - Interrupt 173"]
pub type INT173_R = crate::BitReader<bool>;
#[doc = "Field `INT173` writer - Interrupt 173"]
pub type INT173_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, GICD_ICENABLER5_SPEC, bool, O>;
#[doc = "Field `INT174` reader - Interrupt 174"]
pub type INT174_R = crate::BitReader<bool>;
#[doc = "Field `INT174` writer - Interrupt 174"]
pub type INT174_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, GICD_ICENABLER5_SPEC, bool, O>;
#[doc = "Field `INT175` reader - Interrupt 175"]
pub type INT175_R = crate::BitReader<bool>;
#[doc = "Field `INT175` writer - Interrupt 175"]
pub type INT175_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, GICD_ICENABLER5_SPEC, bool, O>;
#[doc = "Field `INT176` reader - Interrupt 176"]
pub type INT176_R = crate::BitReader<bool>;
#[doc = "Field `INT176` writer - Interrupt 176"]
pub type INT176_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, GICD_ICENABLER5_SPEC, bool, O>;
#[doc = "Field `INT177` reader - Interrupt 177"]
pub type INT177_R = crate::BitReader<bool>;
#[doc = "Field `INT177` writer - Interrupt 177"]
pub type INT177_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, GICD_ICENABLER5_SPEC, bool, O>;
#[doc = "Field `INT178` reader - Interrupt 178"]
pub type INT178_R = crate::BitReader<bool>;
#[doc = "Field `INT178` writer - Interrupt 178"]
pub type INT178_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, GICD_ICENABLER5_SPEC, bool, O>;
#[doc = "Field `INT179` reader - Interrupt 179"]
pub type INT179_R = crate::BitReader<bool>;
#[doc = "Field `INT179` writer - Interrupt 179"]
pub type INT179_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, GICD_ICENABLER5_SPEC, bool, O>;
#[doc = "Field `INT180` reader - Interrupt 180"]
pub type INT180_R = crate::BitReader<bool>;
#[doc = "Field `INT180` writer - Interrupt 180"]
pub type INT180_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, GICD_ICENABLER5_SPEC, bool, O>;
#[doc = "Field `INT181` reader - Interrupt 181"]
pub type INT181_R = crate::BitReader<bool>;
#[doc = "Field `INT181` writer - Interrupt 181"]
pub type INT181_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, GICD_ICENABLER5_SPEC, bool, O>;
#[doc = "Field `INT182` reader - Interrupt 182"]
pub type INT182_R = crate::BitReader<bool>;
#[doc = "Field `INT182` writer - Interrupt 182"]
pub type INT182_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, GICD_ICENABLER5_SPEC, bool, O>;
#[doc = "Field `INT183` reader - Interrupt 183"]
pub type INT183_R = crate::BitReader<bool>;
#[doc = "Field `INT183` writer - Interrupt 183"]
pub type INT183_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, GICD_ICENABLER5_SPEC, bool, O>;
#[doc = "Field `INT184` reader - Interrupt 184"]
pub type INT184_R = crate::BitReader<bool>;
#[doc = "Field `INT184` writer - Interrupt 184"]
pub type INT184_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, GICD_ICENABLER5_SPEC, bool, O>;
#[doc = "Field `INT185` reader - Interrupt 185"]
pub type INT185_R = crate::BitReader<bool>;
#[doc = "Field `INT185` writer - Interrupt 185"]
pub type INT185_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, GICD_ICENABLER5_SPEC, bool, O>;
#[doc = "Field `INT186` reader - Interrupt 186"]
pub type INT186_R = crate::BitReader<bool>;
#[doc = "Field `INT186` writer - Interrupt 186"]
pub type INT186_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, GICD_ICENABLER5_SPEC, bool, O>;
#[doc = "Field `INT187` reader - Interrupt 187"]
pub type INT187_R = crate::BitReader<bool>;
#[doc = "Field `INT187` writer - Interrupt 187"]
pub type INT187_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, GICD_ICENABLER5_SPEC, bool, O>;
#[doc = "Field `INT188` reader - Interrupt 188"]
pub type INT188_R = crate::BitReader<bool>;
#[doc = "Field `INT188` writer - Interrupt 188"]
pub type INT188_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, GICD_ICENABLER5_SPEC, bool, O>;
#[doc = "Field `INT189` reader - Interrupt 189"]
pub type INT189_R = crate::BitReader<bool>;
#[doc = "Field `INT189` writer - Interrupt 189"]
pub type INT189_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, GICD_ICENABLER5_SPEC, bool, O>;
#[doc = "Field `INT190` reader - Interrupt 190"]
pub type INT190_R = crate::BitReader<bool>;
#[doc = "Field `INT190` writer - Interrupt 190"]
pub type INT190_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, GICD_ICENABLER5_SPEC, bool, O>;
#[doc = "Field `INT191` reader - Interrupt 191"]
pub type INT191_R = crate::BitReader<bool>;
#[doc = "Field `INT191` writer - Interrupt 191"]
pub type INT191_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, GICD_ICENABLER5_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Interrupt 160"]
    #[inline(always)]
    pub fn int160(&self) -> INT160_R {
        INT160_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Interrupt 161"]
    #[inline(always)]
    pub fn int161(&self) -> INT161_R {
        INT161_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Interrupt 162"]
    #[inline(always)]
    pub fn int162(&self) -> INT162_R {
        INT162_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Interrupt 163"]
    #[inline(always)]
    pub fn int163(&self) -> INT163_R {
        INT163_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Interrupt 164"]
    #[inline(always)]
    pub fn int164(&self) -> INT164_R {
        INT164_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Interrupt 165"]
    #[inline(always)]
    pub fn int165(&self) -> INT165_R {
        INT165_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Interrupt 166"]
    #[inline(always)]
    pub fn int166(&self) -> INT166_R {
        INT166_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Interrupt 167"]
    #[inline(always)]
    pub fn int167(&self) -> INT167_R {
        INT167_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Interrupt 168"]
    #[inline(always)]
    pub fn int168(&self) -> INT168_R {
        INT168_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Interrupt 169"]
    #[inline(always)]
    pub fn int169(&self) -> INT169_R {
        INT169_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Interrupt 170"]
    #[inline(always)]
    pub fn int170(&self) -> INT170_R {
        INT170_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Interrupt 171"]
    #[inline(always)]
    pub fn int171(&self) -> INT171_R {
        INT171_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Interrupt 172"]
    #[inline(always)]
    pub fn int172(&self) -> INT172_R {
        INT172_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Interrupt 173"]
    #[inline(always)]
    pub fn int173(&self) -> INT173_R {
        INT173_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Interrupt 174"]
    #[inline(always)]
    pub fn int174(&self) -> INT174_R {
        INT174_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Interrupt 175"]
    #[inline(always)]
    pub fn int175(&self) -> INT175_R {
        INT175_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Interrupt 176"]
    #[inline(always)]
    pub fn int176(&self) -> INT176_R {
        INT176_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Interrupt 177"]
    #[inline(always)]
    pub fn int177(&self) -> INT177_R {
        INT177_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Interrupt 178"]
    #[inline(always)]
    pub fn int178(&self) -> INT178_R {
        INT178_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Interrupt 179"]
    #[inline(always)]
    pub fn int179(&self) -> INT179_R {
        INT179_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Interrupt 180"]
    #[inline(always)]
    pub fn int180(&self) -> INT180_R {
        INT180_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Interrupt 181"]
    #[inline(always)]
    pub fn int181(&self) -> INT181_R {
        INT181_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Interrupt 182"]
    #[inline(always)]
    pub fn int182(&self) -> INT182_R {
        INT182_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Interrupt 183"]
    #[inline(always)]
    pub fn int183(&self) -> INT183_R {
        INT183_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Interrupt 184"]
    #[inline(always)]
    pub fn int184(&self) -> INT184_R {
        INT184_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Interrupt 185"]
    #[inline(always)]
    pub fn int185(&self) -> INT185_R {
        INT185_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Interrupt 186"]
    #[inline(always)]
    pub fn int186(&self) -> INT186_R {
        INT186_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Interrupt 187"]
    #[inline(always)]
    pub fn int187(&self) -> INT187_R {
        INT187_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Interrupt 188"]
    #[inline(always)]
    pub fn int188(&self) -> INT188_R {
        INT188_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Interrupt 189"]
    #[inline(always)]
    pub fn int189(&self) -> INT189_R {
        INT189_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Interrupt 190"]
    #[inline(always)]
    pub fn int190(&self) -> INT190_R {
        INT190_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Interrupt 191"]
    #[inline(always)]
    pub fn int191(&self) -> INT191_R {
        INT191_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt 160"]
    #[inline(always)]
    #[must_use]
    pub fn int160(&mut self) -> INT160_W<0> {
        INT160_W::new(self)
    }
    #[doc = "Bit 1 - Interrupt 161"]
    #[inline(always)]
    #[must_use]
    pub fn int161(&mut self) -> INT161_W<1> {
        INT161_W::new(self)
    }
    #[doc = "Bit 2 - Interrupt 162"]
    #[inline(always)]
    #[must_use]
    pub fn int162(&mut self) -> INT162_W<2> {
        INT162_W::new(self)
    }
    #[doc = "Bit 3 - Interrupt 163"]
    #[inline(always)]
    #[must_use]
    pub fn int163(&mut self) -> INT163_W<3> {
        INT163_W::new(self)
    }
    #[doc = "Bit 4 - Interrupt 164"]
    #[inline(always)]
    #[must_use]
    pub fn int164(&mut self) -> INT164_W<4> {
        INT164_W::new(self)
    }
    #[doc = "Bit 5 - Interrupt 165"]
    #[inline(always)]
    #[must_use]
    pub fn int165(&mut self) -> INT165_W<5> {
        INT165_W::new(self)
    }
    #[doc = "Bit 6 - Interrupt 166"]
    #[inline(always)]
    #[must_use]
    pub fn int166(&mut self) -> INT166_W<6> {
        INT166_W::new(self)
    }
    #[doc = "Bit 7 - Interrupt 167"]
    #[inline(always)]
    #[must_use]
    pub fn int167(&mut self) -> INT167_W<7> {
        INT167_W::new(self)
    }
    #[doc = "Bit 8 - Interrupt 168"]
    #[inline(always)]
    #[must_use]
    pub fn int168(&mut self) -> INT168_W<8> {
        INT168_W::new(self)
    }
    #[doc = "Bit 9 - Interrupt 169"]
    #[inline(always)]
    #[must_use]
    pub fn int169(&mut self) -> INT169_W<9> {
        INT169_W::new(self)
    }
    #[doc = "Bit 10 - Interrupt 170"]
    #[inline(always)]
    #[must_use]
    pub fn int170(&mut self) -> INT170_W<10> {
        INT170_W::new(self)
    }
    #[doc = "Bit 11 - Interrupt 171"]
    #[inline(always)]
    #[must_use]
    pub fn int171(&mut self) -> INT171_W<11> {
        INT171_W::new(self)
    }
    #[doc = "Bit 12 - Interrupt 172"]
    #[inline(always)]
    #[must_use]
    pub fn int172(&mut self) -> INT172_W<12> {
        INT172_W::new(self)
    }
    #[doc = "Bit 13 - Interrupt 173"]
    #[inline(always)]
    #[must_use]
    pub fn int173(&mut self) -> INT173_W<13> {
        INT173_W::new(self)
    }
    #[doc = "Bit 14 - Interrupt 174"]
    #[inline(always)]
    #[must_use]
    pub fn int174(&mut self) -> INT174_W<14> {
        INT174_W::new(self)
    }
    #[doc = "Bit 15 - Interrupt 175"]
    #[inline(always)]
    #[must_use]
    pub fn int175(&mut self) -> INT175_W<15> {
        INT175_W::new(self)
    }
    #[doc = "Bit 16 - Interrupt 176"]
    #[inline(always)]
    #[must_use]
    pub fn int176(&mut self) -> INT176_W<16> {
        INT176_W::new(self)
    }
    #[doc = "Bit 17 - Interrupt 177"]
    #[inline(always)]
    #[must_use]
    pub fn int177(&mut self) -> INT177_W<17> {
        INT177_W::new(self)
    }
    #[doc = "Bit 18 - Interrupt 178"]
    #[inline(always)]
    #[must_use]
    pub fn int178(&mut self) -> INT178_W<18> {
        INT178_W::new(self)
    }
    #[doc = "Bit 19 - Interrupt 179"]
    #[inline(always)]
    #[must_use]
    pub fn int179(&mut self) -> INT179_W<19> {
        INT179_W::new(self)
    }
    #[doc = "Bit 20 - Interrupt 180"]
    #[inline(always)]
    #[must_use]
    pub fn int180(&mut self) -> INT180_W<20> {
        INT180_W::new(self)
    }
    #[doc = "Bit 21 - Interrupt 181"]
    #[inline(always)]
    #[must_use]
    pub fn int181(&mut self) -> INT181_W<21> {
        INT181_W::new(self)
    }
    #[doc = "Bit 22 - Interrupt 182"]
    #[inline(always)]
    #[must_use]
    pub fn int182(&mut self) -> INT182_W<22> {
        INT182_W::new(self)
    }
    #[doc = "Bit 23 - Interrupt 183"]
    #[inline(always)]
    #[must_use]
    pub fn int183(&mut self) -> INT183_W<23> {
        INT183_W::new(self)
    }
    #[doc = "Bit 24 - Interrupt 184"]
    #[inline(always)]
    #[must_use]
    pub fn int184(&mut self) -> INT184_W<24> {
        INT184_W::new(self)
    }
    #[doc = "Bit 25 - Interrupt 185"]
    #[inline(always)]
    #[must_use]
    pub fn int185(&mut self) -> INT185_W<25> {
        INT185_W::new(self)
    }
    #[doc = "Bit 26 - Interrupt 186"]
    #[inline(always)]
    #[must_use]
    pub fn int186(&mut self) -> INT186_W<26> {
        INT186_W::new(self)
    }
    #[doc = "Bit 27 - Interrupt 187"]
    #[inline(always)]
    #[must_use]
    pub fn int187(&mut self) -> INT187_W<27> {
        INT187_W::new(self)
    }
    #[doc = "Bit 28 - Interrupt 188"]
    #[inline(always)]
    #[must_use]
    pub fn int188(&mut self) -> INT188_W<28> {
        INT188_W::new(self)
    }
    #[doc = "Bit 29 - Interrupt 189"]
    #[inline(always)]
    #[must_use]
    pub fn int189(&mut self) -> INT189_W<29> {
        INT189_W::new(self)
    }
    #[doc = "Bit 30 - Interrupt 190"]
    #[inline(always)]
    #[must_use]
    pub fn int190(&mut self) -> INT190_W<30> {
        INT190_W::new(self)
    }
    #[doc = "Bit 31 - Interrupt 191"]
    #[inline(always)]
    #[must_use]
    pub fn int191(&mut self) -> INT191_W<31> {
        INT191_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Clear-Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_icenabler5](index.html) module"]
pub struct GICD_ICENABLER5_SPEC;
impl crate::RegisterSpec for GICD_ICENABLER5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gicd_icenabler5::R](R) reader structure"]
impl crate::Readable for GICD_ICENABLER5_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gicd_icenabler5::W](W) writer structure"]
impl crate::Writable for GICD_ICENABLER5_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0xffff_ffff;
}
#[doc = "`reset()` method sets GICD_ICENABLER5 to value 0"]
impl crate::Resettable for GICD_ICENABLER5_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
