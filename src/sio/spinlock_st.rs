#[doc = "Register `SPINLOCK_ST` reader"]
pub type R = crate::R<SPINLOCK_ST_SPEC>;
#[doc = "Field `SPINLOCK_ST` reader - "]
pub type SPINLOCK_ST_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn spinlock_st(&self) -> SPINLOCK_ST_R {
        SPINLOCK_ST_R::new(self.bits)
    }
}
#[doc = "Spinlock state   
 A bitmap containing the state of all 32 spinlocks (1=locked).   
 Mainly intended for debugging.  

You can [`read`](crate::Reg::read) this register and get [`spinlock_st::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPINLOCK_ST_SPEC;
impl crate::RegisterSpec for SPINLOCK_ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spinlock_st::R`](R) reader structure"]
impl crate::Readable for SPINLOCK_ST_SPEC {}
#[doc = "`reset()` method sets SPINLOCK_ST to value 0"]
impl crate::Resettable for SPINLOCK_ST_SPEC {}
