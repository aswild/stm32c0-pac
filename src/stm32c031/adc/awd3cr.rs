///Register `AWD3CR` reader
pub type R = crate::R<AWD3CRrs>;
///Register `AWD3CR` writer
pub type W = crate::W<AWD3CRrs>;
/**Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\[3:0\]
for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AWD3CH0 {
    ///0: ADC analog channel-x is not monitored by AWD3
    NotMonitored = 0,
    ///1: ADC analog channel-x is monitored by AWD3
    Monitored = 1,
}
impl From<AWD3CH0> for bool {
    #[inline(always)]
    fn from(variant: AWD3CH0) -> Self {
        variant as u8 != 0
    }
}
/**Field `AWD3CH(0-22)` reader - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\[3:0\]
for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).*/
pub type AWD3CH_R = crate::BitReader<AWD3CH0>;
impl AWD3CH_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> AWD3CH0 {
        match self.bits {
            false => AWD3CH0::NotMonitored,
            true => AWD3CH0::Monitored,
        }
    }
    ///ADC analog channel-x is not monitored by AWD3
    #[inline(always)]
    pub fn is_not_monitored(&self) -> bool {
        *self == AWD3CH0::NotMonitored
    }
    ///ADC analog channel-x is monitored by AWD3
    #[inline(always)]
    pub fn is_monitored(&self) -> bool {
        *self == AWD3CH0::Monitored
    }
}
/**Field `AWD3CH(0-22)` writer - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\[3:0\]
for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).*/
pub type AWD3CH_W<'a, REG> = crate::BitWriter<'a, REG, AWD3CH0>;
impl<'a, REG> AWD3CH_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///ADC analog channel-x is not monitored by AWD3
    #[inline(always)]
    pub fn not_monitored(self) -> &'a mut crate::W<REG> {
        self.variant(AWD3CH0::NotMonitored)
    }
    ///ADC analog channel-x is monitored by AWD3
    #[inline(always)]
    pub fn monitored(self) -> &'a mut crate::W<REG> {
        self.variant(AWD3CH0::Monitored)
    }
}
impl R {
    /**Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\[3:0\]
    for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).*/
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `AWD3CH0` field.</div>
    #[inline(always)]
    pub fn awd3ch(&self, n: u8) -> AWD3CH_R {
        #[allow(clippy::no_effect)]
        [(); 23][n as usize];
        AWD3CH_R::new(((self.bits >> n) & 1) != 0)
    }
    ///Iterator for array of:
    /**Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\[3:0\]
    for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).*/
    #[inline(always)]
    pub fn awd3ch_iter(&self) -> impl Iterator<Item = AWD3CH_R> + '_ {
        (0..23).map(move |n| AWD3CH_R::new(((self.bits >> n) & 1) != 0))
    }
    /**Bit 0 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\[3:0\]
    for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).*/
    #[inline(always)]
    pub fn awd3ch0(&self) -> AWD3CH_R {
        AWD3CH_R::new((self.bits & 1) != 0)
    }
    /**Bit 1 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\[3:0\]
    for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).*/
    #[inline(always)]
    pub fn awd3ch1(&self) -> AWD3CH_R {
        AWD3CH_R::new(((self.bits >> 1) & 1) != 0)
    }
    /**Bit 2 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\[3:0\]
    for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).*/
    #[inline(always)]
    pub fn awd3ch2(&self) -> AWD3CH_R {
        AWD3CH_R::new(((self.bits >> 2) & 1) != 0)
    }
    /**Bit 3 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\[3:0\]
    for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).*/
    #[inline(always)]
    pub fn awd3ch3(&self) -> AWD3CH_R {
        AWD3CH_R::new(((self.bits >> 3) & 1) != 0)
    }
    /**Bit 4 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\[3:0\]
    for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).*/
    #[inline(always)]
    pub fn awd3ch4(&self) -> AWD3CH_R {
        AWD3CH_R::new(((self.bits >> 4) & 1) != 0)
    }
    /**Bit 5 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\[3:0\]
    for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).*/
    #[inline(always)]
    pub fn awd3ch5(&self) -> AWD3CH_R {
        AWD3CH_R::new(((self.bits >> 5) & 1) != 0)
    }
    /**Bit 6 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\[3:0\]
    for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).*/
    #[inline(always)]
    pub fn awd3ch6(&self) -> AWD3CH_R {
        AWD3CH_R::new(((self.bits >> 6) & 1) != 0)
    }
    /**Bit 7 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\[3:0\]
    for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).*/
    #[inline(always)]
    pub fn awd3ch7(&self) -> AWD3CH_R {
        AWD3CH_R::new(((self.bits >> 7) & 1) != 0)
    }
    /**Bit 8 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\[3:0\]
    for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).*/
    #[inline(always)]
    pub fn awd3ch8(&self) -> AWD3CH_R {
        AWD3CH_R::new(((self.bits >> 8) & 1) != 0)
    }
    /**Bit 9 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\[3:0\]
    for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).*/
    #[inline(always)]
    pub fn awd3ch9(&self) -> AWD3CH_R {
        AWD3CH_R::new(((self.bits >> 9) & 1) != 0)
    }
    /**Bit 10 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\[3:0\]
    for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).*/
    #[inline(always)]
    pub fn awd3ch10(&self) -> AWD3CH_R {
        AWD3CH_R::new(((self.bits >> 10) & 1) != 0)
    }
    /**Bit 11 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\[3:0\]
    for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).*/
    #[inline(always)]
    pub fn awd3ch11(&self) -> AWD3CH_R {
        AWD3CH_R::new(((self.bits >> 11) & 1) != 0)
    }
    /**Bit 12 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\[3:0\]
    for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).*/
    #[inline(always)]
    pub fn awd3ch12(&self) -> AWD3CH_R {
        AWD3CH_R::new(((self.bits >> 12) & 1) != 0)
    }
    /**Bit 13 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\[3:0\]
    for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).*/
    #[inline(always)]
    pub fn awd3ch13(&self) -> AWD3CH_R {
        AWD3CH_R::new(((self.bits >> 13) & 1) != 0)
    }
    /**Bit 14 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\[3:0\]
    for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).*/
    #[inline(always)]
    pub fn awd3ch14(&self) -> AWD3CH_R {
        AWD3CH_R::new(((self.bits >> 14) & 1) != 0)
    }
    /**Bit 15 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\[3:0\]
    for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).*/
    #[inline(always)]
    pub fn awd3ch15(&self) -> AWD3CH_R {
        AWD3CH_R::new(((self.bits >> 15) & 1) != 0)
    }
    /**Bit 16 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\[3:0\]
    for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).*/
    #[inline(always)]
    pub fn awd3ch16(&self) -> AWD3CH_R {
        AWD3CH_R::new(((self.bits >> 16) & 1) != 0)
    }
    /**Bit 17 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\[3:0\]
    for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).*/
    #[inline(always)]
    pub fn awd3ch17(&self) -> AWD3CH_R {
        AWD3CH_R::new(((self.bits >> 17) & 1) != 0)
    }
    /**Bit 18 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\[3:0\]
    for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).*/
    #[inline(always)]
    pub fn awd3ch18(&self) -> AWD3CH_R {
        AWD3CH_R::new(((self.bits >> 18) & 1) != 0)
    }
    /**Bit 19 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\[3:0\]
    for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).*/
    #[inline(always)]
    pub fn awd3ch19(&self) -> AWD3CH_R {
        AWD3CH_R::new(((self.bits >> 19) & 1) != 0)
    }
    /**Bit 20 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\[3:0\]
    for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).*/
    #[inline(always)]
    pub fn awd3ch20(&self) -> AWD3CH_R {
        AWD3CH_R::new(((self.bits >> 20) & 1) != 0)
    }
    /**Bit 21 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\[3:0\]
    for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).*/
    #[inline(always)]
    pub fn awd3ch21(&self) -> AWD3CH_R {
        AWD3CH_R::new(((self.bits >> 21) & 1) != 0)
    }
    /**Bit 22 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\[3:0\]
    for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).*/
    #[inline(always)]
    pub fn awd3ch22(&self) -> AWD3CH_R {
        AWD3CH_R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AWD3CR")
            .field("awd3ch0", &self.awd3ch0())
            .field("awd3ch1", &self.awd3ch1())
            .field("awd3ch2", &self.awd3ch2())
            .field("awd3ch3", &self.awd3ch3())
            .field("awd3ch4", &self.awd3ch4())
            .field("awd3ch5", &self.awd3ch5())
            .field("awd3ch6", &self.awd3ch6())
            .field("awd3ch7", &self.awd3ch7())
            .field("awd3ch8", &self.awd3ch8())
            .field("awd3ch9", &self.awd3ch9())
            .field("awd3ch10", &self.awd3ch10())
            .field("awd3ch11", &self.awd3ch11())
            .field("awd3ch12", &self.awd3ch12())
            .field("awd3ch13", &self.awd3ch13())
            .field("awd3ch14", &self.awd3ch14())
            .field("awd3ch15", &self.awd3ch15())
            .field("awd3ch16", &self.awd3ch16())
            .field("awd3ch17", &self.awd3ch17())
            .field("awd3ch18", &self.awd3ch18())
            .field("awd3ch19", &self.awd3ch19())
            .field("awd3ch20", &self.awd3ch20())
            .field("awd3ch21", &self.awd3ch21())
            .field("awd3ch22", &self.awd3ch22())
            .finish()
    }
}
impl W {
    /**Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\[3:0\]
    for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).*/
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `AWD3CH0` field.</div>
    #[inline(always)]
    pub fn awd3ch(&mut self, n: u8) -> AWD3CH_W<AWD3CRrs> {
        #[allow(clippy::no_effect)]
        [(); 23][n as usize];
        AWD3CH_W::new(self, n)
    }
    /**Bit 0 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\[3:0\]
    for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).*/
    #[inline(always)]
    pub fn awd3ch0(&mut self) -> AWD3CH_W<AWD3CRrs> {
        AWD3CH_W::new(self, 0)
    }
    /**Bit 1 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\[3:0\]
    for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).*/
    #[inline(always)]
    pub fn awd3ch1(&mut self) -> AWD3CH_W<AWD3CRrs> {
        AWD3CH_W::new(self, 1)
    }
    /**Bit 2 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\[3:0\]
    for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).*/
    #[inline(always)]
    pub fn awd3ch2(&mut self) -> AWD3CH_W<AWD3CRrs> {
        AWD3CH_W::new(self, 2)
    }
    /**Bit 3 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\[3:0\]
    for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).*/
    #[inline(always)]
    pub fn awd3ch3(&mut self) -> AWD3CH_W<AWD3CRrs> {
        AWD3CH_W::new(self, 3)
    }
    /**Bit 4 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\[3:0\]
    for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).*/
    #[inline(always)]
    pub fn awd3ch4(&mut self) -> AWD3CH_W<AWD3CRrs> {
        AWD3CH_W::new(self, 4)
    }
    /**Bit 5 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\[3:0\]
    for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).*/
    #[inline(always)]
    pub fn awd3ch5(&mut self) -> AWD3CH_W<AWD3CRrs> {
        AWD3CH_W::new(self, 5)
    }
    /**Bit 6 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\[3:0\]
    for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).*/
    #[inline(always)]
    pub fn awd3ch6(&mut self) -> AWD3CH_W<AWD3CRrs> {
        AWD3CH_W::new(self, 6)
    }
    /**Bit 7 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\[3:0\]
    for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).*/
    #[inline(always)]
    pub fn awd3ch7(&mut self) -> AWD3CH_W<AWD3CRrs> {
        AWD3CH_W::new(self, 7)
    }
    /**Bit 8 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\[3:0\]
    for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).*/
    #[inline(always)]
    pub fn awd3ch8(&mut self) -> AWD3CH_W<AWD3CRrs> {
        AWD3CH_W::new(self, 8)
    }
    /**Bit 9 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\[3:0\]
    for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).*/
    #[inline(always)]
    pub fn awd3ch9(&mut self) -> AWD3CH_W<AWD3CRrs> {
        AWD3CH_W::new(self, 9)
    }
    /**Bit 10 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\[3:0\]
    for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).*/
    #[inline(always)]
    pub fn awd3ch10(&mut self) -> AWD3CH_W<AWD3CRrs> {
        AWD3CH_W::new(self, 10)
    }
    /**Bit 11 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\[3:0\]
    for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).*/
    #[inline(always)]
    pub fn awd3ch11(&mut self) -> AWD3CH_W<AWD3CRrs> {
        AWD3CH_W::new(self, 11)
    }
    /**Bit 12 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\[3:0\]
    for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).*/
    #[inline(always)]
    pub fn awd3ch12(&mut self) -> AWD3CH_W<AWD3CRrs> {
        AWD3CH_W::new(self, 12)
    }
    /**Bit 13 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\[3:0\]
    for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).*/
    #[inline(always)]
    pub fn awd3ch13(&mut self) -> AWD3CH_W<AWD3CRrs> {
        AWD3CH_W::new(self, 13)
    }
    /**Bit 14 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\[3:0\]
    for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).*/
    #[inline(always)]
    pub fn awd3ch14(&mut self) -> AWD3CH_W<AWD3CRrs> {
        AWD3CH_W::new(self, 14)
    }
    /**Bit 15 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\[3:0\]
    for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).*/
    #[inline(always)]
    pub fn awd3ch15(&mut self) -> AWD3CH_W<AWD3CRrs> {
        AWD3CH_W::new(self, 15)
    }
    /**Bit 16 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\[3:0\]
    for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).*/
    #[inline(always)]
    pub fn awd3ch16(&mut self) -> AWD3CH_W<AWD3CRrs> {
        AWD3CH_W::new(self, 16)
    }
    /**Bit 17 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\[3:0\]
    for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).*/
    #[inline(always)]
    pub fn awd3ch17(&mut self) -> AWD3CH_W<AWD3CRrs> {
        AWD3CH_W::new(self, 17)
    }
    /**Bit 18 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\[3:0\]
    for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).*/
    #[inline(always)]
    pub fn awd3ch18(&mut self) -> AWD3CH_W<AWD3CRrs> {
        AWD3CH_W::new(self, 18)
    }
    /**Bit 19 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\[3:0\]
    for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).*/
    #[inline(always)]
    pub fn awd3ch19(&mut self) -> AWD3CH_W<AWD3CRrs> {
        AWD3CH_W::new(self, 19)
    }
    /**Bit 20 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\[3:0\]
    for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).*/
    #[inline(always)]
    pub fn awd3ch20(&mut self) -> AWD3CH_W<AWD3CRrs> {
        AWD3CH_W::new(self, 20)
    }
    /**Bit 21 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\[3:0\]
    for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).*/
    #[inline(always)]
    pub fn awd3ch21(&mut self) -> AWD3CH_W<AWD3CRrs> {
        AWD3CH_W::new(self, 21)
    }
    /**Bit 22 - Analog watchdog channel selection These bits are set and cleared by software. They enable and select the input channels to be guarded by analog watchdog 3 (AWD3). Note: The channels selected through ADC_AWD3CR must be also configured into the ADC_CHSELR registers. Refer to SQ8\[3:0\]
    for a definition of channel selection. The software is allowed to write this bit only when ADSTART=0 (which ensures that no conversion is ongoing).*/
    #[inline(always)]
    pub fn awd3ch22(&mut self) -> AWD3CH_W<AWD3CRrs> {
        AWD3CH_W::new(self, 22)
    }
}
/**ADC Analog Watchdog 3 Configuration register

You can [`read`](crate::Reg::read) this register and get [`awd3cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`awd3cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C031.html#ADC:AWD3CR)*/
pub struct AWD3CRrs;
impl crate::RegisterSpec for AWD3CRrs {
    type Ux = u32;
}
///`read()` method returns [`awd3cr::R`](R) reader structure
impl crate::Readable for AWD3CRrs {}
///`write(|w| ..)` method takes [`awd3cr::W`](W) writer structure
impl crate::Writable for AWD3CRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets AWD3CR to value 0
impl crate::Resettable for AWD3CRrs {
    const RESET_VALUE: u32 = 0;
}
