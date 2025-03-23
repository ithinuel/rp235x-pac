#[doc = "Register `STREAM_FIFO` reader"]
pub type R = crate::R<STREAM_FIFO_SPEC>;
#[doc = "Field `STREAM_FIFO` reader - Streamed data is buffered here, for retrieval by the system DMA.   
 This FIFO can also be accessed via the XIP_AUX slave, to avoid exposing   
 the DMA to bus stalls caused by other XIP traffic.  

<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type STREAM_FIFO_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Streamed data is buffered here, for retrieval by the system DMA.   
 This FIFO can also be accessed via the XIP_AUX slave, to avoid exposing   
 the DMA to bus stalls caused by other XIP traffic."]
    #[inline(always)]
    pub fn stream_fifo(&self) -> STREAM_FIFO_R {
        STREAM_FIFO_R::new(self.bits)
    }
}
#[doc = "FIFO stream data  

You can [`read`](crate::Reg::read) this register and get [`stream_fifo::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STREAM_FIFO_SPEC;
impl crate::RegisterSpec for STREAM_FIFO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stream_fifo::R`](R) reader structure"]
impl crate::Readable for STREAM_FIFO_SPEC {}
#[doc = "`reset()` method sets STREAM_FIFO to value 0"]
impl crate::Resettable for STREAM_FIFO_SPEC {}
