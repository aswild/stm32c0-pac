///Register `OSPEEDR` reader
pub type R = crate::R<OSPEEDRrs>;
///Register `OSPEEDR` writer
pub type W = crate::W<OSPEEDRrs>;
/**Port x configuration pin %s

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OSPEED0 {
    ///0: Low speed
    LowSpeed = 0,
    ///1: Medium speed
    MediumSpeed = 1,
    ///2: High speed
    HighSpeed = 2,
    ///3: Very high speed
    VeryHighSpeed = 3,
}
impl From<OSPEED0> for u8 {
    #[inline(always)]
    fn from(variant: OSPEED0) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for OSPEED0 {
    type Ux = u8;
}
impl crate::IsEnum for OSPEED0 {}
///Field `OSPEED(0-15)` reader - Port x configuration pin %s
pub type OSPEED_R = crate::FieldReader<OSPEED0>;
impl OSPEED_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OSPEED0 {
        match self.bits {
            0 => OSPEED0::LowSpeed,
            1 => OSPEED0::MediumSpeed,
            2 => OSPEED0::HighSpeed,
            3 => OSPEED0::VeryHighSpeed,
            _ => unreachable!(),
        }
    }
    ///Low speed
    #[inline(always)]
    pub fn is_low_speed(&self) -> bool {
        *self == OSPEED0::LowSpeed
    }
    ///Medium speed
    #[inline(always)]
    pub fn is_medium_speed(&self) -> bool {
        *self == OSPEED0::MediumSpeed
    }
    ///High speed
    #[inline(always)]
    pub fn is_high_speed(&self) -> bool {
        *self == OSPEED0::HighSpeed
    }
    ///Very high speed
    #[inline(always)]
    pub fn is_very_high_speed(&self) -> bool {
        *self == OSPEED0::VeryHighSpeed
    }
}
///Field `OSPEED(0-15)` writer - Port x configuration pin %s
pub type OSPEED_W<'a, REG> = crate::FieldWriter<'a, REG, 2, OSPEED0, crate::Safe>;
impl<'a, REG> OSPEED_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Low speed
    #[inline(always)]
    pub fn low_speed(self) -> &'a mut crate::W<REG> {
        self.variant(OSPEED0::LowSpeed)
    }
    ///Medium speed
    #[inline(always)]
    pub fn medium_speed(self) -> &'a mut crate::W<REG> {
        self.variant(OSPEED0::MediumSpeed)
    }
    ///High speed
    #[inline(always)]
    pub fn high_speed(self) -> &'a mut crate::W<REG> {
        self.variant(OSPEED0::HighSpeed)
    }
    ///Very high speed
    #[inline(always)]
    pub fn very_high_speed(self) -> &'a mut crate::W<REG> {
        self.variant(OSPEED0::VeryHighSpeed)
    }
}
impl R {
    ///Port x configuration pin (0-15)
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `OSPEED0` field.</div>
    #[inline(always)]
    pub fn ospeed(&self, n: u8) -> OSPEED_R {
        #[allow(clippy::no_effect)]
        [(); 16][n as usize];
        OSPEED_R::new(((self.bits >> (n * 2)) & 3) as u8)
    }
    ///Iterator for array of:
    ///Port x configuration pin (0-15)
    #[inline(always)]
    pub fn ospeed_iter(&self) -> impl Iterator<Item = OSPEED_R> + '_ {
        (0..16).map(move |n| OSPEED_R::new(((self.bits >> (n * 2)) & 3) as u8))
    }
    ///Bits 0:1 - Port x configuration pin 0
    #[inline(always)]
    pub fn ospeed0(&self) -> OSPEED_R {
        OSPEED_R::new((self.bits & 3) as u8)
    }
    ///Bits 2:3 - Port x configuration pin 1
    #[inline(always)]
    pub fn ospeed1(&self) -> OSPEED_R {
        OSPEED_R::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bits 4:5 - Port x configuration pin 2
    #[inline(always)]
    pub fn ospeed2(&self) -> OSPEED_R {
        OSPEED_R::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bits 6:7 - Port x configuration pin 3
    #[inline(always)]
    pub fn ospeed3(&self) -> OSPEED_R {
        OSPEED_R::new(((self.bits >> 6) & 3) as u8)
    }
    ///Bits 8:9 - Port x configuration pin 4
    #[inline(always)]
    pub fn ospeed4(&self) -> OSPEED_R {
        OSPEED_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bits 10:11 - Port x configuration pin 5
    #[inline(always)]
    pub fn ospeed5(&self) -> OSPEED_R {
        OSPEED_R::new(((self.bits >> 10) & 3) as u8)
    }
    ///Bits 12:13 - Port x configuration pin 6
    #[inline(always)]
    pub fn ospeed6(&self) -> OSPEED_R {
        OSPEED_R::new(((self.bits >> 12) & 3) as u8)
    }
    ///Bits 14:15 - Port x configuration pin 7
    #[inline(always)]
    pub fn ospeed7(&self) -> OSPEED_R {
        OSPEED_R::new(((self.bits >> 14) & 3) as u8)
    }
    ///Bits 16:17 - Port x configuration pin 8
    #[inline(always)]
    pub fn ospeed8(&self) -> OSPEED_R {
        OSPEED_R::new(((self.bits >> 16) & 3) as u8)
    }
    ///Bits 18:19 - Port x configuration pin 9
    #[inline(always)]
    pub fn ospeed9(&self) -> OSPEED_R {
        OSPEED_R::new(((self.bits >> 18) & 3) as u8)
    }
    ///Bits 20:21 - Port x configuration pin 10
    #[inline(always)]
    pub fn ospeed10(&self) -> OSPEED_R {
        OSPEED_R::new(((self.bits >> 20) & 3) as u8)
    }
    ///Bits 22:23 - Port x configuration pin 11
    #[inline(always)]
    pub fn ospeed11(&self) -> OSPEED_R {
        OSPEED_R::new(((self.bits >> 22) & 3) as u8)
    }
    ///Bits 24:25 - Port x configuration pin 12
    #[inline(always)]
    pub fn ospeed12(&self) -> OSPEED_R {
        OSPEED_R::new(((self.bits >> 24) & 3) as u8)
    }
    ///Bits 26:27 - Port x configuration pin 13
    #[inline(always)]
    pub fn ospeed13(&self) -> OSPEED_R {
        OSPEED_R::new(((self.bits >> 26) & 3) as u8)
    }
    ///Bits 28:29 - Port x configuration pin 14
    #[inline(always)]
    pub fn ospeed14(&self) -> OSPEED_R {
        OSPEED_R::new(((self.bits >> 28) & 3) as u8)
    }
    ///Bits 30:31 - Port x configuration pin 15
    #[inline(always)]
    pub fn ospeed15(&self) -> OSPEED_R {
        OSPEED_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OSPEEDR")
            .field("ospeed0", &self.ospeed0())
            .field("ospeed1", &self.ospeed1())
            .field("ospeed2", &self.ospeed2())
            .field("ospeed3", &self.ospeed3())
            .field("ospeed4", &self.ospeed4())
            .field("ospeed5", &self.ospeed5())
            .field("ospeed6", &self.ospeed6())
            .field("ospeed7", &self.ospeed7())
            .field("ospeed8", &self.ospeed8())
            .field("ospeed9", &self.ospeed9())
            .field("ospeed10", &self.ospeed10())
            .field("ospeed11", &self.ospeed11())
            .field("ospeed12", &self.ospeed12())
            .field("ospeed13", &self.ospeed13())
            .field("ospeed14", &self.ospeed14())
            .field("ospeed15", &self.ospeed15())
            .finish()
    }
}
impl W {
    ///Port x configuration pin (0-15)
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `OSPEED0` field.</div>
    #[inline(always)]
    pub fn ospeed(&mut self, n: u8) -> OSPEED_W<OSPEEDRrs> {
        #[allow(clippy::no_effect)]
        [(); 16][n as usize];
        OSPEED_W::new(self, n * 2)
    }
    ///Bits 0:1 - Port x configuration pin 0
    #[inline(always)]
    pub fn ospeed0(&mut self) -> OSPEED_W<OSPEEDRrs> {
        OSPEED_W::new(self, 0)
    }
    ///Bits 2:3 - Port x configuration pin 1
    #[inline(always)]
    pub fn ospeed1(&mut self) -> OSPEED_W<OSPEEDRrs> {
        OSPEED_W::new(self, 2)
    }
    ///Bits 4:5 - Port x configuration pin 2
    #[inline(always)]
    pub fn ospeed2(&mut self) -> OSPEED_W<OSPEEDRrs> {
        OSPEED_W::new(self, 4)
    }
    ///Bits 6:7 - Port x configuration pin 3
    #[inline(always)]
    pub fn ospeed3(&mut self) -> OSPEED_W<OSPEEDRrs> {
        OSPEED_W::new(self, 6)
    }
    ///Bits 8:9 - Port x configuration pin 4
    #[inline(always)]
    pub fn ospeed4(&mut self) -> OSPEED_W<OSPEEDRrs> {
        OSPEED_W::new(self, 8)
    }
    ///Bits 10:11 - Port x configuration pin 5
    #[inline(always)]
    pub fn ospeed5(&mut self) -> OSPEED_W<OSPEEDRrs> {
        OSPEED_W::new(self, 10)
    }
    ///Bits 12:13 - Port x configuration pin 6
    #[inline(always)]
    pub fn ospeed6(&mut self) -> OSPEED_W<OSPEEDRrs> {
        OSPEED_W::new(self, 12)
    }
    ///Bits 14:15 - Port x configuration pin 7
    #[inline(always)]
    pub fn ospeed7(&mut self) -> OSPEED_W<OSPEEDRrs> {
        OSPEED_W::new(self, 14)
    }
    ///Bits 16:17 - Port x configuration pin 8
    #[inline(always)]
    pub fn ospeed8(&mut self) -> OSPEED_W<OSPEEDRrs> {
        OSPEED_W::new(self, 16)
    }
    ///Bits 18:19 - Port x configuration pin 9
    #[inline(always)]
    pub fn ospeed9(&mut self) -> OSPEED_W<OSPEEDRrs> {
        OSPEED_W::new(self, 18)
    }
    ///Bits 20:21 - Port x configuration pin 10
    #[inline(always)]
    pub fn ospeed10(&mut self) -> OSPEED_W<OSPEEDRrs> {
        OSPEED_W::new(self, 20)
    }
    ///Bits 22:23 - Port x configuration pin 11
    #[inline(always)]
    pub fn ospeed11(&mut self) -> OSPEED_W<OSPEEDRrs> {
        OSPEED_W::new(self, 22)
    }
    ///Bits 24:25 - Port x configuration pin 12
    #[inline(always)]
    pub fn ospeed12(&mut self) -> OSPEED_W<OSPEEDRrs> {
        OSPEED_W::new(self, 24)
    }
    ///Bits 26:27 - Port x configuration pin 13
    #[inline(always)]
    pub fn ospeed13(&mut self) -> OSPEED_W<OSPEEDRrs> {
        OSPEED_W::new(self, 26)
    }
    ///Bits 28:29 - Port x configuration pin 14
    #[inline(always)]
    pub fn ospeed14(&mut self) -> OSPEED_W<OSPEEDRrs> {
        OSPEED_W::new(self, 28)
    }
    ///Bits 30:31 - Port x configuration pin 15
    #[inline(always)]
    pub fn ospeed15(&mut self) -> OSPEED_W<OSPEEDRrs> {
        OSPEED_W::new(self, 30)
    }
}
/**GPIO port output speed register

You can [`read`](crate::Reg::read) this register and get [`ospeedr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ospeedr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C031.html#GPIOC:OSPEEDR)*/
pub struct OSPEEDRrs;
impl crate::RegisterSpec for OSPEEDRrs {
    type Ux = u32;
}
///`read()` method returns [`ospeedr::R`](R) reader structure
impl crate::Readable for OSPEEDRrs {}
///`write(|w| ..)` method takes [`ospeedr::W`](W) writer structure
impl crate::Writable for OSPEEDRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets OSPEEDR to value 0x0c00_0000
impl crate::Resettable for OSPEEDRrs {
    const RESET_VALUE: u32 = 0x0c00_0000;
}
