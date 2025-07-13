#[doc = "Register `CR` reader"]
pub type R = crate::R<CrSpec>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CrSpec>;
#[doc = "Field `HSION` reader - Internal high-speed clock enable"]
pub type HsionR = crate::BitReader;
#[doc = "Field `HSION` writer - Internal high-speed clock enable"]
pub type HsionW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSIRDY` reader - Internal high-speed clock ready flag"]
pub type HsirdyR = crate::BitReader;
#[doc = "Field `HSIRDY` writer - Internal high-speed clock ready flag"]
pub type HsirdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSEON` reader - Internal high-speed clock enable"]
pub type HseonR = crate::BitReader;
#[doc = "Field `HSEON` writer - Internal high-speed clock enable"]
pub type HseonW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSERDY` reader - External high-speed clock ready flag"]
pub type HserdyR = crate::BitReader;
#[doc = "Field `HSERDY` writer - External high-speed clock ready flag"]
pub type HserdyW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Internal high-speed clock enable"]
    #[inline(always)]
    pub fn hsion(&self) -> HsionR {
        HsionR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Internal high-speed clock ready flag"]
    #[inline(always)]
    pub fn hsirdy(&self) -> HsirdyR {
        HsirdyR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 16 - Internal high-speed clock enable"]
    #[inline(always)]
    pub fn hseon(&self) -> HseonR {
        HseonR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - External high-speed clock ready flag"]
    #[inline(always)]
    pub fn hserdy(&self) -> HserdyR {
        HserdyR::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Internal high-speed clock enable"]
    #[inline(always)]
    pub fn hsion(&mut self) -> HsionW<CrSpec> {
        HsionW::new(self, 0)
    }
    #[doc = "Bit 1 - Internal high-speed clock ready flag"]
    #[inline(always)]
    pub fn hsirdy(&mut self) -> HsirdyW<CrSpec> {
        HsirdyW::new(self, 1)
    }
    #[doc = "Bit 16 - Internal high-speed clock enable"]
    #[inline(always)]
    pub fn hseon(&mut self) -> HseonW<CrSpec> {
        HseonW::new(self, 16)
    }
    #[doc = "Bit 17 - External high-speed clock ready flag"]
    #[inline(always)]
    pub fn hserdy(&mut self) -> HserdyW<CrSpec> {
        HserdyW::new(self, 17)
    }
}
#[doc = "Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrSpec;
impl crate::RegisterSpec for CrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr::R`](R) reader structure"]
impl crate::Readable for CrSpec {}
#[doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"]
impl crate::Writable for CrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR to value 0x01"]
impl crate::Resettable for CrSpec {
    const RESET_VALUE: u32 = 0x01;
}
