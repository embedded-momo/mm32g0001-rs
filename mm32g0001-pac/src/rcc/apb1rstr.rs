#[doc = "Register `APB1RSTR` reader"]
pub type R = crate::R<Apb1rstrSpec>;
#[doc = "Register `APB1RSTR` writer"]
pub type W = crate::W<Apb1rstrSpec>;
#[doc = "Field `TIM3` reader - TIM3 reset"]
pub type Tim3R = crate::BitReader;
#[doc = "Field `TIM3` writer - TIM3 reset"]
pub type Tim3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM1` reader - TIM1 reset"]
pub type Tim1R = crate::BitReader;
#[doc = "Field `TIM1` writer - TIM1 reset"]
pub type Tim1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM14` reader - TIM14 reset"]
pub type Tim14R = crate::BitReader;
#[doc = "Field `TIM14` writer - TIM14 reset"]
pub type Tim14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC1` reader - ADC1 reset"]
pub type Adc1R = crate::BitReader;
#[doc = "Field `ADC1` writer - ADC1 reset"]
pub type Adc1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI1` reader - SPI1 reset"]
pub type Spi1R = crate::BitReader;
#[doc = "Field `SPI1` writer - SPI1 reset"]
pub type Spi1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART1` reader - UART1 reset"]
pub type Usart1R = crate::BitReader;
#[doc = "Field `USART1` writer - UART1 reset"]
pub type Usart1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART2` reader - UART2 reset"]
pub type Usart2R = crate::BitReader;
#[doc = "Field `USART2` writer - UART2 reset"]
pub type Usart2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C1` reader - I2C1 reset"]
pub type I2c1R = crate::BitReader;
#[doc = "Field `I2C1` writer - I2C1 reset"]
pub type I2c1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWR` reader - Power interface reset"]
pub type PwrR = crate::BitReader;
#[doc = "Field `PWR` writer - Power interface reset"]
pub type PwrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBGMCU` reader - DBGMCU reset"]
pub type DbgmcuR = crate::BitReader;
#[doc = "Field `DBGMCU` writer - DBGMCU reset"]
pub type DbgmcuW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYSCFG` reader - SYSCFG reset"]
pub type SyscfgR = crate::BitReader;
#[doc = "Field `SYSCFG` writer - SYSCFG reset"]
pub type SyscfgW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - TIM3 reset"]
    #[inline(always)]
    pub fn tim3(&self) -> Tim3R {
        Tim3R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TIM1 reset"]
    #[inline(always)]
    pub fn tim1(&self) -> Tim1R {
        Tim1R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TIM14 reset"]
    #[inline(always)]
    pub fn tim14(&self) -> Tim14R {
        Tim14R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 9 - ADC1 reset"]
    #[inline(always)]
    pub fn adc1(&self) -> Adc1R {
        Adc1R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12 - SPI1 reset"]
    #[inline(always)]
    pub fn spi1(&self) -> Spi1R {
        Spi1R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16 - UART1 reset"]
    #[inline(always)]
    pub fn usart1(&self) -> Usart1R {
        Usart1R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - UART2 reset"]
    #[inline(always)]
    pub fn usart2(&self) -> Usart2R {
        Usart2R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 21 - I2C1 reset"]
    #[inline(always)]
    pub fn i2c1(&self) -> I2c1R {
        I2c1R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 28 - Power interface reset"]
    #[inline(always)]
    pub fn pwr(&self) -> PwrR {
        PwrR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - DBGMCU reset"]
    #[inline(always)]
    pub fn dbgmcu(&self) -> DbgmcuR {
        DbgmcuR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - SYSCFG reset"]
    #[inline(always)]
    pub fn syscfg(&self) -> SyscfgR {
        SyscfgR::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - TIM3 reset"]
    #[inline(always)]
    pub fn tim3(&mut self) -> Tim3W<Apb1rstrSpec> {
        Tim3W::new(self, 1)
    }
    #[doc = "Bit 2 - TIM1 reset"]
    #[inline(always)]
    pub fn tim1(&mut self) -> Tim1W<Apb1rstrSpec> {
        Tim1W::new(self, 2)
    }
    #[doc = "Bit 3 - TIM14 reset"]
    #[inline(always)]
    pub fn tim14(&mut self) -> Tim14W<Apb1rstrSpec> {
        Tim14W::new(self, 3)
    }
    #[doc = "Bit 9 - ADC1 reset"]
    #[inline(always)]
    pub fn adc1(&mut self) -> Adc1W<Apb1rstrSpec> {
        Adc1W::new(self, 9)
    }
    #[doc = "Bit 12 - SPI1 reset"]
    #[inline(always)]
    pub fn spi1(&mut self) -> Spi1W<Apb1rstrSpec> {
        Spi1W::new(self, 12)
    }
    #[doc = "Bit 16 - UART1 reset"]
    #[inline(always)]
    pub fn usart1(&mut self) -> Usart1W<Apb1rstrSpec> {
        Usart1W::new(self, 16)
    }
    #[doc = "Bit 17 - UART2 reset"]
    #[inline(always)]
    pub fn usart2(&mut self) -> Usart2W<Apb1rstrSpec> {
        Usart2W::new(self, 17)
    }
    #[doc = "Bit 21 - I2C1 reset"]
    #[inline(always)]
    pub fn i2c1(&mut self) -> I2c1W<Apb1rstrSpec> {
        I2c1W::new(self, 21)
    }
    #[doc = "Bit 28 - Power interface reset"]
    #[inline(always)]
    pub fn pwr(&mut self) -> PwrW<Apb1rstrSpec> {
        PwrW::new(self, 28)
    }
    #[doc = "Bit 29 - DBGMCU reset"]
    #[inline(always)]
    pub fn dbgmcu(&mut self) -> DbgmcuW<Apb1rstrSpec> {
        DbgmcuW::new(self, 29)
    }
    #[doc = "Bit 30 - SYSCFG reset"]
    #[inline(always)]
    pub fn syscfg(&mut self) -> SyscfgW<Apb1rstrSpec> {
        SyscfgW::new(self, 30)
    }
}
#[doc = "Advanced Peripheral Bus 1 Reset Register\n\nYou can [`read`](crate::Reg::read) this register and get [`apb1rstr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1rstr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Apb1rstrSpec;
impl crate::RegisterSpec for Apb1rstrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb1rstr::R`](R) reader structure"]
impl crate::Readable for Apb1rstrSpec {}
#[doc = "`write(|w| ..)` method takes [`apb1rstr::W`](W) writer structure"]
impl crate::Writable for Apb1rstrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets APB1RSTR to value 0"]
impl crate::Resettable for Apb1rstrSpec {}
