#[doc = "Register `TMDS_POP_DOUBLE_L0` reader"]
pub type R = crate::R<TMDS_POP_DOUBLE_L0_SPEC>;
#[doc = "Field `TMDS_POP_DOUBLE_L0` reader -   

<div class=\"warning\">The field is <b>modified</b> in some way after a read operation.</div>"]
pub type TMDS_POP_DOUBLE_L0_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn tmds_pop_double_l0(&self) -> TMDS_POP_DOUBLE_L0_R {
        TMDS_POP_DOUBLE_L0_R::new(self.bits)
    }
}
#[doc = "Get lane 0 of the encoding of two pixels' worth of colour data. Two 10-bit TMDS symbols are packed at the bottom of a 32-bit word.  

 The POP alias shifts the colour register when read, according to the values of PIX_SHIFT and PIX2_NOSHIFT.  

You can [`read`](crate::Reg::read) this register and get [`tmds_pop_double_l0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TMDS_POP_DOUBLE_L0_SPEC;
impl crate::RegisterSpec for TMDS_POP_DOUBLE_L0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tmds_pop_double_l0::R`](R) reader structure"]
impl crate::Readable for TMDS_POP_DOUBLE_L0_SPEC {}
#[doc = "`reset()` method sets TMDS_POP_DOUBLE_L0 to value 0"]
impl crate::Resettable for TMDS_POP_DOUBLE_L0_SPEC {}
