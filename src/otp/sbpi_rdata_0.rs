#[doc = "Register `SBPI_RDATA_0` reader"]
pub type R = crate::R<SBPI_RDATA_0_SPEC>;
#[doc = "Field `SBPI_RDATA_0` reader -   

<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type SBPI_RDATA_0_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn sbpi_rdata_0(&self) -> SBPI_RDATA_0_R {
        SBPI_RDATA_0_R::new(self.bits)
    }
}
#[doc = "Read payload bytes 3..0. Once read, the data in the register will automatically clear to 0.  

You can [`read`](crate::Reg::read) this register and get [`sbpi_rdata_0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SBPI_RDATA_0_SPEC;
impl crate::RegisterSpec for SBPI_RDATA_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sbpi_rdata_0::R`](R) reader structure"]
impl crate::Readable for SBPI_RDATA_0_SPEC {}
#[doc = "`reset()` method sets SBPI_RDATA_0 to value 0"]
impl crate::Resettable for SBPI_RDATA_0_SPEC {
    const RESET_VALUE: u32 = 0;
}
