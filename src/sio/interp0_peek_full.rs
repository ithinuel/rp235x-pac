#[doc = "Register `INTERP0_PEEK_FULL` reader"]
pub type R = crate::R<INTERP0_PEEK_FULL_SPEC>;
#[doc = "Field `INTERP0_PEEK_FULL` reader - "]
pub type INTERP0_PEEK_FULL_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn interp0_peek_full(&self) -> INTERP0_PEEK_FULL_R {
        INTERP0_PEEK_FULL_R::new(self.bits)
    }
}
#[doc = "Read FULL result, without altering any internal state (PEEK).  

You can [`read`](crate::Reg::read) this register and get [`interp0_peek_full::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTERP0_PEEK_FULL_SPEC;
impl crate::RegisterSpec for INTERP0_PEEK_FULL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`interp0_peek_full::R`](R) reader structure"]
impl crate::Readable for INTERP0_PEEK_FULL_SPEC {}
#[doc = "`reset()` method sets INTERP0_PEEK_FULL to value 0"]
impl crate::Resettable for INTERP0_PEEK_FULL_SPEC {}
