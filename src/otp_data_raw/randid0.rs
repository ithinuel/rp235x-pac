#[doc = "Register `RANDID0` reader"]
pub type R = crate::R<RANDID0_SPEC>;
#[doc = "Field `RANDID0` reader - "]
pub type RANDID0_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn randid0(&self) -> RANDID0_R {
        RANDID0_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Bits 15:0 of private per-device random number (ECC)  

 The RANDID0..7 rows form a 128-bit random number generated during device test.  

 This ID is not exposed through the USB PICOBOOT GET_INFO command or the ROM `get_sys_info()` API. However note that the USB PICOBOOT OTP access point can read the entirety of page 0, so this value is not meaningfully private unless the USB PICOBOOT interface is disabled via the DISABLE_BOOTSEL_USB_PICOBOOT_IFC flag in BOOT_FLAGS0.  

You can [`read`](crate::Reg::read) this register and get [`randid0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RANDID0_SPEC;
impl crate::RegisterSpec for RANDID0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`randid0::R`](R) reader structure"]
impl crate::Readable for RANDID0_SPEC {}
#[doc = "`reset()` method sets RANDID0 to value 0"]
impl crate::Resettable for RANDID0_SPEC {
    const RESET_VALUE: u32 = 0;
}
