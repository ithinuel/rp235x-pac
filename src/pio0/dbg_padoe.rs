#[doc = "Register `DBG_PADOE` reader"]
pub type R = crate::R<DBG_PADOE_SPEC>;
#[doc = "Field `DBG_PADOE` reader - "]
pub type DBG_PADOE_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn dbg_padoe(&self) -> DBG_PADOE_R {
        DBG_PADOE_R::new(self.bits)
    }
}
#[doc = "Read to sample the pad output enables (direction) PIO is currently driving to the GPIOs. On RP2040 there are 30 GPIOs, so the two most significant bits are hardwired to 0.  

You can [`read`](crate::Reg::read) this register and get [`dbg_padoe::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DBG_PADOE_SPEC;
impl crate::RegisterSpec for DBG_PADOE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbg_padoe::R`](R) reader structure"]
impl crate::Readable for DBG_PADOE_SPEC {}
#[doc = "`reset()` method sets DBG_PADOE to value 0"]
impl crate::Resettable for DBG_PADOE_SPEC {}
