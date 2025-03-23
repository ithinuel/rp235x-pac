#[doc = "Register `EHR_DATA2` reader"]
pub type R = crate::R<EHR_DATA2_SPEC>;
#[doc = "Field `EHR_DATA2` reader - Bits \\[95:64\\] of Entropy Holding Register (EHR) - RNG output register"]
pub type EHR_DATA2_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Bits \\[95:64\\] of Entropy Holding Register (EHR) - RNG output register"]
    #[inline(always)]
    pub fn ehr_data2(&self) -> EHR_DATA2_R {
        EHR_DATA2_R::new(self.bits)
    }
}
#[doc = "RNG collected bits.  

You can [`read`](crate::Reg::read) this register and get [`ehr_data2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EHR_DATA2_SPEC;
impl crate::RegisterSpec for EHR_DATA2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ehr_data2::R`](R) reader structure"]
impl crate::Readable for EHR_DATA2_SPEC {}
#[doc = "`reset()` method sets EHR_DATA2 to value 0"]
impl crate::Resettable for EHR_DATA2_SPEC {}
