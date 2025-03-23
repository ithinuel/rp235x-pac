#[doc = "Register `TMDS_POP_SINGLE` reader"]
pub type R = crate::R<TMDS_POP_SINGLE_SPEC>;
#[doc = "Field `TMDS_POP_SINGLE` reader -   

<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type TMDS_POP_SINGLE_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn tmds_pop_single(&self) -> TMDS_POP_SINGLE_R {
        TMDS_POP_SINGLE_R::new(self.bits)
    }
}
#[doc = "Get the encoding of one pixel's worth of colour data, packed into a 32-bit value. The packing is 5 chunks of 3 lanes times 2 bits (30 bits total). Each chunk contains two bits of a TMDS symbol per lane. This format is intended for shifting out with the HSTX peripheral on RP2350.  

 The POP alias shifts the colour register when read, as well as advancing the running DC balance state of each encoder.  

You can [`read`](crate::Reg::read) this register and get [`tmds_pop_single::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TMDS_POP_SINGLE_SPEC;
impl crate::RegisterSpec for TMDS_POP_SINGLE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tmds_pop_single::R`](R) reader structure"]
impl crate::Readable for TMDS_POP_SINGLE_SPEC {}
#[doc = "`reset()` method sets TMDS_POP_SINGLE to value 0"]
impl crate::Resettable for TMDS_POP_SINGLE_SPEC {}
