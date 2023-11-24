#[doc = "Register `GICD_ISPENDR2` reader"]
pub type R = crate::R<GICD_ISPENDR2_SPEC>;
#[doc = "Register `GICD_ISPENDR2` writer"]
pub type W = crate::W<GICD_ISPENDR2_SPEC>;
#[doc = "Field `TIMER` reader - ARMC Timer"]
pub type TIMER_R = crate::BitReader;
#[doc = "Field `TIMER` writer - ARMC Timer"]
pub type TIMER_W<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `MAILBOX` reader - Mailbox"]
pub type MAILBOX_R = crate::BitReader;
#[doc = "Field `MAILBOX` writer - Mailbox"]
pub type MAILBOX_W<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `DOORBELL0` reader - Doorbell 0"]
pub type DOORBELL0_R = crate::BitReader;
#[doc = "Field `DOORBELL0` writer - Doorbell 0"]
pub type DOORBELL0_W<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `DOORBELL1` reader - Doorbell 1"]
pub type DOORBELL1_R = crate::BitReader;
#[doc = "Field `DOORBELL1` writer - Doorbell 1"]
pub type DOORBELL1_W<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `VPU0_HALTED` reader - VPU0 halted"]
pub type VPU0_HALTED_R = crate::BitReader;
#[doc = "Field `VPU0_HALTED` writer - VPU0 halted"]
pub type VPU0_HALTED_W<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `VPU1_HALTED` reader - VPU1 halted"]
pub type VPU1_HALTED_R = crate::BitReader;
#[doc = "Field `VPU1_HALTED` writer - VPU1 halted"]
pub type VPU1_HALTED_W<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `ARM_ADDRESS_ERROR` reader - ARM address error"]
pub type ARM_ADDRESS_ERROR_R = crate::BitReader;
#[doc = "Field `ARM_ADDRESS_ERROR` writer - ARM address error"]
pub type ARM_ADDRESS_ERROR_W<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `ARM_AXI_ERROR` reader - ARM AXI error"]
pub type ARM_AXI_ERROR_R = crate::BitReader;
#[doc = "Field `ARM_AXI_ERROR` writer - ARM AXI error"]
pub type ARM_AXI_ERROR_W<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `SWI0` reader - Software interrupt 0"]
pub type SWI0_R = crate::BitReader;
#[doc = "Field `SWI0` writer - Software interrupt 0"]
pub type SWI0_W<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `SWI1` reader - Software interrupt 1"]
pub type SWI1_R = crate::BitReader;
#[doc = "Field `SWI1` writer - Software interrupt 1"]
pub type SWI1_W<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `SWI2` reader - Software interrupt 2"]
pub type SWI2_R = crate::BitReader;
#[doc = "Field `SWI2` writer - Software interrupt 2"]
pub type SWI2_W<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `SWI3` reader - Software interrupt 3"]
pub type SWI3_R = crate::BitReader;
#[doc = "Field `SWI3` writer - Software interrupt 3"]
pub type SWI3_W<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `SWI4` reader - Software interrupt 4"]
pub type SWI4_R = crate::BitReader;
#[doc = "Field `SWI4` writer - Software interrupt 4"]
pub type SWI4_W<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `SWI5` reader - Software interrupt 5"]
pub type SWI5_R = crate::BitReader;
#[doc = "Field `SWI5` writer - Software interrupt 5"]
pub type SWI5_W<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `SWI6` reader - Software interrupt 6"]
pub type SWI6_R = crate::BitReader;
#[doc = "Field `SWI6` writer - Software interrupt 6"]
pub type SWI6_W<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `SWI7` reader - Software interrupt 7"]
pub type SWI7_R = crate::BitReader;
#[doc = "Field `SWI7` writer - Software interrupt 7"]
pub type SWI7_W<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `INT80` reader - Interrupt 80"]
pub type INT80_R = crate::BitReader;
#[doc = "Field `INT80` writer - Interrupt 80"]
pub type INT80_W<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `INT81` reader - Interrupt 81"]
pub type INT81_R = crate::BitReader;
#[doc = "Field `INT81` writer - Interrupt 81"]
pub type INT81_W<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `INT82` reader - Interrupt 82"]
pub type INT82_R = crate::BitReader;
#[doc = "Field `INT82` writer - Interrupt 82"]
pub type INT82_W<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `INT83` reader - Interrupt 83"]
pub type INT83_R = crate::BitReader;
#[doc = "Field `INT83` writer - Interrupt 83"]
pub type INT83_W<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `INT84` reader - Interrupt 84"]
pub type INT84_R = crate::BitReader;
#[doc = "Field `INT84` writer - Interrupt 84"]
pub type INT84_W<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `INT85` reader - Interrupt 85"]
pub type INT85_R = crate::BitReader;
#[doc = "Field `INT85` writer - Interrupt 85"]
pub type INT85_W<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `INT86` reader - Interrupt 86"]
pub type INT86_R = crate::BitReader;
#[doc = "Field `INT86` writer - Interrupt 86"]
pub type INT86_W<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `INT87` reader - Interrupt 87"]
pub type INT87_R = crate::BitReader;
#[doc = "Field `INT87` writer - Interrupt 87"]
pub type INT87_W<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `INT88` reader - Interrupt 88"]
pub type INT88_R = crate::BitReader;
#[doc = "Field `INT88` writer - Interrupt 88"]
pub type INT88_W<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `INT89` reader - Interrupt 89"]
pub type INT89_R = crate::BitReader;
#[doc = "Field `INT89` writer - Interrupt 89"]
pub type INT89_W<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `INT90` reader - Interrupt 90"]
pub type INT90_R = crate::BitReader;
#[doc = "Field `INT90` writer - Interrupt 90"]
pub type INT90_W<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `INT91` reader - Interrupt 91"]
pub type INT91_R = crate::BitReader;
#[doc = "Field `INT91` writer - Interrupt 91"]
pub type INT91_W<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `INT92` reader - Interrupt 92"]
pub type INT92_R = crate::BitReader;
#[doc = "Field `INT92` writer - Interrupt 92"]
pub type INT92_W<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `INT93` reader - Interrupt 93"]
pub type INT93_R = crate::BitReader;
#[doc = "Field `INT93` writer - Interrupt 93"]
pub type INT93_W<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `INT94` reader - Interrupt 94"]
pub type INT94_R = crate::BitReader;
#[doc = "Field `INT94` writer - Interrupt 94"]
pub type INT94_W<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `INT95` reader - Interrupt 95"]
pub type INT95_R = crate::BitReader;
#[doc = "Field `INT95` writer - Interrupt 95"]
pub type INT95_W<'a, REG> = crate::BitWriter1S<'a, REG>;
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
    #[doc = "Bit 16 - Interrupt 80"]
    #[inline(always)]
    pub fn int80(&self) -> INT80_R {
        INT80_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Interrupt 81"]
    #[inline(always)]
    pub fn int81(&self) -> INT81_R {
        INT81_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Interrupt 82"]
    #[inline(always)]
    pub fn int82(&self) -> INT82_R {
        INT82_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Interrupt 83"]
    #[inline(always)]
    pub fn int83(&self) -> INT83_R {
        INT83_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Interrupt 84"]
    #[inline(always)]
    pub fn int84(&self) -> INT84_R {
        INT84_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Interrupt 85"]
    #[inline(always)]
    pub fn int85(&self) -> INT85_R {
        INT85_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Interrupt 86"]
    #[inline(always)]
    pub fn int86(&self) -> INT86_R {
        INT86_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Interrupt 87"]
    #[inline(always)]
    pub fn int87(&self) -> INT87_R {
        INT87_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Interrupt 88"]
    #[inline(always)]
    pub fn int88(&self) -> INT88_R {
        INT88_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Interrupt 89"]
    #[inline(always)]
    pub fn int89(&self) -> INT89_R {
        INT89_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Interrupt 90"]
    #[inline(always)]
    pub fn int90(&self) -> INT90_R {
        INT90_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Interrupt 91"]
    #[inline(always)]
    pub fn int91(&self) -> INT91_R {
        INT91_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Interrupt 92"]
    #[inline(always)]
    pub fn int92(&self) -> INT92_R {
        INT92_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Interrupt 93"]
    #[inline(always)]
    pub fn int93(&self) -> INT93_R {
        INT93_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Interrupt 94"]
    #[inline(always)]
    pub fn int94(&self) -> INT94_R {
        INT94_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Interrupt 95"]
    #[inline(always)]
    pub fn int95(&self) -> INT95_R {
        INT95_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICD_ISPENDR2")
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
            .field("int80", &format_args!("{}", self.int80().bit()))
            .field("int81", &format_args!("{}", self.int81().bit()))
            .field("int82", &format_args!("{}", self.int82().bit()))
            .field("int83", &format_args!("{}", self.int83().bit()))
            .field("int84", &format_args!("{}", self.int84().bit()))
            .field("int85", &format_args!("{}", self.int85().bit()))
            .field("int86", &format_args!("{}", self.int86().bit()))
            .field("int87", &format_args!("{}", self.int87().bit()))
            .field("int88", &format_args!("{}", self.int88().bit()))
            .field("int89", &format_args!("{}", self.int89().bit()))
            .field("int90", &format_args!("{}", self.int90().bit()))
            .field("int91", &format_args!("{}", self.int91().bit()))
            .field("int92", &format_args!("{}", self.int92().bit()))
            .field("int93", &format_args!("{}", self.int93().bit()))
            .field("int94", &format_args!("{}", self.int94().bit()))
            .field("int95", &format_args!("{}", self.int95().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<GICD_ISPENDR2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - ARMC Timer"]
    #[inline(always)]
    #[must_use]
    pub fn timer(&mut self) -> TIMER_W<GICD_ISPENDR2_SPEC> {
        TIMER_W::new(self, 0)
    }
    #[doc = "Bit 1 - Mailbox"]
    #[inline(always)]
    #[must_use]
    pub fn mailbox(&mut self) -> MAILBOX_W<GICD_ISPENDR2_SPEC> {
        MAILBOX_W::new(self, 1)
    }
    #[doc = "Bit 2 - Doorbell 0"]
    #[inline(always)]
    #[must_use]
    pub fn doorbell0(&mut self) -> DOORBELL0_W<GICD_ISPENDR2_SPEC> {
        DOORBELL0_W::new(self, 2)
    }
    #[doc = "Bit 3 - Doorbell 1"]
    #[inline(always)]
    #[must_use]
    pub fn doorbell1(&mut self) -> DOORBELL1_W<GICD_ISPENDR2_SPEC> {
        DOORBELL1_W::new(self, 3)
    }
    #[doc = "Bit 4 - VPU0 halted"]
    #[inline(always)]
    #[must_use]
    pub fn vpu0_halted(&mut self) -> VPU0_HALTED_W<GICD_ISPENDR2_SPEC> {
        VPU0_HALTED_W::new(self, 4)
    }
    #[doc = "Bit 5 - VPU1 halted"]
    #[inline(always)]
    #[must_use]
    pub fn vpu1_halted(&mut self) -> VPU1_HALTED_W<GICD_ISPENDR2_SPEC> {
        VPU1_HALTED_W::new(self, 5)
    }
    #[doc = "Bit 6 - ARM address error"]
    #[inline(always)]
    #[must_use]
    pub fn arm_address_error(&mut self) -> ARM_ADDRESS_ERROR_W<GICD_ISPENDR2_SPEC> {
        ARM_ADDRESS_ERROR_W::new(self, 6)
    }
    #[doc = "Bit 7 - ARM AXI error"]
    #[inline(always)]
    #[must_use]
    pub fn arm_axi_error(&mut self) -> ARM_AXI_ERROR_W<GICD_ISPENDR2_SPEC> {
        ARM_AXI_ERROR_W::new(self, 7)
    }
    #[doc = "Bit 8 - Software interrupt 0"]
    #[inline(always)]
    #[must_use]
    pub fn swi0(&mut self) -> SWI0_W<GICD_ISPENDR2_SPEC> {
        SWI0_W::new(self, 8)
    }
    #[doc = "Bit 9 - Software interrupt 1"]
    #[inline(always)]
    #[must_use]
    pub fn swi1(&mut self) -> SWI1_W<GICD_ISPENDR2_SPEC> {
        SWI1_W::new(self, 9)
    }
    #[doc = "Bit 10 - Software interrupt 2"]
    #[inline(always)]
    #[must_use]
    pub fn swi2(&mut self) -> SWI2_W<GICD_ISPENDR2_SPEC> {
        SWI2_W::new(self, 10)
    }
    #[doc = "Bit 11 - Software interrupt 3"]
    #[inline(always)]
    #[must_use]
    pub fn swi3(&mut self) -> SWI3_W<GICD_ISPENDR2_SPEC> {
        SWI3_W::new(self, 11)
    }
    #[doc = "Bit 12 - Software interrupt 4"]
    #[inline(always)]
    #[must_use]
    pub fn swi4(&mut self) -> SWI4_W<GICD_ISPENDR2_SPEC> {
        SWI4_W::new(self, 12)
    }
    #[doc = "Bit 13 - Software interrupt 5"]
    #[inline(always)]
    #[must_use]
    pub fn swi5(&mut self) -> SWI5_W<GICD_ISPENDR2_SPEC> {
        SWI5_W::new(self, 13)
    }
    #[doc = "Bit 14 - Software interrupt 6"]
    #[inline(always)]
    #[must_use]
    pub fn swi6(&mut self) -> SWI6_W<GICD_ISPENDR2_SPEC> {
        SWI6_W::new(self, 14)
    }
    #[doc = "Bit 15 - Software interrupt 7"]
    #[inline(always)]
    #[must_use]
    pub fn swi7(&mut self) -> SWI7_W<GICD_ISPENDR2_SPEC> {
        SWI7_W::new(self, 15)
    }
    #[doc = "Bit 16 - Interrupt 80"]
    #[inline(always)]
    #[must_use]
    pub fn int80(&mut self) -> INT80_W<GICD_ISPENDR2_SPEC> {
        INT80_W::new(self, 16)
    }
    #[doc = "Bit 17 - Interrupt 81"]
    #[inline(always)]
    #[must_use]
    pub fn int81(&mut self) -> INT81_W<GICD_ISPENDR2_SPEC> {
        INT81_W::new(self, 17)
    }
    #[doc = "Bit 18 - Interrupt 82"]
    #[inline(always)]
    #[must_use]
    pub fn int82(&mut self) -> INT82_W<GICD_ISPENDR2_SPEC> {
        INT82_W::new(self, 18)
    }
    #[doc = "Bit 19 - Interrupt 83"]
    #[inline(always)]
    #[must_use]
    pub fn int83(&mut self) -> INT83_W<GICD_ISPENDR2_SPEC> {
        INT83_W::new(self, 19)
    }
    #[doc = "Bit 20 - Interrupt 84"]
    #[inline(always)]
    #[must_use]
    pub fn int84(&mut self) -> INT84_W<GICD_ISPENDR2_SPEC> {
        INT84_W::new(self, 20)
    }
    #[doc = "Bit 21 - Interrupt 85"]
    #[inline(always)]
    #[must_use]
    pub fn int85(&mut self) -> INT85_W<GICD_ISPENDR2_SPEC> {
        INT85_W::new(self, 21)
    }
    #[doc = "Bit 22 - Interrupt 86"]
    #[inline(always)]
    #[must_use]
    pub fn int86(&mut self) -> INT86_W<GICD_ISPENDR2_SPEC> {
        INT86_W::new(self, 22)
    }
    #[doc = "Bit 23 - Interrupt 87"]
    #[inline(always)]
    #[must_use]
    pub fn int87(&mut self) -> INT87_W<GICD_ISPENDR2_SPEC> {
        INT87_W::new(self, 23)
    }
    #[doc = "Bit 24 - Interrupt 88"]
    #[inline(always)]
    #[must_use]
    pub fn int88(&mut self) -> INT88_W<GICD_ISPENDR2_SPEC> {
        INT88_W::new(self, 24)
    }
    #[doc = "Bit 25 - Interrupt 89"]
    #[inline(always)]
    #[must_use]
    pub fn int89(&mut self) -> INT89_W<GICD_ISPENDR2_SPEC> {
        INT89_W::new(self, 25)
    }
    #[doc = "Bit 26 - Interrupt 90"]
    #[inline(always)]
    #[must_use]
    pub fn int90(&mut self) -> INT90_W<GICD_ISPENDR2_SPEC> {
        INT90_W::new(self, 26)
    }
    #[doc = "Bit 27 - Interrupt 91"]
    #[inline(always)]
    #[must_use]
    pub fn int91(&mut self) -> INT91_W<GICD_ISPENDR2_SPEC> {
        INT91_W::new(self, 27)
    }
    #[doc = "Bit 28 - Interrupt 92"]
    #[inline(always)]
    #[must_use]
    pub fn int92(&mut self) -> INT92_W<GICD_ISPENDR2_SPEC> {
        INT92_W::new(self, 28)
    }
    #[doc = "Bit 29 - Interrupt 93"]
    #[inline(always)]
    #[must_use]
    pub fn int93(&mut self) -> INT93_W<GICD_ISPENDR2_SPEC> {
        INT93_W::new(self, 29)
    }
    #[doc = "Bit 30 - Interrupt 94"]
    #[inline(always)]
    #[must_use]
    pub fn int94(&mut self) -> INT94_W<GICD_ISPENDR2_SPEC> {
        INT94_W::new(self, 30)
    }
    #[doc = "Bit 31 - Interrupt 95"]
    #[inline(always)]
    #[must_use]
    pub fn int95(&mut self) -> INT95_W<GICD_ISPENDR2_SPEC> {
        INT95_W::new(self, 31)
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
#[doc = "Interrupt Set-Pending\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gicd_ispendr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gicd_ispendr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GICD_ISPENDR2_SPEC;
impl crate::RegisterSpec for GICD_ISPENDR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gicd_ispendr2::R`](R) reader structure"]
impl crate::Readable for GICD_ISPENDR2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gicd_ispendr2::W`](W) writer structure"]
impl crate::Writable for GICD_ISPENDR2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0xffff_ffff;
}
#[doc = "`reset()` method sets GICD_ISPENDR2 to value 0"]
impl crate::Resettable for GICD_ISPENDR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
