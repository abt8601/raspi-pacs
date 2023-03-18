#[doc = "Register `HW_DIRECTION` reader"]
pub struct R(crate::R<HW_DIRECTION_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HW_DIRECTION_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HW_DIRECTION_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HW_DIRECTION_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DIRECTION[0-15]` reader - Direction %s"]
pub type DIRECTION_R = crate::FieldReader<u8, DIRECTION_A>;
#[doc = "Direction %s"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DIRECTION_A {
    #[doc = "0: `0`"]
    BIDIR = 0,
    #[doc = "1: `1`"]
    IN = 1,
    #[doc = "2: `10`"]
    OUT = 2,
}
impl From<DIRECTION_A> for u8 {
    #[inline(always)]
    fn from(variant: DIRECTION_A) -> Self {
        variant as _
    }
}
impl DIRECTION_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DIRECTION_A> {
        match self.bits {
            0 => Some(DIRECTION_A::BIDIR),
            1 => Some(DIRECTION_A::IN),
            2 => Some(DIRECTION_A::OUT),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `BIDIR`"]
    #[inline(always)]
    pub fn is_bidir(&self) -> bool {
        *self == DIRECTION_A::BIDIR
    }
    #[doc = "Checks if the value of the field is `IN`"]
    #[inline(always)]
    pub fn is_in(&self) -> bool {
        *self == DIRECTION_A::IN
    }
    #[doc = "Checks if the value of the field is `OUT`"]
    #[inline(always)]
    pub fn is_out(&self) -> bool {
        *self == DIRECTION_A::OUT
    }
}
impl R {
    #[doc = "Direction [0-15]"]
    #[inline(always)]
    pub unsafe fn direction(&self, n: u8) -> DIRECTION_R {
        DIRECTION_R::new(((self.bits >> (n * 2)) & 3) as u8)
    }
    #[doc = "Bits 0:1 - Direction 0"]
    #[inline(always)]
    pub fn direction0(&self) -> DIRECTION_R {
        DIRECTION_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Direction 1"]
    #[inline(always)]
    pub fn direction1(&self) -> DIRECTION_R {
        DIRECTION_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Direction 2"]
    #[inline(always)]
    pub fn direction2(&self) -> DIRECTION_R {
        DIRECTION_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Direction 3"]
    #[inline(always)]
    pub fn direction3(&self) -> DIRECTION_R {
        DIRECTION_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Direction 4"]
    #[inline(always)]
    pub fn direction4(&self) -> DIRECTION_R {
        DIRECTION_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Direction 5"]
    #[inline(always)]
    pub fn direction5(&self) -> DIRECTION_R {
        DIRECTION_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Direction 6"]
    #[inline(always)]
    pub fn direction6(&self) -> DIRECTION_R {
        DIRECTION_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Direction 7"]
    #[inline(always)]
    pub fn direction7(&self) -> DIRECTION_R {
        DIRECTION_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Direction 8"]
    #[inline(always)]
    pub fn direction8(&self) -> DIRECTION_R {
        DIRECTION_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Direction 9"]
    #[inline(always)]
    pub fn direction9(&self) -> DIRECTION_R {
        DIRECTION_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Direction 10"]
    #[inline(always)]
    pub fn direction10(&self) -> DIRECTION_R {
        DIRECTION_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - Direction 11"]
    #[inline(always)]
    pub fn direction11(&self) -> DIRECTION_R {
        DIRECTION_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - Direction 12"]
    #[inline(always)]
    pub fn direction12(&self) -> DIRECTION_R {
        DIRECTION_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - Direction 13"]
    #[inline(always)]
    pub fn direction13(&self) -> DIRECTION_R {
        DIRECTION_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - Direction 14"]
    #[inline(always)]
    pub fn direction14(&self) -> DIRECTION_R {
        DIRECTION_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - Direction 15"]
    #[inline(always)]
    pub fn direction15(&self) -> DIRECTION_R {
        DIRECTION_R::new(((self.bits >> 30) & 3) as u8)
    }
}
#[doc = "Direction\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hw_direction](index.html) module"]
pub struct HW_DIRECTION_SPEC;
impl crate::RegisterSpec for HW_DIRECTION_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hw_direction::R](R) reader structure"]
impl crate::Readable for HW_DIRECTION_SPEC {
    type Reader = R;
}
