#[doc = "Register `CHIPID2` reader"]
pub type R = crate::R<CHIPID2_SPEC>;
#[doc = "Field `CHIPID2` reader - "]
pub type CHIPID2_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn chipid2(&self) -> CHIPID2_R {
        CHIPID2_R::new(self.bits)
    }
}
#[doc = "Bits 47:32 of public device ID (ECC)  

You can [`read`](crate::Reg::read) this register and get [`chipid2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CHIPID2_SPEC;
impl crate::RegisterSpec for CHIPID2_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`chipid2::R`](R) reader structure"]
impl crate::Readable for CHIPID2_SPEC {}
#[doc = "`reset()` method sets CHIPID2 to value 0"]
impl crate::Resettable for CHIPID2_SPEC {
    const RESET_VALUE: u16 = 0;
}
