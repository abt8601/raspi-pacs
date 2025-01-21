#[doc = "Register `C` reader"]
pub type R = crate::R<C_SPEC>;
#[doc = "Register `C` writer"]
pub type W = crate::W<C_SPEC>;
#[doc = "Field `READ` reader - Transfer is read"]
pub type READ_R = crate::BitReader;
#[doc = "Field `READ` writer - Transfer is read"]
pub type READ_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLEAR` reader - Clear the FIFO"]
pub type CLEAR_R = crate::FieldReader;
#[doc = "Field `CLEAR` writer - Clear the FIFO"]
pub type CLEAR_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ST` reader - Start transfer"]
pub type ST_R = crate::BitReader;
#[doc = "Field `ST` writer - Start transfer"]
pub type ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTD` reader - Interrupt on done"]
pub type INTD_R = crate::BitReader;
#[doc = "Field `INTD` writer - Interrupt on done"]
pub type INTD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTT` reader - Interrupt on TX"]
pub type INTT_R = crate::BitReader;
#[doc = "Field `INTT` writer - Interrupt on TX"]
pub type INTT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTR` reader - Interrupt on RX"]
pub type INTR_R = crate::BitReader;
#[doc = "Field `INTR` writer - Interrupt on RX"]
pub type INTR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2CEN` reader - I2C Enable"]
pub type I2CEN_R = crate::BitReader;
#[doc = "Field `I2CEN` writer - I2C Enable"]
pub type I2CEN_W<'a, REG> = crate::BitWriter<'a, REG>;
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
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("C")
            .field("i2cen", &format_args!("{}", self.i2cen().bit()))
            .field("intr", &format_args!("{}", self.intr().bit()))
            .field("intt", &format_args!("{}", self.intt().bit()))
            .field("intd", &format_args!("{}", self.intd().bit()))
            .field("st", &format_args!("{}", self.st().bit()))
            .field("clear", &format_args!("{}", self.clear().bits()))
            .field("read", &format_args!("{}", self.read().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<C_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - Transfer is read"]
    #[inline(always)]
    #[must_use]
    pub fn read(&mut self) -> READ_W<C_SPEC> {
        READ_W::new(self, 0)
    }
    #[doc = "Bits 4:5 - Clear the FIFO"]
    #[inline(always)]
    #[must_use]
    pub fn clear(&mut self) -> CLEAR_W<C_SPEC> {
        CLEAR_W::new(self, 4)
    }
    #[doc = "Bit 7 - Start transfer"]
    #[inline(always)]
    #[must_use]
    pub fn st(&mut self) -> ST_W<C_SPEC> {
        ST_W::new(self, 7)
    }
    #[doc = "Bit 8 - Interrupt on done"]
    #[inline(always)]
    #[must_use]
    pub fn intd(&mut self) -> INTD_W<C_SPEC> {
        INTD_W::new(self, 8)
    }
    #[doc = "Bit 9 - Interrupt on TX"]
    #[inline(always)]
    #[must_use]
    pub fn intt(&mut self) -> INTT_W<C_SPEC> {
        INTT_W::new(self, 9)
    }
    #[doc = "Bit 10 - Interrupt on RX"]
    #[inline(always)]
    #[must_use]
    pub fn intr(&mut self) -> INTR_W<C_SPEC> {
        INTR_W::new(self, 10)
    }
    #[doc = "Bit 15 - I2C Enable"]
    #[inline(always)]
    #[must_use]
    pub fn i2cen(&mut self) -> I2CEN_W<C_SPEC> {
        I2CEN_W::new(self, 15)
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
#[doc = "Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct C_SPEC;
impl crate::RegisterSpec for C_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`c::R`](R) reader structure"]
impl crate::Readable for C_SPEC {}
#[doc = "`write(|w| ..)` method takes [`c::W`](W) writer structure"]
impl crate::Writable for C_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets C to value 0"]
impl crate::Resettable for C_SPEC {
    const RESET_VALUE: u32 = 0;
}
