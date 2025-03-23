#[doc = "Register `SUM2` reader"]
pub type R = crate::R<SUM2_SPEC>;
#[doc = "Field `SUM2` reader - "]
pub type SUM2_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn sum2(&self) -> SUM2_R {
        SUM2_R::new(self.bits)
    }
}
#[doc = "256-bit checksum result. Contents are undefined when CSR_SUM_VLD is 0.  

You can [`read`](crate::Reg::read) this register and get [`sum2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SUM2_SPEC;
impl crate::RegisterSpec for SUM2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sum2::R`](R) reader structure"]
impl crate::Readable for SUM2_SPEC {}
#[doc = "`reset()` method sets SUM2 to value 0"]
impl crate::Resettable for SUM2_SPEC {}
