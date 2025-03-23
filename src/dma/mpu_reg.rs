#[repr(C)]
#[doc = "Cluster MPU_REG%s, containing MPU_LAR?, MPU_BAR?"]
pub struct MPU_REG {
    mpu_bar: MPU_BAR,
    mpu_lar: MPU_LAR,
}
impl MPU_REG {
    #[doc = "0x00 - Base address register for MPU region 0. Writable only from a Secure, Privileged context."]
    #[inline(always)]
    pub const fn mpu_bar(&self) -> &MPU_BAR {
        &self.mpu_bar
    }
    #[doc = "0x04 - Limit address register for MPU region 0. Writable only from a Secure, Privileged context, with the exception of the P bit."]
    #[inline(always)]
    pub const fn mpu_lar(&self) -> &MPU_LAR {
        &self.mpu_lar
    }
}
#[doc = "MPU_LAR (rw) register accessor: Limit address register for MPU region 0. Writable only from a Secure, Privileged context, with the exception of the P bit.  

You can [`read`](crate::Reg::read) this register and get [`mpu_lar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpu_lar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@mpu_lar`] module"]
pub type MPU_LAR = crate::Reg<mpu_lar::MPU_LAR_SPEC>;
#[doc = "Limit address register for MPU region 0. Writable only from a Secure, Privileged context, with the exception of the P bit."]
pub mod mpu_lar;
#[doc = "MPU_BAR (rw) register accessor: Base address register for MPU region 0. Writable only from a Secure, Privileged context.  

You can [`read`](crate::Reg::read) this register and get [`mpu_bar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpu_bar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@mpu_bar`] module"]
pub type MPU_BAR = crate::Reg<mpu_bar::MPU_BAR_SPEC>;
#[doc = "Base address register for MPU region 0. Writable only from a Secure, Privileged context."]
pub mod mpu_bar;
