#[repr(C)]
#[doc = "Cluster SECCFG%s, containing SECCFG_CH?,SECCFG_CH??"]
pub struct SECCFG {
    seccfg_ch: SECCFG_CH,
}
impl SECCFG {
    #[doc = "0x00 - Security configuration for channel 0. Control whether this channel performs Secure/Non-secure and Privileged/Unprivileged bus accesses.  

 If this channel generates bus accesses of some security level, an access of at least that level (in the order S+P > S+U > NS+P > NS+U) is required to program, trigger, abort, check the status of, interrupt on or acknowledge the interrupt of this channel.  

 This register automatically locks down (becomes read-only) once software starts to configure the channel.  

 This register is world-readable, but is writable only from a Secure, Privileged context."]
    #[inline(always)]
    pub const fn seccfg_ch(&self) -> &SECCFG_CH {
        &self.seccfg_ch
    }
}
#[doc = "SECCFG_CH (rw) register accessor: Security configuration for channel 0. Control whether this channel performs Secure/Non-secure and Privileged/Unprivileged bus accesses.  

 If this channel generates bus accesses of some security level, an access of at least that level (in the order S+P > S+U > NS+P > NS+U) is required to program, trigger, abort, check the status of, interrupt on or acknowledge the interrupt of this channel.  

 This register automatically locks down (becomes read-only) once software starts to configure the channel.  

 This register is world-readable, but is writable only from a Secure, Privileged context.  

You can [`read`](crate::Reg::read) this register and get [`seccfg_ch::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seccfg_ch::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [`mod@seccfg_ch`]
module"]
pub type SECCFG_CH = crate::Reg<seccfg_ch::SECCFG_CH_SPEC>;
#[doc = "Security configuration for channel 0. Control whether this channel performs Secure/Non-secure and Privileged/Unprivileged bus accesses.  

 If this channel generates bus accesses of some security level, an access of at least that level (in the order S+P > S+U > NS+P > NS+U) is required to program, trigger, abort, check the status of, interrupt on or acknowledge the interrupt of this channel.  

 This register automatically locks down (becomes read-only) once software starts to configure the channel.  

 This register is world-readable, but is writable only from a Secure, Privileged context."]
pub mod seccfg_ch;
