#[doc = "Register `GICD_SPISR1` reader"]
pub type R = crate::R<GICD_SPISR1_SPEC>;
#[doc = "Register `GICD_SPISR1` writer"]
pub type W = crate::W<GICD_SPISR1_SPEC>;
#[doc = "Field `TIMER` reader - ARMC Timer"]
pub type TIMER_R = crate::BitReader;
#[doc = "Field `TIMER` writer - ARMC Timer"]
pub type TIMER_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MAILBOX` reader - Mailbox"]
pub type MAILBOX_R = crate::BitReader;
#[doc = "Field `MAILBOX` writer - Mailbox"]
pub type MAILBOX_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DOORBELL0` reader - Doorbell 0"]
pub type DOORBELL0_R = crate::BitReader;
#[doc = "Field `DOORBELL0` writer - Doorbell 0"]
pub type DOORBELL0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DOORBELL1` reader - Doorbell 1"]
pub type DOORBELL1_R = crate::BitReader;
#[doc = "Field `DOORBELL1` writer - Doorbell 1"]
pub type DOORBELL1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VPU0_HALTED` reader - VPU0 halted"]
pub type VPU0_HALTED_R = crate::BitReader;
#[doc = "Field `VPU0_HALTED` writer - VPU0 halted"]
pub type VPU0_HALTED_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VPU1_HALTED` reader - VPU1 halted"]
pub type VPU1_HALTED_R = crate::BitReader;
#[doc = "Field `VPU1_HALTED` writer - VPU1 halted"]
pub type VPU1_HALTED_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARM_ADDRESS_ERROR` reader - ARM address error"]
pub type ARM_ADDRESS_ERROR_R = crate::BitReader;
#[doc = "Field `ARM_ADDRESS_ERROR` writer - ARM address error"]
pub type ARM_ADDRESS_ERROR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARM_AXI_ERROR` reader - ARM AXI error"]
pub type ARM_AXI_ERROR_R = crate::BitReader;
#[doc = "Field `ARM_AXI_ERROR` writer - ARM AXI error"]
pub type ARM_AXI_ERROR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWI0` reader - Software interrupt 0"]
pub type SWI0_R = crate::BitReader;
#[doc = "Field `SWI0` writer - Software interrupt 0"]
pub type SWI0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWI1` reader - Software interrupt 1"]
pub type SWI1_R = crate::BitReader;
#[doc = "Field `SWI1` writer - Software interrupt 1"]
pub type SWI1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWI2` reader - Software interrupt 2"]
pub type SWI2_R = crate::BitReader;
#[doc = "Field `SWI2` writer - Software interrupt 2"]
pub type SWI2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWI3` reader - Software interrupt 3"]
pub type SWI3_R = crate::BitReader;
#[doc = "Field `SWI3` writer - Software interrupt 3"]
pub type SWI3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWI4` reader - Software interrupt 4"]
pub type SWI4_R = crate::BitReader;
#[doc = "Field `SWI4` writer - Software interrupt 4"]
pub type SWI4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWI5` reader - Software interrupt 5"]
pub type SWI5_R = crate::BitReader;
#[doc = "Field `SWI5` writer - Software interrupt 5"]
pub type SWI5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWI6` reader - Software interrupt 6"]
pub type SWI6_R = crate::BitReader;
#[doc = "Field `SWI6` writer - Software interrupt 6"]
pub type SWI6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWI7` reader - Software interrupt 7"]
pub type SWI7_R = crate::BitReader;
#[doc = "Field `SWI7` writer - Software interrupt 7"]
pub type SWI7_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI80` reader - Shared interrupt 80"]
pub type SPI80_R = crate::BitReader;
#[doc = "Field `SPI80` writer - Shared interrupt 80"]
pub type SPI80_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI81` reader - Shared interrupt 81"]
pub type SPI81_R = crate::BitReader;
#[doc = "Field `SPI81` writer - Shared interrupt 81"]
pub type SPI81_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI82` reader - Shared interrupt 82"]
pub type SPI82_R = crate::BitReader;
#[doc = "Field `SPI82` writer - Shared interrupt 82"]
pub type SPI82_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI83` reader - Shared interrupt 83"]
pub type SPI83_R = crate::BitReader;
#[doc = "Field `SPI83` writer - Shared interrupt 83"]
pub type SPI83_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI84` reader - Shared interrupt 84"]
pub type SPI84_R = crate::BitReader;
#[doc = "Field `SPI84` writer - Shared interrupt 84"]
pub type SPI84_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI85` reader - Shared interrupt 85"]
pub type SPI85_R = crate::BitReader;
#[doc = "Field `SPI85` writer - Shared interrupt 85"]
pub type SPI85_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI86` reader - Shared interrupt 86"]
pub type SPI86_R = crate::BitReader;
#[doc = "Field `SPI86` writer - Shared interrupt 86"]
pub type SPI86_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI87` reader - Shared interrupt 87"]
pub type SPI87_R = crate::BitReader;
#[doc = "Field `SPI87` writer - Shared interrupt 87"]
pub type SPI87_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI88` reader - Shared interrupt 88"]
pub type SPI88_R = crate::BitReader;
#[doc = "Field `SPI88` writer - Shared interrupt 88"]
pub type SPI88_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI89` reader - Shared interrupt 89"]
pub type SPI89_R = crate::BitReader;
#[doc = "Field `SPI89` writer - Shared interrupt 89"]
pub type SPI89_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI90` reader - Shared interrupt 90"]
pub type SPI90_R = crate::BitReader;
#[doc = "Field `SPI90` writer - Shared interrupt 90"]
pub type SPI90_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI91` reader - Shared interrupt 91"]
pub type SPI91_R = crate::BitReader;
#[doc = "Field `SPI91` writer - Shared interrupt 91"]
pub type SPI91_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI92` reader - Shared interrupt 92"]
pub type SPI92_R = crate::BitReader;
#[doc = "Field `SPI92` writer - Shared interrupt 92"]
pub type SPI92_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI93` reader - Shared interrupt 93"]
pub type SPI93_R = crate::BitReader;
#[doc = "Field `SPI93` writer - Shared interrupt 93"]
pub type SPI93_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI94` reader - Shared interrupt 94"]
pub type SPI94_R = crate::BitReader;
#[doc = "Field `SPI94` writer - Shared interrupt 94"]
pub type SPI94_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI95` reader - Shared interrupt 95"]
pub type SPI95_R = crate::BitReader;
#[doc = "Field `SPI95` writer - Shared interrupt 95"]
pub type SPI95_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - ARMC Timer"]
    #[inline(always)]
    pub fn timer(&self) -> TIMER_R {
        TIMER_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Mailbox"]
    #[inline(always)]
    pub fn mailbox(&self) -> MAILBOX_R {
        MAILBOX_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Doorbell 0"]
    #[inline(always)]
    pub fn doorbell0(&self) -> DOORBELL0_R {
        DOORBELL0_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Doorbell 1"]
    #[inline(always)]
    pub fn doorbell1(&self) -> DOORBELL1_R {
        DOORBELL1_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - VPU0 halted"]
    #[inline(always)]
    pub fn vpu0_halted(&self) -> VPU0_HALTED_R {
        VPU0_HALTED_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - VPU1 halted"]
    #[inline(always)]
    pub fn vpu1_halted(&self) -> VPU1_HALTED_R {
        VPU1_HALTED_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - ARM address error"]
    #[inline(always)]
    pub fn arm_address_error(&self) -> ARM_ADDRESS_ERROR_R {
        ARM_ADDRESS_ERROR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - ARM AXI error"]
    #[inline(always)]
    pub fn arm_axi_error(&self) -> ARM_AXI_ERROR_R {
        ARM_AXI_ERROR_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Software interrupt 0"]
    #[inline(always)]
    pub fn swi0(&self) -> SWI0_R {
        SWI0_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Software interrupt 1"]
    #[inline(always)]
    pub fn swi1(&self) -> SWI1_R {
        SWI1_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Software interrupt 2"]
    #[inline(always)]
    pub fn swi2(&self) -> SWI2_R {
        SWI2_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Software interrupt 3"]
    #[inline(always)]
    pub fn swi3(&self) -> SWI3_R {
        SWI3_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Software interrupt 4"]
    #[inline(always)]
    pub fn swi4(&self) -> SWI4_R {
        SWI4_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Software interrupt 5"]
    #[inline(always)]
    pub fn swi5(&self) -> SWI5_R {
        SWI5_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Software interrupt 6"]
    #[inline(always)]
    pub fn swi6(&self) -> SWI6_R {
        SWI6_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Software interrupt 7"]
    #[inline(always)]
    pub fn swi7(&self) -> SWI7_R {
        SWI7_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Shared interrupt 80"]
    #[inline(always)]
    pub fn spi80(&self) -> SPI80_R {
        SPI80_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Shared interrupt 81"]
    #[inline(always)]
    pub fn spi81(&self) -> SPI81_R {
        SPI81_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Shared interrupt 82"]
    #[inline(always)]
    pub fn spi82(&self) -> SPI82_R {
        SPI82_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Shared interrupt 83"]
    #[inline(always)]
    pub fn spi83(&self) -> SPI83_R {
        SPI83_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Shared interrupt 84"]
    #[inline(always)]
    pub fn spi84(&self) -> SPI84_R {
        SPI84_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Shared interrupt 85"]
    #[inline(always)]
    pub fn spi85(&self) -> SPI85_R {
        SPI85_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Shared interrupt 86"]
    #[inline(always)]
    pub fn spi86(&self) -> SPI86_R {
        SPI86_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Shared interrupt 87"]
    #[inline(always)]
    pub fn spi87(&self) -> SPI87_R {
        SPI87_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Shared interrupt 88"]
    #[inline(always)]
    pub fn spi88(&self) -> SPI88_R {
        SPI88_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Shared interrupt 89"]
    #[inline(always)]
    pub fn spi89(&self) -> SPI89_R {
        SPI89_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Shared interrupt 90"]
    #[inline(always)]
    pub fn spi90(&self) -> SPI90_R {
        SPI90_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Shared interrupt 91"]
    #[inline(always)]
    pub fn spi91(&self) -> SPI91_R {
        SPI91_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Shared interrupt 92"]
    #[inline(always)]
    pub fn spi92(&self) -> SPI92_R {
        SPI92_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Shared interrupt 93"]
    #[inline(always)]
    pub fn spi93(&self) -> SPI93_R {
        SPI93_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Shared interrupt 94"]
    #[inline(always)]
    pub fn spi94(&self) -> SPI94_R {
        SPI94_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Shared interrupt 95"]
    #[inline(always)]
    pub fn spi95(&self) -> SPI95_R {
        SPI95_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICD_SPISR1")
            .field("timer", &format_args!("{}", self.timer().bit()))
            .field("mailbox", &format_args!("{}", self.mailbox().bit()))
            .field("doorbell0", &format_args!("{}", self.doorbell0().bit()))
            .field("doorbell1", &format_args!("{}", self.doorbell1().bit()))
            .field("vpu0_halted", &format_args!("{}", self.vpu0_halted().bit()))
            .field("vpu1_halted", &format_args!("{}", self.vpu1_halted().bit()))
            .field(
                "arm_address_error",
                &format_args!("{}", self.arm_address_error().bit()),
            )
            .field(
                "arm_axi_error",
                &format_args!("{}", self.arm_axi_error().bit()),
            )
            .field("swi0", &format_args!("{}", self.swi0().bit()))
            .field("swi1", &format_args!("{}", self.swi1().bit()))
            .field("swi2", &format_args!("{}", self.swi2().bit()))
            .field("swi3", &format_args!("{}", self.swi3().bit()))
            .field("swi4", &format_args!("{}", self.swi4().bit()))
            .field("swi5", &format_args!("{}", self.swi5().bit()))
            .field("swi6", &format_args!("{}", self.swi6().bit()))
            .field("swi7", &format_args!("{}", self.swi7().bit()))
            .field("spi80", &format_args!("{}", self.spi80().bit()))
            .field("spi81", &format_args!("{}", self.spi81().bit()))
            .field("spi82", &format_args!("{}", self.spi82().bit()))
            .field("spi83", &format_args!("{}", self.spi83().bit()))
            .field("spi84", &format_args!("{}", self.spi84().bit()))
            .field("spi85", &format_args!("{}", self.spi85().bit()))
            .field("spi86", &format_args!("{}", self.spi86().bit()))
            .field("spi87", &format_args!("{}", self.spi87().bit()))
            .field("spi88", &format_args!("{}", self.spi88().bit()))
            .field("spi89", &format_args!("{}", self.spi89().bit()))
            .field("spi90", &format_args!("{}", self.spi90().bit()))
            .field("spi91", &format_args!("{}", self.spi91().bit()))
            .field("spi92", &format_args!("{}", self.spi92().bit()))
            .field("spi93", &format_args!("{}", self.spi93().bit()))
            .field("spi94", &format_args!("{}", self.spi94().bit()))
            .field("spi95", &format_args!("{}", self.spi95().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<GICD_SPISR1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - ARMC Timer"]
    #[inline(always)]
    #[must_use]
    pub fn timer(&mut self) -> TIMER_W<GICD_SPISR1_SPEC> {
        TIMER_W::new(self, 0)
    }
    #[doc = "Bit 1 - Mailbox"]
    #[inline(always)]
    #[must_use]
    pub fn mailbox(&mut self) -> MAILBOX_W<GICD_SPISR1_SPEC> {
        MAILBOX_W::new(self, 1)
    }
    #[doc = "Bit 2 - Doorbell 0"]
    #[inline(always)]
    #[must_use]
    pub fn doorbell0(&mut self) -> DOORBELL0_W<GICD_SPISR1_SPEC> {
        DOORBELL0_W::new(self, 2)
    }
    #[doc = "Bit 3 - Doorbell 1"]
    #[inline(always)]
    #[must_use]
    pub fn doorbell1(&mut self) -> DOORBELL1_W<GICD_SPISR1_SPEC> {
        DOORBELL1_W::new(self, 3)
    }
    #[doc = "Bit 4 - VPU0 halted"]
    #[inline(always)]
    #[must_use]
    pub fn vpu0_halted(&mut self) -> VPU0_HALTED_W<GICD_SPISR1_SPEC> {
        VPU0_HALTED_W::new(self, 4)
    }
    #[doc = "Bit 5 - VPU1 halted"]
    #[inline(always)]
    #[must_use]
    pub fn vpu1_halted(&mut self) -> VPU1_HALTED_W<GICD_SPISR1_SPEC> {
        VPU1_HALTED_W::new(self, 5)
    }
    #[doc = "Bit 6 - ARM address error"]
    #[inline(always)]
    #[must_use]
    pub fn arm_address_error(&mut self) -> ARM_ADDRESS_ERROR_W<GICD_SPISR1_SPEC> {
        ARM_ADDRESS_ERROR_W::new(self, 6)
    }
    #[doc = "Bit 7 - ARM AXI error"]
    #[inline(always)]
    #[must_use]
    pub fn arm_axi_error(&mut self) -> ARM_AXI_ERROR_W<GICD_SPISR1_SPEC> {
        ARM_AXI_ERROR_W::new(self, 7)
    }
    #[doc = "Bit 8 - Software interrupt 0"]
    #[inline(always)]
    #[must_use]
    pub fn swi0(&mut self) -> SWI0_W<GICD_SPISR1_SPEC> {
        SWI0_W::new(self, 8)
    }
    #[doc = "Bit 9 - Software interrupt 1"]
    #[inline(always)]
    #[must_use]
    pub fn swi1(&mut self) -> SWI1_W<GICD_SPISR1_SPEC> {
        SWI1_W::new(self, 9)
    }
    #[doc = "Bit 10 - Software interrupt 2"]
    #[inline(always)]
    #[must_use]
    pub fn swi2(&mut self) -> SWI2_W<GICD_SPISR1_SPEC> {
        SWI2_W::new(self, 10)
    }
    #[doc = "Bit 11 - Software interrupt 3"]
    #[inline(always)]
    #[must_use]
    pub fn swi3(&mut self) -> SWI3_W<GICD_SPISR1_SPEC> {
        SWI3_W::new(self, 11)
    }
    #[doc = "Bit 12 - Software interrupt 4"]
    #[inline(always)]
    #[must_use]
    pub fn swi4(&mut self) -> SWI4_W<GICD_SPISR1_SPEC> {
        SWI4_W::new(self, 12)
    }
    #[doc = "Bit 13 - Software interrupt 5"]
    #[inline(always)]
    #[must_use]
    pub fn swi5(&mut self) -> SWI5_W<GICD_SPISR1_SPEC> {
        SWI5_W::new(self, 13)
    }
    #[doc = "Bit 14 - Software interrupt 6"]
    #[inline(always)]
    #[must_use]
    pub fn swi6(&mut self) -> SWI6_W<GICD_SPISR1_SPEC> {
        SWI6_W::new(self, 14)
    }
    #[doc = "Bit 15 - Software interrupt 7"]
    #[inline(always)]
    #[must_use]
    pub fn swi7(&mut self) -> SWI7_W<GICD_SPISR1_SPEC> {
        SWI7_W::new(self, 15)
    }
    #[doc = "Bit 16 - Shared interrupt 80"]
    #[inline(always)]
    #[must_use]
    pub fn spi80(&mut self) -> SPI80_W<GICD_SPISR1_SPEC> {
        SPI80_W::new(self, 16)
    }
    #[doc = "Bit 17 - Shared interrupt 81"]
    #[inline(always)]
    #[must_use]
    pub fn spi81(&mut self) -> SPI81_W<GICD_SPISR1_SPEC> {
        SPI81_W::new(self, 17)
    }
    #[doc = "Bit 18 - Shared interrupt 82"]
    #[inline(always)]
    #[must_use]
    pub fn spi82(&mut self) -> SPI82_W<GICD_SPISR1_SPEC> {
        SPI82_W::new(self, 18)
    }
    #[doc = "Bit 19 - Shared interrupt 83"]
    #[inline(always)]
    #[must_use]
    pub fn spi83(&mut self) -> SPI83_W<GICD_SPISR1_SPEC> {
        SPI83_W::new(self, 19)
    }
    #[doc = "Bit 20 - Shared interrupt 84"]
    #[inline(always)]
    #[must_use]
    pub fn spi84(&mut self) -> SPI84_W<GICD_SPISR1_SPEC> {
        SPI84_W::new(self, 20)
    }
    #[doc = "Bit 21 - Shared interrupt 85"]
    #[inline(always)]
    #[must_use]
    pub fn spi85(&mut self) -> SPI85_W<GICD_SPISR1_SPEC> {
        SPI85_W::new(self, 21)
    }
    #[doc = "Bit 22 - Shared interrupt 86"]
    #[inline(always)]
    #[must_use]
    pub fn spi86(&mut self) -> SPI86_W<GICD_SPISR1_SPEC> {
        SPI86_W::new(self, 22)
    }
    #[doc = "Bit 23 - Shared interrupt 87"]
    #[inline(always)]
    #[must_use]
    pub fn spi87(&mut self) -> SPI87_W<GICD_SPISR1_SPEC> {
        SPI87_W::new(self, 23)
    }
    #[doc = "Bit 24 - Shared interrupt 88"]
    #[inline(always)]
    #[must_use]
    pub fn spi88(&mut self) -> SPI88_W<GICD_SPISR1_SPEC> {
        SPI88_W::new(self, 24)
    }
    #[doc = "Bit 25 - Shared interrupt 89"]
    #[inline(always)]
    #[must_use]
    pub fn spi89(&mut self) -> SPI89_W<GICD_SPISR1_SPEC> {
        SPI89_W::new(self, 25)
    }
    #[doc = "Bit 26 - Shared interrupt 90"]
    #[inline(always)]
    #[must_use]
    pub fn spi90(&mut self) -> SPI90_W<GICD_SPISR1_SPEC> {
        SPI90_W::new(self, 26)
    }
    #[doc = "Bit 27 - Shared interrupt 91"]
    #[inline(always)]
    #[must_use]
    pub fn spi91(&mut self) -> SPI91_W<GICD_SPISR1_SPEC> {
        SPI91_W::new(self, 27)
    }
    #[doc = "Bit 28 - Shared interrupt 92"]
    #[inline(always)]
    #[must_use]
    pub fn spi92(&mut self) -> SPI92_W<GICD_SPISR1_SPEC> {
        SPI92_W::new(self, 28)
    }
    #[doc = "Bit 29 - Shared interrupt 93"]
    #[inline(always)]
    #[must_use]
    pub fn spi93(&mut self) -> SPI93_W<GICD_SPISR1_SPEC> {
        SPI93_W::new(self, 29)
    }
    #[doc = "Bit 30 - Shared interrupt 94"]
    #[inline(always)]
    #[must_use]
    pub fn spi94(&mut self) -> SPI94_W<GICD_SPISR1_SPEC> {
        SPI94_W::new(self, 30)
    }
    #[doc = "Bit 31 - Shared interrupt 95"]
    #[inline(always)]
    #[must_use]
    pub fn spi95(&mut self) -> SPI95_W<GICD_SPISR1_SPEC> {
        SPI95_W::new(self, 31)
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
#[doc = "Shared Peripheral Interrupt Status Registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_spisr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_spisr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GICD_SPISR1_SPEC;
impl crate::RegisterSpec for GICD_SPISR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_spisr1::R`](R) reader structure"]
impl crate::Readable for GICD_SPISR1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gicd_spisr1::W`](W) writer structure"]
impl crate::Writable for GICD_SPISR1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GICD_SPISR1 to value 0"]
impl crate::Resettable for GICD_SPISR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
