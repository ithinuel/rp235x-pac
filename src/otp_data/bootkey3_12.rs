#[doc = "Register `BOOTKEY3_12` reader"]
pub type R = crate::R<BOOTKEY3_12_SPEC>;
#[doc = "Field `BOOTKEY3_12` reader - "]
pub type BOOTKEY3_12_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn bootkey3_12(&self) -> BOOTKEY3_12_R {
        BOOTKEY3_12_R::new(self.bits)
    }
}
#[doc = "Bits 207:192 of SHA-256 hash of boot key 3 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`bootkey3_12::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BOOTKEY3_12_SPEC;
impl crate::RegisterSpec for BOOTKEY3_12_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`bootkey3_12::R`](R) reader structure"]
impl crate::Readable for BOOTKEY3_12_SPEC {}
#[doc = "`reset()` method sets BOOTKEY3_12 to value 0"]
impl crate::Resettable for BOOTKEY3_12_SPEC {}
