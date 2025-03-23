#[doc = "Register `EHR_DATA3` reader"]
pub type R = crate::R<EHR_DATA3_SPEC>;
#[doc = "Field `EHR_DATA3` reader - Bits \\[127:96\\]
of Entropy Holding Register (EHR) - RNG output register"]
pub type EHR_DATA3_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Bits \\[127:96\\]
of Entropy Holding Register (EHR) - RNG output register"]
    #[inline(always)]
    pub fn ehr_data3(&self) -> EHR_DATA3_R {
        EHR_DATA3_R::new(self.bits)
    }
}
#[doc = "RNG collected bits.  

You can [`read`](crate::Reg::read) this register and get [`ehr_data3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EHR_DATA3_SPEC;
impl crate::RegisterSpec for EHR_DATA3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ehr_data3::R`](R) reader structure"]
impl crate::Readable for EHR_DATA3_SPEC {}
#[doc = "`reset()` method sets EHR_DATA3 to value 0"]
impl crate::Resettable for EHR_DATA3_SPEC {
    const RESET_VALUE: u32 = 0;
}
