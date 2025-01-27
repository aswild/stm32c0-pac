///Register `ARR` reader
pub type R = crate::R<ARRrs>;
///Register `ARR` writer
pub type W = crate::W<ARRrs>;
///Field `ARR` reader - Low Auto-reload value
pub type ARR_R = crate::FieldReader<u32>;
///Field `ARR` writer - Low Auto-reload value
pub type ARR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32, crate::Safe>;
impl R {
    ///Bits 0:31 - Low Auto-reload value
    #[inline(always)]
    pub fn arr(&self) -> ARR_R {
        ARR_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ARR").field("arr", &self.arr()).finish()
    }
}
impl W {
    ///Bits 0:31 - Low Auto-reload value
    #[inline(always)]
    pub fn arr(&mut self) -> ARR_W<ARRrs> {
        ARR_W::new(self, 0)
    }
}
/**TIM2 auto-reload register

You can [`read`](crate::Reg::read) this register and get [`arr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`arr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C071.html#TIM2:ARR)*/
pub struct ARRrs;
impl crate::RegisterSpec for ARRrs {
    type Ux = u32;
}
///`read()` method returns [`arr::R`](R) reader structure
impl crate::Readable for ARRrs {}
///`write(|w| ..)` method takes [`arr::W`](W) writer structure
impl crate::Writable for ARRrs {
    type Safety = crate::Safe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets ARR to value 0xffff_ffff
impl crate::Resettable for ARRrs {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
