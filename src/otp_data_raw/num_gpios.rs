#[doc = "Register `NUM_GPIOS` reader"]
pub type R = crate::R<NUM_GPIOS_SPEC>;
#[doc = "Field `NUM_GPIOS` reader - "]
pub type NUM_GPIOS_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn num_gpios(&self) -> NUM_GPIOS_R {
        NUM_GPIOS_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "The number of main user GPIOs (bank 0). Should read 48 in the QFN80 package, and 30 in the QFN60 package. (ECC)  

You can [`read`](crate::Reg::read) this register and get [`num_gpios::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NUM_GPIOS_SPEC;
impl crate::RegisterSpec for NUM_GPIOS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`num_gpios::R`](R) reader structure"]
impl crate::Readable for NUM_GPIOS_SPEC {}
#[doc = "`reset()` method sets NUM_GPIOS to value 0"]
impl crate::Resettable for NUM_GPIOS_SPEC {}
