#[doc = "Register `RANDID5` reader"]
pub type R = crate::R<RANDID5_SPEC>;
#[doc = "Field `RANDID5` reader - "]
pub type RANDID5_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn randid5(&self) -> RANDID5_R {
        RANDID5_R::new(self.bits)
    }
}
#[doc = "Bits 95:80 of private per-device random number (ECC)  

You can [`read`](crate::Reg::read) this register and get [`randid5::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RANDID5_SPEC;
impl crate::RegisterSpec for RANDID5_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`randid5::R`](R) reader structure"]
impl crate::Readable for RANDID5_SPEC {}
#[doc = "`reset()` method sets RANDID5 to value 0"]
impl crate::Resettable for RANDID5_SPEC {}
