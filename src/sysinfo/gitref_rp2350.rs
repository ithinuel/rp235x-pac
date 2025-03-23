#[doc = "Register `GITREF_RP2350` reader"]
pub type R = crate::R<GITREF_RP2350_SPEC>;
#[doc = "Field `GITREF_RP2350` reader - "]
pub type GITREF_RP2350_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn gitref_rp2350(&self) -> GITREF_RP2350_R {
        GITREF_RP2350_R::new(self.bits)
    }
}
#[doc = "Git hash of the chip source. Used to identify chip version.  

You can [`read`](crate::Reg::read) this register and get [`gitref_rp2350::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GITREF_RP2350_SPEC;
impl crate::RegisterSpec for GITREF_RP2350_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gitref_rp2350::R`](R) reader structure"]
impl crate::Readable for GITREF_RP2350_SPEC {}
#[doc = "`reset()` method sets GITREF_RP2350 to value 0"]
impl crate::Resettable for GITREF_RP2350_SPEC {}
