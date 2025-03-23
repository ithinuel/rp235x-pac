#[doc = "Register `EHR_DATA4` reader"]
pub type R = crate::R<EHR_DATA4_SPEC>;
#[doc = "Field `EHR_DATA4` reader - Bits \\[159:128\\] of Entropy Holding Register (EHR) - RNG output register"]
pub type EHR_DATA4_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Bits \\[159:128\\] of Entropy Holding Register (EHR) - RNG output register"]
    #[inline(always)]
    pub fn ehr_data4(&self) -> EHR_DATA4_R {
        EHR_DATA4_R::new(self.bits)
    }
}
#[doc = "RNG collected bits.  

You can [`read`](crate::Reg::read) this register and get [`ehr_data4::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EHR_DATA4_SPEC;
impl crate::RegisterSpec for EHR_DATA4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ehr_data4::R`](R) reader structure"]
impl crate::Readable for EHR_DATA4_SPEC {}
#[doc = "`reset()` method sets EHR_DATA4 to value 0"]
impl crate::Resettable for EHR_DATA4_SPEC {}
