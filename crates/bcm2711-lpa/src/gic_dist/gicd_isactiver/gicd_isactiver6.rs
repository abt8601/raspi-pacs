#[doc = "Register `GICD_ISACTIVER6` reader"]
pub struct R(crate::R<GICD_ISACTIVER6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GICD_ISACTIVER6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GICD_ISACTIVER6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GICD_ISACTIVER6_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GICD_ISACTIVER6` writer"]
pub struct W(crate::W<GICD_ISACTIVER6_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GICD_ISACTIVER6_SPEC>;
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
impl From<crate::W<GICD_ISACTIVER6_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GICD_ISACTIVER6_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INT192` reader - Interrupt 192"]
pub type INT192_R = crate::BitReader<bool>;
#[doc = "Field `INT192` writer - Interrupt 192"]
pub type INT192_W<'a, const O: u8> = crate::BitWriter1S<'a, u32, GICD_ISACTIVER6_SPEC, bool, O>;
#[doc = "Field `INT193` reader - Interrupt 193"]
pub type INT193_R = crate::BitReader<bool>;
#[doc = "Field `INT193` writer - Interrupt 193"]
pub type INT193_W<'a, const O: u8> = crate::BitWriter1S<'a, u32, GICD_ISACTIVER6_SPEC, bool, O>;
#[doc = "Field `INT194` reader - Interrupt 194"]
pub type INT194_R = crate::BitReader<bool>;
#[doc = "Field `INT194` writer - Interrupt 194"]
pub type INT194_W<'a, const O: u8> = crate::BitWriter1S<'a, u32, GICD_ISACTIVER6_SPEC, bool, O>;
#[doc = "Field `INT195` reader - Interrupt 195"]
pub type INT195_R = crate::BitReader<bool>;
#[doc = "Field `INT195` writer - Interrupt 195"]
pub type INT195_W<'a, const O: u8> = crate::BitWriter1S<'a, u32, GICD_ISACTIVER6_SPEC, bool, O>;
#[doc = "Field `INT196` reader - Interrupt 196"]
pub type INT196_R = crate::BitReader<bool>;
#[doc = "Field `INT196` writer - Interrupt 196"]
pub type INT196_W<'a, const O: u8> = crate::BitWriter1S<'a, u32, GICD_ISACTIVER6_SPEC, bool, O>;
#[doc = "Field `INT197` reader - Interrupt 197"]
pub type INT197_R = crate::BitReader<bool>;
#[doc = "Field `INT197` writer - Interrupt 197"]
pub type INT197_W<'a, const O: u8> = crate::BitWriter1S<'a, u32, GICD_ISACTIVER6_SPEC, bool, O>;
#[doc = "Field `INT198` reader - Interrupt 198"]
pub type INT198_R = crate::BitReader<bool>;
#[doc = "Field `INT198` writer - Interrupt 198"]
pub type INT198_W<'a, const O: u8> = crate::BitWriter1S<'a, u32, GICD_ISACTIVER6_SPEC, bool, O>;
#[doc = "Field `INT199` reader - Interrupt 199"]
pub type INT199_R = crate::BitReader<bool>;
#[doc = "Field `INT199` writer - Interrupt 199"]
pub type INT199_W<'a, const O: u8> = crate::BitWriter1S<'a, u32, GICD_ISACTIVER6_SPEC, bool, O>;
#[doc = "Field `INT200` reader - Interrupt 200"]
pub type INT200_R = crate::BitReader<bool>;
#[doc = "Field `INT200` writer - Interrupt 200"]
pub type INT200_W<'a, const O: u8> = crate::BitWriter1S<'a, u32, GICD_ISACTIVER6_SPEC, bool, O>;
#[doc = "Field `INT201` reader - Interrupt 201"]
pub type INT201_R = crate::BitReader<bool>;
#[doc = "Field `INT201` writer - Interrupt 201"]
pub type INT201_W<'a, const O: u8> = crate::BitWriter1S<'a, u32, GICD_ISACTIVER6_SPEC, bool, O>;
#[doc = "Field `INT202` reader - Interrupt 202"]
pub type INT202_R = crate::BitReader<bool>;
#[doc = "Field `INT202` writer - Interrupt 202"]
pub type INT202_W<'a, const O: u8> = crate::BitWriter1S<'a, u32, GICD_ISACTIVER6_SPEC, bool, O>;
#[doc = "Field `INT203` reader - Interrupt 203"]
pub type INT203_R = crate::BitReader<bool>;
#[doc = "Field `INT203` writer - Interrupt 203"]
pub type INT203_W<'a, const O: u8> = crate::BitWriter1S<'a, u32, GICD_ISACTIVER6_SPEC, bool, O>;
#[doc = "Field `INT204` reader - Interrupt 204"]
pub type INT204_R = crate::BitReader<bool>;
#[doc = "Field `INT204` writer - Interrupt 204"]
pub type INT204_W<'a, const O: u8> = crate::BitWriter1S<'a, u32, GICD_ISACTIVER6_SPEC, bool, O>;
#[doc = "Field `INT205` reader - Interrupt 205"]
pub type INT205_R = crate::BitReader<bool>;
#[doc = "Field `INT205` writer - Interrupt 205"]
pub type INT205_W<'a, const O: u8> = crate::BitWriter1S<'a, u32, GICD_ISACTIVER6_SPEC, bool, O>;
#[doc = "Field `INT206` reader - Interrupt 206"]
pub type INT206_R = crate::BitReader<bool>;
#[doc = "Field `INT206` writer - Interrupt 206"]
pub type INT206_W<'a, const O: u8> = crate::BitWriter1S<'a, u32, GICD_ISACTIVER6_SPEC, bool, O>;
#[doc = "Field `INT207` reader - Interrupt 207"]
pub type INT207_R = crate::BitReader<bool>;
#[doc = "Field `INT207` writer - Interrupt 207"]
pub type INT207_W<'a, const O: u8> = crate::BitWriter1S<'a, u32, GICD_ISACTIVER6_SPEC, bool, O>;
#[doc = "Field `INT208` reader - Interrupt 208"]
pub type INT208_R = crate::BitReader<bool>;
#[doc = "Field `INT208` writer - Interrupt 208"]
pub type INT208_W<'a, const O: u8> = crate::BitWriter1S<'a, u32, GICD_ISACTIVER6_SPEC, bool, O>;
#[doc = "Field `INT209` reader - Interrupt 209"]
pub type INT209_R = crate::BitReader<bool>;
#[doc = "Field `INT209` writer - Interrupt 209"]
pub type INT209_W<'a, const O: u8> = crate::BitWriter1S<'a, u32, GICD_ISACTIVER6_SPEC, bool, O>;
#[doc = "Field `INT210` reader - Interrupt 210"]
pub type INT210_R = crate::BitReader<bool>;
#[doc = "Field `INT210` writer - Interrupt 210"]
pub type INT210_W<'a, const O: u8> = crate::BitWriter1S<'a, u32, GICD_ISACTIVER6_SPEC, bool, O>;
#[doc = "Field `INT211` reader - Interrupt 211"]
pub type INT211_R = crate::BitReader<bool>;
#[doc = "Field `INT211` writer - Interrupt 211"]
pub type INT211_W<'a, const O: u8> = crate::BitWriter1S<'a, u32, GICD_ISACTIVER6_SPEC, bool, O>;
#[doc = "Field `INT212` reader - Interrupt 212"]
pub type INT212_R = crate::BitReader<bool>;
#[doc = "Field `INT212` writer - Interrupt 212"]
pub type INT212_W<'a, const O: u8> = crate::BitWriter1S<'a, u32, GICD_ISACTIVER6_SPEC, bool, O>;
#[doc = "Field `INT213` reader - Interrupt 213"]
pub type INT213_R = crate::BitReader<bool>;
#[doc = "Field `INT213` writer - Interrupt 213"]
pub type INT213_W<'a, const O: u8> = crate::BitWriter1S<'a, u32, GICD_ISACTIVER6_SPEC, bool, O>;
#[doc = "Field `INT214` reader - Interrupt 214"]
pub type INT214_R = crate::BitReader<bool>;
#[doc = "Field `INT214` writer - Interrupt 214"]
pub type INT214_W<'a, const O: u8> = crate::BitWriter1S<'a, u32, GICD_ISACTIVER6_SPEC, bool, O>;
#[doc = "Field `INT215` reader - Interrupt 215"]
pub type INT215_R = crate::BitReader<bool>;
#[doc = "Field `INT215` writer - Interrupt 215"]
pub type INT215_W<'a, const O: u8> = crate::BitWriter1S<'a, u32, GICD_ISACTIVER6_SPEC, bool, O>;
#[doc = "Field `INT216` reader - Interrupt 216"]
pub type INT216_R = crate::BitReader<bool>;
#[doc = "Field `INT216` writer - Interrupt 216"]
pub type INT216_W<'a, const O: u8> = crate::BitWriter1S<'a, u32, GICD_ISACTIVER6_SPEC, bool, O>;
#[doc = "Field `INT217` reader - Interrupt 217"]
pub type INT217_R = crate::BitReader<bool>;
#[doc = "Field `INT217` writer - Interrupt 217"]
pub type INT217_W<'a, const O: u8> = crate::BitWriter1S<'a, u32, GICD_ISACTIVER6_SPEC, bool, O>;
#[doc = "Field `INT218` reader - Interrupt 218"]
pub type INT218_R = crate::BitReader<bool>;
#[doc = "Field `INT218` writer - Interrupt 218"]
pub type INT218_W<'a, const O: u8> = crate::BitWriter1S<'a, u32, GICD_ISACTIVER6_SPEC, bool, O>;
#[doc = "Field `INT219` reader - Interrupt 219"]
pub type INT219_R = crate::BitReader<bool>;
#[doc = "Field `INT219` writer - Interrupt 219"]
pub type INT219_W<'a, const O: u8> = crate::BitWriter1S<'a, u32, GICD_ISACTIVER6_SPEC, bool, O>;
#[doc = "Field `INT220` reader - Interrupt 220"]
pub type INT220_R = crate::BitReader<bool>;
#[doc = "Field `INT220` writer - Interrupt 220"]
pub type INT220_W<'a, const O: u8> = crate::BitWriter1S<'a, u32, GICD_ISACTIVER6_SPEC, bool, O>;
#[doc = "Field `INT221` reader - Interrupt 221"]
pub type INT221_R = crate::BitReader<bool>;
#[doc = "Field `INT221` writer - Interrupt 221"]
pub type INT221_W<'a, const O: u8> = crate::BitWriter1S<'a, u32, GICD_ISACTIVER6_SPEC, bool, O>;
#[doc = "Field `INT222` reader - Interrupt 222"]
pub type INT222_R = crate::BitReader<bool>;
#[doc = "Field `INT222` writer - Interrupt 222"]
pub type INT222_W<'a, const O: u8> = crate::BitWriter1S<'a, u32, GICD_ISACTIVER6_SPEC, bool, O>;
#[doc = "Field `INT223` reader - Interrupt 223"]
pub type INT223_R = crate::BitReader<bool>;
#[doc = "Field `INT223` writer - Interrupt 223"]
pub type INT223_W<'a, const O: u8> = crate::BitWriter1S<'a, u32, GICD_ISACTIVER6_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Interrupt 192"]
    #[inline(always)]
    pub fn int192(&self) -> INT192_R {
        INT192_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Interrupt 193"]
    #[inline(always)]
    pub fn int193(&self) -> INT193_R {
        INT193_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Interrupt 194"]
    #[inline(always)]
    pub fn int194(&self) -> INT194_R {
        INT194_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Interrupt 195"]
    #[inline(always)]
    pub fn int195(&self) -> INT195_R {
        INT195_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Interrupt 196"]
    #[inline(always)]
    pub fn int196(&self) -> INT196_R {
        INT196_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Interrupt 197"]
    #[inline(always)]
    pub fn int197(&self) -> INT197_R {
        INT197_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Interrupt 198"]
    #[inline(always)]
    pub fn int198(&self) -> INT198_R {
        INT198_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Interrupt 199"]
    #[inline(always)]
    pub fn int199(&self) -> INT199_R {
        INT199_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Interrupt 200"]
    #[inline(always)]
    pub fn int200(&self) -> INT200_R {
        INT200_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Interrupt 201"]
    #[inline(always)]
    pub fn int201(&self) -> INT201_R {
        INT201_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Interrupt 202"]
    #[inline(always)]
    pub fn int202(&self) -> INT202_R {
        INT202_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Interrupt 203"]
    #[inline(always)]
    pub fn int203(&self) -> INT203_R {
        INT203_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Interrupt 204"]
    #[inline(always)]
    pub fn int204(&self) -> INT204_R {
        INT204_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Interrupt 205"]
    #[inline(always)]
    pub fn int205(&self) -> INT205_R {
        INT205_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Interrupt 206"]
    #[inline(always)]
    pub fn int206(&self) -> INT206_R {
        INT206_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Interrupt 207"]
    #[inline(always)]
    pub fn int207(&self) -> INT207_R {
        INT207_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Interrupt 208"]
    #[inline(always)]
    pub fn int208(&self) -> INT208_R {
        INT208_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Interrupt 209"]
    #[inline(always)]
    pub fn int209(&self) -> INT209_R {
        INT209_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Interrupt 210"]
    #[inline(always)]
    pub fn int210(&self) -> INT210_R {
        INT210_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Interrupt 211"]
    #[inline(always)]
    pub fn int211(&self) -> INT211_R {
        INT211_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Interrupt 212"]
    #[inline(always)]
    pub fn int212(&self) -> INT212_R {
        INT212_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Interrupt 213"]
    #[inline(always)]
    pub fn int213(&self) -> INT213_R {
        INT213_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Interrupt 214"]
    #[inline(always)]
    pub fn int214(&self) -> INT214_R {
        INT214_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Interrupt 215"]
    #[inline(always)]
    pub fn int215(&self) -> INT215_R {
        INT215_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Interrupt 216"]
    #[inline(always)]
    pub fn int216(&self) -> INT216_R {
        INT216_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Interrupt 217"]
    #[inline(always)]
    pub fn int217(&self) -> INT217_R {
        INT217_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Interrupt 218"]
    #[inline(always)]
    pub fn int218(&self) -> INT218_R {
        INT218_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Interrupt 219"]
    #[inline(always)]
    pub fn int219(&self) -> INT219_R {
        INT219_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Interrupt 220"]
    #[inline(always)]
    pub fn int220(&self) -> INT220_R {
        INT220_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Interrupt 221"]
    #[inline(always)]
    pub fn int221(&self) -> INT221_R {
        INT221_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Interrupt 222"]
    #[inline(always)]
    pub fn int222(&self) -> INT222_R {
        INT222_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Interrupt 223"]
    #[inline(always)]
    pub fn int223(&self) -> INT223_R {
        INT223_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt 192"]
    #[inline(always)]
    #[must_use]
    pub fn int192(&mut self) -> INT192_W<0> {
        INT192_W::new(self)
    }
    #[doc = "Bit 1 - Interrupt 193"]
    #[inline(always)]
    #[must_use]
    pub fn int193(&mut self) -> INT193_W<1> {
        INT193_W::new(self)
    }
    #[doc = "Bit 2 - Interrupt 194"]
    #[inline(always)]
    #[must_use]
    pub fn int194(&mut self) -> INT194_W<2> {
        INT194_W::new(self)
    }
    #[doc = "Bit 3 - Interrupt 195"]
    #[inline(always)]
    #[must_use]
    pub fn int195(&mut self) -> INT195_W<3> {
        INT195_W::new(self)
    }
    #[doc = "Bit 4 - Interrupt 196"]
    #[inline(always)]
    #[must_use]
    pub fn int196(&mut self) -> INT196_W<4> {
        INT196_W::new(self)
    }
    #[doc = "Bit 5 - Interrupt 197"]
    #[inline(always)]
    #[must_use]
    pub fn int197(&mut self) -> INT197_W<5> {
        INT197_W::new(self)
    }
    #[doc = "Bit 6 - Interrupt 198"]
    #[inline(always)]
    #[must_use]
    pub fn int198(&mut self) -> INT198_W<6> {
        INT198_W::new(self)
    }
    #[doc = "Bit 7 - Interrupt 199"]
    #[inline(always)]
    #[must_use]
    pub fn int199(&mut self) -> INT199_W<7> {
        INT199_W::new(self)
    }
    #[doc = "Bit 8 - Interrupt 200"]
    #[inline(always)]
    #[must_use]
    pub fn int200(&mut self) -> INT200_W<8> {
        INT200_W::new(self)
    }
    #[doc = "Bit 9 - Interrupt 201"]
    #[inline(always)]
    #[must_use]
    pub fn int201(&mut self) -> INT201_W<9> {
        INT201_W::new(self)
    }
    #[doc = "Bit 10 - Interrupt 202"]
    #[inline(always)]
    #[must_use]
    pub fn int202(&mut self) -> INT202_W<10> {
        INT202_W::new(self)
    }
    #[doc = "Bit 11 - Interrupt 203"]
    #[inline(always)]
    #[must_use]
    pub fn int203(&mut self) -> INT203_W<11> {
        INT203_W::new(self)
    }
    #[doc = "Bit 12 - Interrupt 204"]
    #[inline(always)]
    #[must_use]
    pub fn int204(&mut self) -> INT204_W<12> {
        INT204_W::new(self)
    }
    #[doc = "Bit 13 - Interrupt 205"]
    #[inline(always)]
    #[must_use]
    pub fn int205(&mut self) -> INT205_W<13> {
        INT205_W::new(self)
    }
    #[doc = "Bit 14 - Interrupt 206"]
    #[inline(always)]
    #[must_use]
    pub fn int206(&mut self) -> INT206_W<14> {
        INT206_W::new(self)
    }
    #[doc = "Bit 15 - Interrupt 207"]
    #[inline(always)]
    #[must_use]
    pub fn int207(&mut self) -> INT207_W<15> {
        INT207_W::new(self)
    }
    #[doc = "Bit 16 - Interrupt 208"]
    #[inline(always)]
    #[must_use]
    pub fn int208(&mut self) -> INT208_W<16> {
        INT208_W::new(self)
    }
    #[doc = "Bit 17 - Interrupt 209"]
    #[inline(always)]
    #[must_use]
    pub fn int209(&mut self) -> INT209_W<17> {
        INT209_W::new(self)
    }
    #[doc = "Bit 18 - Interrupt 210"]
    #[inline(always)]
    #[must_use]
    pub fn int210(&mut self) -> INT210_W<18> {
        INT210_W::new(self)
    }
    #[doc = "Bit 19 - Interrupt 211"]
    #[inline(always)]
    #[must_use]
    pub fn int211(&mut self) -> INT211_W<19> {
        INT211_W::new(self)
    }
    #[doc = "Bit 20 - Interrupt 212"]
    #[inline(always)]
    #[must_use]
    pub fn int212(&mut self) -> INT212_W<20> {
        INT212_W::new(self)
    }
    #[doc = "Bit 21 - Interrupt 213"]
    #[inline(always)]
    #[must_use]
    pub fn int213(&mut self) -> INT213_W<21> {
        INT213_W::new(self)
    }
    #[doc = "Bit 22 - Interrupt 214"]
    #[inline(always)]
    #[must_use]
    pub fn int214(&mut self) -> INT214_W<22> {
        INT214_W::new(self)
    }
    #[doc = "Bit 23 - Interrupt 215"]
    #[inline(always)]
    #[must_use]
    pub fn int215(&mut self) -> INT215_W<23> {
        INT215_W::new(self)
    }
    #[doc = "Bit 24 - Interrupt 216"]
    #[inline(always)]
    #[must_use]
    pub fn int216(&mut self) -> INT216_W<24> {
        INT216_W::new(self)
    }
    #[doc = "Bit 25 - Interrupt 217"]
    #[inline(always)]
    #[must_use]
    pub fn int217(&mut self) -> INT217_W<25> {
        INT217_W::new(self)
    }
    #[doc = "Bit 26 - Interrupt 218"]
    #[inline(always)]
    #[must_use]
    pub fn int218(&mut self) -> INT218_W<26> {
        INT218_W::new(self)
    }
    #[doc = "Bit 27 - Interrupt 219"]
    #[inline(always)]
    #[must_use]
    pub fn int219(&mut self) -> INT219_W<27> {
        INT219_W::new(self)
    }
    #[doc = "Bit 28 - Interrupt 220"]
    #[inline(always)]
    #[must_use]
    pub fn int220(&mut self) -> INT220_W<28> {
        INT220_W::new(self)
    }
    #[doc = "Bit 29 - Interrupt 221"]
    #[inline(always)]
    #[must_use]
    pub fn int221(&mut self) -> INT221_W<29> {
        INT221_W::new(self)
    }
    #[doc = "Bit 30 - Interrupt 222"]
    #[inline(always)]
    #[must_use]
    pub fn int222(&mut self) -> INT222_W<30> {
        INT222_W::new(self)
    }
    #[doc = "Bit 31 - Interrupt 223"]
    #[inline(always)]
    #[must_use]
    pub fn int223(&mut self) -> INT223_W<31> {
        INT223_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Set-Active\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_isactiver6](index.html) module"]
pub struct GICD_ISACTIVER6_SPEC;
impl crate::RegisterSpec for GICD_ISACTIVER6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gicd_isactiver6::R](R) reader structure"]
impl crate::Readable for GICD_ISACTIVER6_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gicd_isactiver6::W](W) writer structure"]
impl crate::Writable for GICD_ISACTIVER6_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0xffff_ffff;
}
#[doc = "`reset()` method sets GICD_ISACTIVER6 to value 0"]
impl crate::Resettable for GICD_ISACTIVER6_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
