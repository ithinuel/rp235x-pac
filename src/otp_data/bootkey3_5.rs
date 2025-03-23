#[doc = "Register `BOOTKEY3_5` reader"]
pub type R = crate::R<BOOTKEY3_5_SPEC>;
#[doc = "Field `BOOTKEY3_5` reader - "]
pub type BOOTKEY3_5_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn bootkey3_5(&self) -> BOOTKEY3_5_R {
        BOOTKEY3_5_R::new(self.bits)
    }
}
#[doc = "Bits 95:80 of SHA-256 hash of boot key 3 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`bootkey3_5::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BOOTKEY3_5_SPEC;
impl crate::RegisterSpec for BOOTKEY3_5_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`bootkey3_5::R`](R) reader structure"]
impl crate::Readable for BOOTKEY3_5_SPEC {}
#[doc = "`reset()` method sets BOOTKEY3_5 to value 0"]
impl crate::Resettable for BOOTKEY3_5_SPEC {
    const RESET_VALUE: u16 = 0;
}
