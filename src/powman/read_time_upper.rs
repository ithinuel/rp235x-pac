#[doc = "Register `READ_TIME_UPPER` reader"]
pub type R = crate::R<READ_TIME_UPPER_SPEC>;
#[doc = "Field `READ_TIME_UPPER` reader - For reading bits 63:32 of the timer. When reading all 64 bits it is possible for the LOWER count to rollover during the read. It is recommended to read UPPER, then LOWER, then re-read UPPER and, if it has changed, re-read LOWER."]
pub type READ_TIME_UPPER_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - For reading bits 63:32 of the timer. When reading all 64 bits it is possible for the LOWER count to rollover during the read. It is recommended to read UPPER, then LOWER, then re-read UPPER and, if it has changed, re-read LOWER."]
    #[inline(always)]
    pub fn read_time_upper(&self) -> READ_TIME_UPPER_R {
        READ_TIME_UPPER_R::new(self.bits)
    }
}
#[doc = "  

You can [`read`](crate::Reg::read) this register and get [`read_time_upper::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct READ_TIME_UPPER_SPEC;
impl crate::RegisterSpec for READ_TIME_UPPER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`read_time_upper::R`](R) reader structure"]
impl crate::Readable for READ_TIME_UPPER_SPEC {}
#[doc = "`reset()` method sets READ_TIME_UPPER to value 0"]
impl crate::Resettable for READ_TIME_UPPER_SPEC {
    const RESET_VALUE: u32 = 0;
}
