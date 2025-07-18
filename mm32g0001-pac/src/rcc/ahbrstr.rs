#[doc = "Register `AHBRSTR` reader"]
pub type R = crate::R<AhbrstrSpec>;
#[doc = "Register `AHBRSTR` writer"]
pub type W = crate::W<AhbrstrSpec>;
#[doc = "Field `GPIOA` reader - I/O port A clock reset"]
pub type GpioaR = crate::BitReader;
#[doc = "Field `GPIOA` writer - I/O port A clock reset"]
pub type GpioaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOB` reader - I/O port B clock reset"]
pub type GpiobR = crate::BitReader;
#[doc = "Field `GPIOB` writer - I/O port B clock reset"]
pub type GpiobW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 17 - I/O port A clock reset"]
    #[inline(always)]
    pub fn gpioa(&self) -> GpioaR {
        GpioaR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - I/O port B clock reset"]
    #[inline(always)]
    pub fn gpiob(&self) -> GpiobR {
        GpiobR::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 17 - I/O port A clock reset"]
    #[inline(always)]
    pub fn gpioa(&mut self) -> GpioaW<AhbrstrSpec> {
        GpioaW::new(self, 17)
    }
    #[doc = "Bit 18 - I/O port B clock reset"]
    #[inline(always)]
    pub fn gpiob(&mut self) -> GpiobW<AhbrstrSpec> {
        GpiobW::new(self, 18)
    }
}
#[doc = "Advanced High Performance Bus Reset Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ahbrstr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahbrstr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AhbrstrSpec;
impl crate::RegisterSpec for AhbrstrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahbrstr::R`](R) reader structure"]
impl crate::Readable for AhbrstrSpec {}
#[doc = "`write(|w| ..)` method takes [`ahbrstr::W`](W) writer structure"]
impl crate::Writable for AhbrstrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AHBRSTR to value 0"]
impl crate::Resettable for AhbrstrSpec {}
