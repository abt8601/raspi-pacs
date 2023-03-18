#[doc = "Register `C` reader"]
pub struct R(crate::R<C_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<C_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<C_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<C_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `C` writer"]
pub struct W(crate::W<C_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<C_SPEC>;
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
impl From<crate::W<C_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<C_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `READ` reader - Transfer is read"]
pub type READ_R = crate::BitReader<bool>;
#[doc = "Field `READ` writer - Transfer is read"]
pub type READ_W<'a, const O: u8> = crate::BitWriter<'a, u32, C_SPEC, bool, O>;
#[doc = "Field `CLEAR` reader - Clear the FIFO"]
pub type CLEAR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CLEAR` writer - Clear the FIFO"]
pub type CLEAR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, C_SPEC, u8, u8, 2, O>;
#[doc = "Field `ST` reader - Start transfer"]
pub type ST_R = crate::BitReader<bool>;
#[doc = "Field `ST` writer - Start transfer"]
pub type ST_W<'a, const O: u8> = crate::BitWriter<'a, u32, C_SPEC, bool, O>;
#[doc = "Field `INTD` reader - Interrupt on done"]
pub type INTD_R = crate::BitReader<bool>;
#[doc = "Field `INTD` writer - Interrupt on done"]
pub type INTD_W<'a, const O: u8> = crate::BitWriter<'a, u32, C_SPEC, bool, O>;
#[doc = "Field `INTT` reader - Interrupt on TX"]
pub type INTT_R = crate::BitReader<bool>;
#[doc = "Field `INTT` writer - Interrupt on TX"]
pub type INTT_W<'a, const O: u8> = crate::BitWriter<'a, u32, C_SPEC, bool, O>;
#[doc = "Field `INTR` reader - Interrupt on RX"]
pub type INTR_R = crate::BitReader<bool>;
#[doc = "Field `INTR` writer - Interrupt on RX"]
pub type INTR_W<'a, const O: u8> = crate::BitWriter<'a, u32, C_SPEC, bool, O>;
#[doc = "Field `I2CEN` reader - I2C Enable"]
pub type I2CEN_R = crate::BitReader<bool>;
#[doc = "Field `I2CEN` writer - I2C Enable"]
pub type I2CEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, C_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Transfer is read"]
    #[inline(always)]
    pub fn read(&self) -> READ_R {
        READ_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:5 - Clear the FIFO"]
    #[inline(always)]
    pub fn clear(&self) -> CLEAR_R {
        CLEAR_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 7 - Start transfer"]
    #[inline(always)]
    pub fn st(&self) -> ST_R {
        ST_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Interrupt on done"]
    #[inline(always)]
    pub fn intd(&self) -> INTD_R {
        INTD_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Interrupt on TX"]
    #[inline(always)]
    pub fn intt(&self) -> INTT_R {
        INTT_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Interrupt on RX"]
    #[inline(always)]
    pub fn intr(&self) -> INTR_R {
        INTR_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 15 - I2C Enable"]
    #[inline(always)]
    pub fn i2cen(&self) -> I2CEN_R {
        I2CEN_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transfer is read"]
    #[inline(always)]
    #[must_use]
    pub fn read(&mut self) -> READ_W<0> {
        READ_W::new(self)
    }
    #[doc = "Bits 4:5 - Clear the FIFO"]
    #[inline(always)]
    #[must_use]
    pub fn clear(&mut self) -> CLEAR_W<4> {
        CLEAR_W::new(self)
    }
    #[doc = "Bit 7 - Start transfer"]
    #[inline(always)]
    #[must_use]
    pub fn st(&mut self) -> ST_W<7> {
        ST_W::new(self)
    }
    #[doc = "Bit 8 - Interrupt on done"]
    #[inline(always)]
    #[must_use]
    pub fn intd(&mut self) -> INTD_W<8> {
        INTD_W::new(self)
    }
    #[doc = "Bit 9 - Interrupt on TX"]
    #[inline(always)]
    #[must_use]
    pub fn intt(&mut self) -> INTT_W<9> {
        INTT_W::new(self)
    }
    #[doc = "Bit 10 - Interrupt on RX"]
    #[inline(always)]
    #[must_use]
    pub fn intr(&mut self) -> INTR_W<10> {
        INTR_W::new(self)
    }
    #[doc = "Bit 15 - I2C Enable"]
    #[inline(always)]
    #[must_use]
    pub fn i2cen(&mut self) -> I2CEN_W<15> {
        I2CEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c](index.html) module"]
pub struct C_SPEC;
impl crate::RegisterSpec for C_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [c::R](R) reader structure"]
impl crate::Readable for C_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [c::W](W) writer structure"]
impl crate::Writable for C_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets C to value 0"]
impl crate::Resettable for C_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
