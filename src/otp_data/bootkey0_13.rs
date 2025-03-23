#[doc = "Register `BOOTKEY0_13` reader"]
pub type R = crate::R<BOOTKEY0_13_SPEC>;
#[doc = "Field `BOOTKEY0_13` reader - "]
pub type BOOTKEY0_13_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn bootkey0_13(&self) -> BOOTKEY0_13_R {
        BOOTKEY0_13_R::new(self.bits)
    }
}
#[doc = "Bits 223:208 of SHA-256 hash of boot key 0 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`bootkey0_13::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BOOTKEY0_13_SPEC;
impl crate::RegisterSpec for BOOTKEY0_13_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`bootkey0_13::R`](R) reader structure"]
impl crate::Readable for BOOTKEY0_13_SPEC {}
#[doc = "`reset()` method sets BOOTKEY0_13 to value 0"]
impl crate::Resettable for BOOTKEY0_13_SPEC {}
