#[doc = "Register `BOOTKEY0_12` reader"]
pub type R = crate::R<BOOTKEY0_12_SPEC>;
#[doc = "Field `BOOTKEY0_12` reader - "]
pub type BOOTKEY0_12_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn bootkey0_12(&self) -> BOOTKEY0_12_R {
        BOOTKEY0_12_R::new(self.bits)
    }
}
#[doc = "Bits 207:192 of SHA-256 hash of boot key 0 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`bootkey0_12::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BOOTKEY0_12_SPEC;
impl crate::RegisterSpec for BOOTKEY0_12_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`bootkey0_12::R`](R) reader structure"]
impl crate::Readable for BOOTKEY0_12_SPEC {}
#[doc = "`reset()` method sets BOOTKEY0_12 to value 0"]
impl crate::Resettable for BOOTKEY0_12_SPEC {}
