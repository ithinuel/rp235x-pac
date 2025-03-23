#[doc = "Register `SUM3` reader"]
pub type R = crate::R<SUM3_SPEC>;
#[doc = "Field `SUM3` reader - "]
pub type SUM3_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn sum3(&self) -> SUM3_R {
        SUM3_R::new(self.bits)
    }
}
#[doc = "256-bit checksum result. Contents are undefined when CSR_SUM_VLD is 0.  

You can [`read`](crate::Reg::read) this register and get [`sum3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SUM3_SPEC;
impl crate::RegisterSpec for SUM3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sum3::R`](R) reader structure"]
impl crate::Readable for SUM3_SPEC {}
#[doc = "`reset()` method sets SUM3 to value 0"]
impl crate::Resettable for SUM3_SPEC {}
