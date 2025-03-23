#[doc = "Register `CHIPID3` reader"]
pub type R = crate::R<CHIPID3_SPEC>;
#[doc = "Field `CHIPID3` reader - "]
pub type CHIPID3_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn chipid3(&self) -> CHIPID3_R {
        CHIPID3_R::new(self.bits)
    }
}
#[doc = "Bits 63:48 of public device ID (ECC)  

You can [`read`](crate::Reg::read) this register and get [`chipid3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CHIPID3_SPEC;
impl crate::RegisterSpec for CHIPID3_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`chipid3::R`](R) reader structure"]
impl crate::Readable for CHIPID3_SPEC {}
#[doc = "`reset()` method sets CHIPID3 to value 0"]
impl crate::Resettable for CHIPID3_SPEC {}
