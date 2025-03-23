#[doc = "Register `CH5_DBG_TCR` reader"]
pub type R = crate::R<CH5_DBG_TCR_SPEC>;
#[doc = "Field `CH5_DBG_TCR` reader - "]
pub type CH5_DBG_TCR_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn ch5_dbg_tcr(&self) -> CH5_DBG_TCR_R {
        CH5_DBG_TCR_R::new(self.bits)
    }
}
#[doc = "Read to get channel TRANS_COUNT reload value, i.e. the length of the next transfer  

You can [`read`](crate::Reg::read) this register and get [`ch5_dbg_tcr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CH5_DBG_TCR_SPEC;
impl crate::RegisterSpec for CH5_DBG_TCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch5_dbg_tcr::R`](R) reader structure"]
impl crate::Readable for CH5_DBG_TCR_SPEC {}
#[doc = "`reset()` method sets CH5_DBG_TCR to value 0"]
impl crate::Resettable for CH5_DBG_TCR_SPEC {}
