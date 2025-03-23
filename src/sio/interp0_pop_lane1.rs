#[doc = "Register `INTERP0_POP_LANE1` reader"]
pub type R = crate::R<INTERP0_POP_LANE1_SPEC>;
#[doc = "Field `INTERP0_POP_LANE1` reader - "]
pub type INTERP0_POP_LANE1_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn interp0_pop_lane1(&self) -> INTERP0_POP_LANE1_R {
        INTERP0_POP_LANE1_R::new(self.bits)
    }
}
#[doc = "Read LANE1 result, and simultaneously write lane results to both accumulators (POP).  

You can [`read`](crate::Reg::read) this register and get [`interp0_pop_lane1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTERP0_POP_LANE1_SPEC;
impl crate::RegisterSpec for INTERP0_POP_LANE1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`interp0_pop_lane1::R`](R) reader structure"]
impl crate::Readable for INTERP0_POP_LANE1_SPEC {}
#[doc = "`reset()` method sets INTERP0_POP_LANE1 to value 0"]
impl crate::Resettable for INTERP0_POP_LANE1_SPEC {}
