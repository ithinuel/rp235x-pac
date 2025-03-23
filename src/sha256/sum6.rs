#[doc = "Register `SUM6` reader"]
pub type R = crate::R<SUM6_SPEC>;
#[doc = "Field `SUM6` reader - "]
pub type SUM6_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn sum6(&self) -> SUM6_R {
        SUM6_R::new(self.bits)
    }
}
#[doc = "256-bit checksum result. Contents are undefined when CSR_SUM_VLD is 0.  

You can [`read`](crate::Reg::read) this register and get [`sum6::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SUM6_SPEC;
impl crate::RegisterSpec for SUM6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sum6::R`](R) reader structure"]
impl crate::Readable for SUM6_SPEC {}
#[doc = "`reset()` method sets SUM6 to value 0"]
impl crate::Resettable for SUM6_SPEC {
    const RESET_VALUE: u32 = 0;
}
