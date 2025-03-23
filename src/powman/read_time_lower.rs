#[doc = "Register `READ_TIME_LOWER` reader"]
pub type R = crate::R<READ_TIME_LOWER_SPEC>;
#[doc = "Field `READ_TIME_LOWER` reader - For reading bits 31:0 of the timer."]
pub type READ_TIME_LOWER_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - For reading bits 31:0 of the timer."]
    #[inline(always)]
    pub fn read_time_lower(&self) -> READ_TIME_LOWER_R {
        READ_TIME_LOWER_R::new(self.bits)
    }
}
#[doc = "  

You can [`read`](crate::Reg::read) this register and get [`read_time_lower::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct READ_TIME_LOWER_SPEC;
impl crate::RegisterSpec for READ_TIME_LOWER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`read_time_lower::R`](R) reader structure"]
impl crate::Readable for READ_TIME_LOWER_SPEC {}
#[doc = "`reset()` method sets READ_TIME_LOWER to value 0"]
impl crate::Resettable for READ_TIME_LOWER_SPEC {
    const RESET_VALUE: u32 = 0;
}
