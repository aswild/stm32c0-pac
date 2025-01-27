///Register `IDR` reader
pub type R = crate::R<IDRrs>;
///Register `IDR` writer
pub type W = crate::W<IDRrs>;
///Field `GPDR` reader - General-purpose 32-bit data register bits These bits can be used as a temporary storage location for four bytes. This register is not affected by CRC resets generated by the RESET bit in the CRC_CR register
pub type GPDR_R = crate::FieldReader<u32>;
///Field `GPDR` writer - General-purpose 32-bit data register bits These bits can be used as a temporary storage location for four bytes. This register is not affected by CRC resets generated by the RESET bit in the CRC_CR register
pub type GPDR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - General-purpose 32-bit data register bits These bits can be used as a temporary storage location for four bytes. This register is not affected by CRC resets generated by the RESET bit in the CRC_CR register
    #[inline(always)]
    pub fn gpdr(&self) -> GPDR_R {
        GPDR_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IDR").field("gpdr", &self.gpdr()).finish()
    }
}
impl W {
    ///Bits 0:31 - General-purpose 32-bit data register bits These bits can be used as a temporary storage location for four bytes. This register is not affected by CRC resets generated by the RESET bit in the CRC_CR register
    #[inline(always)]
    pub fn gpdr(&mut self) -> GPDR_W<IDRrs> {
        GPDR_W::new(self, 0)
    }
}
/**CRC independent data register

You can [`read`](crate::Reg::read) this register and get [`idr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`idr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C031.html#CRC:IDR)*/
pub struct IDRrs;
impl crate::RegisterSpec for IDRrs {
    type Ux = u32;
}
///`read()` method returns [`idr::R`](R) reader structure
impl crate::Readable for IDRrs {}
///`write(|w| ..)` method takes [`idr::W`](W) writer structure
impl crate::Writable for IDRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets IDR to value 0
impl crate::Resettable for IDRrs {
    const RESET_VALUE: u32 = 0;
}
