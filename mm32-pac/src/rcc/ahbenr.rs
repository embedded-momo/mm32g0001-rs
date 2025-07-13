#[doc = "Register `AHBENR` reader"]
pub type R = crate::R<AhbenrSpec>;
#[doc = "Register `AHBENR` writer"]
pub type W = crate::W<AhbenrSpec>;
#[doc = "Field `SRAM` reader - SRAM interface clock enable"]
pub type SramR = crate::BitReader;
#[doc = "Field `SRAM` writer - SRAM interface clock enable"]
pub type SramW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLASH` reader - FLASH clock enable"]
pub type FlashR = crate::BitReader;
#[doc = "Field `FLASH` writer - FLASH clock enable"]
pub type FlashW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRC` reader - CRC clock enable"]
pub type CrcR = crate::BitReader;
#[doc = "Field `CRC` writer - CRC clock enable"]
pub type CrcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOA` reader - I/O port A clock enable"]
pub type GpioaR = crate::BitReader;
#[doc = "Field `GPIOA` writer - I/O port A clock enable"]
pub type GpioaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOB` reader - I/O port B clock enable"]
pub type GpiobR = crate::BitReader;
#[doc = "Field `GPIOB` writer - I/O port B clock enable"]
pub type GpiobW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 2 - SRAM interface clock enable"]
    #[inline(always)]
    pub fn sram(&self) -> SramR {
        SramR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - FLASH clock enable"]
    #[inline(always)]
    pub fn flash(&self) -> FlashR {
        FlashR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - CRC clock enable"]
    #[inline(always)]
    pub fn crc(&self) -> CrcR {
        CrcR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 17 - I/O port A clock enable"]
    #[inline(always)]
    pub fn gpioa(&self) -> GpioaR {
        GpioaR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - I/O port B clock enable"]
    #[inline(always)]
    pub fn gpiob(&self) -> GpiobR {
        GpiobR::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - SRAM interface clock enable"]
    #[inline(always)]
    pub fn sram(&mut self) -> SramW<AhbenrSpec> {
        SramW::new(self, 2)
    }
    #[doc = "Bit 4 - FLASH clock enable"]
    #[inline(always)]
    pub fn flash(&mut self) -> FlashW<AhbenrSpec> {
        FlashW::new(self, 4)
    }
    #[doc = "Bit 6 - CRC clock enable"]
    #[inline(always)]
    pub fn crc(&mut self) -> CrcW<AhbenrSpec> {
        CrcW::new(self, 6)
    }
    #[doc = "Bit 17 - I/O port A clock enable"]
    #[inline(always)]
    pub fn gpioa(&mut self) -> GpioaW<AhbenrSpec> {
        GpioaW::new(self, 17)
    }
    #[doc = "Bit 18 - I/O port B clock enable"]
    #[inline(always)]
    pub fn gpiob(&mut self) -> GpiobW<AhbenrSpec> {
        GpiobW::new(self, 18)
    }
}
#[doc = "Advanced High Performance Bus Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ahbenr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahbenr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AhbenrSpec;
impl crate::RegisterSpec for AhbenrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahbenr::R`](R) reader structure"]
impl crate::Readable for AhbenrSpec {}
#[doc = "`write(|w| ..)` method takes [`ahbenr::W`](W) writer structure"]
impl crate::Writable for AhbenrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AHBENR to value 0x14"]
impl crate::Resettable for AhbenrSpec {
    const RESET_VALUE: u32 = 0x14;
}
