#[doc = "Register `DR` reader"]
pub type R = crate::R<DR_SPEC>;
#[doc = "Register `DR` writer"]
pub type W = crate::W<DR_SPEC>;
#[doc = "Field `DATA` reader - DATA"]
pub type DATA_R = crate::FieldReader;
#[doc = "Field `DATA` writer - DATA"]
pub type DATA_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `FE` reader - FE"]
pub type FE_R = crate::BitReader;
#[doc = "Field `FE` writer - FE"]
pub type FE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PE` reader - PE"]
pub type PE_R = crate::BitReader;
#[doc = "Field `PE` writer - PE"]
pub type PE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BE` reader - BE"]
pub type BE_R = crate::BitReader;
#[doc = "Field `BE` writer - BE"]
pub type BE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OE` reader - OE"]
pub type OE_R = crate::BitReader;
#[doc = "Field `OE` writer - OE"]
pub type OE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - DATA"]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - FE"]
    #[inline(always)]
    pub fn fe(&self) -> FE_R {
        FE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - PE"]
    #[inline(always)]
    pub fn pe(&self) -> PE_R {
        PE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - BE"]
    #[inline(always)]
    pub fn be(&self) -> BE_R {
        BE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - OE"]
    #[inline(always)]
    pub fn oe(&self) -> OE_R {
        OE_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DR")
            .field("data", &format_args!("{}", self.data().bits()))
            .field("fe", &format_args!("{}", self.fe().bit()))
            .field("pe", &format_args!("{}", self.pe().bit()))
            .field("be", &format_args!("{}", self.be().bit()))
            .field("oe", &format_args!("{}", self.oe().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<DR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:7 - DATA"]
    #[inline(always)]
    #[must_use]
    pub fn data(&mut self) -> DATA_W<DR_SPEC> {
        DATA_W::new(self, 0)
    }
    #[doc = "Bit 8 - FE"]
    #[inline(always)]
    #[must_use]
    pub fn fe(&mut self) -> FE_W<DR_SPEC> {
        FE_W::new(self, 8)
    }
    #[doc = "Bit 9 - PE"]
    #[inline(always)]
    #[must_use]
    pub fn pe(&mut self) -> PE_W<DR_SPEC> {
        PE_W::new(self, 9)
    }
    #[doc = "Bit 10 - BE"]
    #[inline(always)]
    #[must_use]
    pub fn be(&mut self) -> BE_W<DR_SPEC> {
        BE_W::new(self, 10)
    }
    #[doc = "Bit 11 - OE"]
    #[inline(always)]
    #[must_use]
    pub fn oe(&mut self) -> OE_W<DR_SPEC> {
        OE_W::new(self, 11)
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
#[doc = "Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DR_SPEC;
impl crate::RegisterSpec for DR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dr::R`](R) reader structure"]
impl crate::Readable for DR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dr::W`](W) writer structure"]
impl crate::Writable for DR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DR to value 0"]
impl crate::Resettable for DR_SPEC {
    const RESET_VALUE: u32 = 0;
}
