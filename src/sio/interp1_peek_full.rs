#[doc = "Register `INTERP1_PEEK_FULL` reader"]
pub type R = crate::R<INTERP1_PEEK_FULL_SPEC>;
#[doc = "Field `INTERP1_PEEK_FULL` reader - "]
pub type INTERP1_PEEK_FULL_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn interp1_peek_full(&self) -> INTERP1_PEEK_FULL_R {
        INTERP1_PEEK_FULL_R::new(self.bits)
    }
}
#[doc = "Read FULL result, without altering any internal state (PEEK).  

You can [`read`](crate::Reg::read) this register and get [`interp1_peek_full::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTERP1_PEEK_FULL_SPEC;
impl crate::RegisterSpec for INTERP1_PEEK_FULL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`interp1_peek_full::R`](R) reader structure"]
impl crate::Readable for INTERP1_PEEK_FULL_SPEC {}
#[doc = "`reset()` method sets INTERP1_PEEK_FULL to value 0"]
impl crate::Resettable for INTERP1_PEEK_FULL_SPEC {
    const RESET_VALUE: u32 = 0;
}
