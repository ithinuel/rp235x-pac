#[doc = "Register `ROSC_CALIB` reader"]
pub type R = crate::R<ROSC_CALIB_SPEC>;
#[doc = "Field `ROSC_CALIB` reader - "]
pub type ROSC_CALIB_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn rosc_calib(&self) -> ROSC_CALIB_R {
        ROSC_CALIB_R::new(self.bits)
    }
}
#[doc = "Ring oscillator frequency in kHz, measured during manufacturing (ECC)  

 This is measured at 1.1 V, at room temperature, with the ROSC configuration registers in their reset state.  

You can [`read`](crate::Reg::read) this register and get [`rosc_calib::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ROSC_CALIB_SPEC;
impl crate::RegisterSpec for ROSC_CALIB_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`rosc_calib::R`](R) reader structure"]
impl crate::Readable for ROSC_CALIB_SPEC {}
#[doc = "`reset()` method sets ROSC_CALIB to value 0"]
impl crate::Resettable for ROSC_CALIB_SPEC {}
