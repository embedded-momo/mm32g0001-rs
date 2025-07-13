#[doc = "Register `APB1ENR` reader"]
pub type R = crate::R<Apb1enrSpec>;
#[doc = "Register `APB1ENR` writer"]
pub type W = crate::W<Apb1enrSpec>;
#[doc = "Field `TIM3` reader - TIM3 enable"]
pub type Tim3R = crate::BitReader;
#[doc = "Field `TIM3` writer - TIM3 enable"]
pub type Tim3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM1` reader - TIM1 enable"]
pub type Tim1R = crate::BitReader;
#[doc = "Field `TIM1` writer - TIM1 enable"]
pub type Tim1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM14` reader - TIM14 enable"]
pub type Tim14R = crate::BitReader;
#[doc = "Field `TIM14` writer - TIM14 enable"]
pub type Tim14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC1` reader - ADC1 enable"]
pub type Adc1R = crate::BitReader;
#[doc = "Field `ADC1` writer - ADC1 enable"]
pub type Adc1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI1` reader - SPI1 enable"]
pub type Spi1R = crate::BitReader;
#[doc = "Field `SPI1` writer - SPI1 enable"]
pub type Spi1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART1` reader - UART1 enable"]
pub type Usart1R = crate::BitReader;
#[doc = "Field `USART1` writer - UART1 enable"]
pub type Usart1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART2` reader - UART2 enable"]
pub type Usart2R = crate::BitReader;
#[doc = "Field `USART2` writer - UART2 enable"]
pub type Usart2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C1` reader - I2C1 enable"]
pub type I2c1R = crate::BitReader;
#[doc = "Field `I2C1` writer - I2C1 enable"]
pub type I2c1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWR` reader - Power interface enable"]
pub type PwrR = crate::BitReader;
#[doc = "Field `PWR` writer - Power interface enable"]
pub type PwrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBGMCU` reader - DBGMCU enable"]
pub type DbgmcuR = crate::BitReader;
#[doc = "Field `DBGMCU` writer - DBGMCU enable"]
pub type DbgmcuW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYSCFG` reader - SYSCFG enable"]
pub type SyscfgR = crate::BitReader;
#[doc = "Field `SYSCFG` writer - SYSCFG enable"]
pub type SyscfgW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - TIM3 enable"]
    #[inline(always)]
    pub fn tim3(&self) -> Tim3R {
        Tim3R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TIM1 enable"]
    #[inline(always)]
    pub fn tim1(&self) -> Tim1R {
        Tim1R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TIM14 enable"]
    #[inline(always)]
    pub fn tim14(&self) -> Tim14R {
        Tim14R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 9 - ADC1 enable"]
    #[inline(always)]
    pub fn adc1(&self) -> Adc1R {
        Adc1R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12 - SPI1 enable"]
    #[inline(always)]
    pub fn spi1(&self) -> Spi1R {
        Spi1R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16 - UART1 enable"]
    #[inline(always)]
    pub fn usart1(&self) -> Usart1R {
        Usart1R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - UART2 enable"]
    #[inline(always)]
    pub fn usart2(&self) -> Usart2R {
        Usart2R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 21 - I2C1 enable"]
    #[inline(always)]
    pub fn i2c1(&self) -> I2c1R {
        I2c1R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 28 - Power interface enable"]
    #[inline(always)]
    pub fn pwr(&self) -> PwrR {
        PwrR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - DBGMCU enable"]
    #[inline(always)]
    pub fn dbgmcu(&self) -> DbgmcuR {
        DbgmcuR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - SYSCFG enable"]
    #[inline(always)]
    pub fn syscfg(&self) -> SyscfgR {
        SyscfgR::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - TIM3 enable"]
    #[inline(always)]
    pub fn tim3(&mut self) -> Tim3W<Apb1enrSpec> {
        Tim3W::new(self, 1)
    }
    #[doc = "Bit 2 - TIM1 enable"]
    #[inline(always)]
    pub fn tim1(&mut self) -> Tim1W<Apb1enrSpec> {
        Tim1W::new(self, 2)
    }
    #[doc = "Bit 3 - TIM14 enable"]
    #[inline(always)]
    pub fn tim14(&mut self) -> Tim14W<Apb1enrSpec> {
        Tim14W::new(self, 3)
    }
    #[doc = "Bit 9 - ADC1 enable"]
    #[inline(always)]
    pub fn adc1(&mut self) -> Adc1W<Apb1enrSpec> {
        Adc1W::new(self, 9)
    }
    #[doc = "Bit 12 - SPI1 enable"]
    #[inline(always)]
    pub fn spi1(&mut self) -> Spi1W<Apb1enrSpec> {
        Spi1W::new(self, 12)
    }
    #[doc = "Bit 16 - UART1 enable"]
    #[inline(always)]
    pub fn usart1(&mut self) -> Usart1W<Apb1enrSpec> {
        Usart1W::new(self, 16)
    }
    #[doc = "Bit 17 - UART2 enable"]
    #[inline(always)]
    pub fn usart2(&mut self) -> Usart2W<Apb1enrSpec> {
        Usart2W::new(self, 17)
    }
    #[doc = "Bit 21 - I2C1 enable"]
    #[inline(always)]
    pub fn i2c1(&mut self) -> I2c1W<Apb1enrSpec> {
        I2c1W::new(self, 21)
    }
    #[doc = "Bit 28 - Power interface enable"]
    #[inline(always)]
    pub fn pwr(&mut self) -> PwrW<Apb1enrSpec> {
        PwrW::new(self, 28)
    }
    #[doc = "Bit 29 - DBGMCU enable"]
    #[inline(always)]
    pub fn dbgmcu(&mut self) -> DbgmcuW<Apb1enrSpec> {
        DbgmcuW::new(self, 29)
    }
    #[doc = "Bit 30 - SYSCFG enable"]
    #[inline(always)]
    pub fn syscfg(&mut self) -> SyscfgW<Apb1enrSpec> {
        SyscfgW::new(self, 30)
    }
}
#[doc = "Advanced Peripheral Bus 1 Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`apb1enr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1enr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Apb1enrSpec;
impl crate::RegisterSpec for Apb1enrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb1enr::R`](R) reader structure"]
impl crate::Readable for Apb1enrSpec {}
#[doc = "`write(|w| ..)` method takes [`apb1enr::W`](W) writer structure"]
impl crate::Writable for Apb1enrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets APB1ENR to value 0"]
impl crate::Resettable for Apb1enrSpec {}
