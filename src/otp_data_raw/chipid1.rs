#[doc = "Register `CHIPID1` reader"]
pub type R = crate::R<CHIPID1_SPEC>;
#[doc = "Field `CHIPID1` reader - "]
pub type CHIPID1_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn chipid1(&self) -> CHIPID1_R {
        CHIPID1_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Bits 31:16 of public device ID (ECC)  

You can [`read`](crate::Reg::read) this register and get [`chipid1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CHIPID1_SPEC;
impl crate::RegisterSpec for CHIPID1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chipid1::R`](R) reader structure"]
impl crate::Readable for CHIPID1_SPEC {}
#[doc = "`reset()` method sets CHIPID1 to value 0"]
impl crate::Resettable for CHIPID1_SPEC {
    const RESET_VALUE: u32 = 0;
}
