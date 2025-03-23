#[doc = "Register `FIFO_WR` writer"]
pub type W = crate::W<FIFO_WR_SPEC>;
#[doc = "Field `FIFO_WR` writer - "]
pub type FIFO_WR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn fifo_wr(&mut self) -> FIFO_WR_W<FIFO_WR_SPEC> {
        FIFO_WR_W::new(self, 0)
    }
}
#[doc = "Write access to this core's TX FIFO  

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo_wr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FIFO_WR_SPEC;
impl crate::RegisterSpec for FIFO_WR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`fifo_wr::W`](W) writer structure"]
impl crate::Writable for FIFO_WR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FIFO_WR to value 0"]
impl crate::Resettable for FIFO_WR_SPEC {
    const RESET_VALUE: u32 = 0;
}
