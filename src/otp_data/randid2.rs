#[doc = "Register `RANDID2` reader"]
pub type R = crate::R<RANDID2_SPEC>;
#[doc = "Field `RANDID2` reader - "]
pub type RANDID2_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn randid2(&self) -> RANDID2_R {
        RANDID2_R::new(self.bits)
    }
}
#[doc = "Bits 47:32 of private per-device random number (ECC)  

You can [`read`](crate::Reg::read) this register and get [`randid2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RANDID2_SPEC;
impl crate::RegisterSpec for RANDID2_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`randid2::R`](R) reader structure"]
impl crate::Readable for RANDID2_SPEC {}
#[doc = "`reset()` method sets RANDID2 to value 0"]
impl crate::Resettable for RANDID2_SPEC {
    const RESET_VALUE: u16 = 0;
}
