#[doc = "Register `FIFO_RD` reader"]
pub type R = crate::R<FIFO_RD_SPEC>;
#[doc = "Field `FIFO_RD` reader -   

<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type FIFO_RD_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn fifo_rd(&self) -> FIFO_RD_R {
        FIFO_RD_R::new(self.bits)
    }
}
#[doc = "Read access to this core's RX FIFO  

You can [`read`](crate::Reg::read) this register and get [`fifo_rd::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FIFO_RD_SPEC;
impl crate::RegisterSpec for FIFO_RD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fifo_rd::R`](R) reader structure"]
impl crate::Readable for FIFO_RD_SPEC {}
#[doc = "`reset()` method sets FIFO_RD to value 0"]
impl crate::Resettable for FIFO_RD_SPEC {}
