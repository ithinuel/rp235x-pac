#[doc = "Register `SBPI_RDATA_3` reader"]
pub type R = crate::R<SBPI_RDATA_3_SPEC>;
#[doc = "Field `SBPI_RDATA_3` reader -   

<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type SBPI_RDATA_3_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn sbpi_rdata_3(&self) -> SBPI_RDATA_3_R {
        SBPI_RDATA_3_R::new(self.bits)
    }
}
#[doc = "Read payload bytes 15..12. Once read, the data in the register will automatically clear to 0.  

You can [`read`](crate::Reg::read) this register and get [`sbpi_rdata_3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SBPI_RDATA_3_SPEC;
impl crate::RegisterSpec for SBPI_RDATA_3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sbpi_rdata_3::R`](R) reader structure"]
impl crate::Readable for SBPI_RDATA_3_SPEC {}
#[doc = "`reset()` method sets SBPI_RDATA_3 to value 0"]
impl crate::Resettable for SBPI_RDATA_3_SPEC {}
