#[doc = "Register `CNT` reader"]
pub type R = crate::R<CntSpec>;
#[doc = "Register `CNT` writer"]
pub type W = crate::W<CntSpec>;
#[doc = "Field `PS` reader - Watchdog prescaler counter value"]
pub type PsR = crate::FieldReader;
#[doc = "Field `PS` writer - Watchdog prescaler counter value"]
pub type PsW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CNT` reader - Current value of watchdog counter"]
pub type CntR = crate::FieldReader<u16>;
#[doc = "Field `CNT` writer - Current value of watchdog counter"]
pub type CntW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:7 - Watchdog prescaler counter value"]
    #[inline(always)]
    pub fn ps(&self) -> PsR {
        PsR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:18 - Current value of watchdog counter"]
    #[inline(always)]
    pub fn cnt(&self) -> CntR {
        CntR::new(((self.bits >> 8) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:7 - Watchdog prescaler counter value"]
    #[inline(always)]
    pub fn ps(&mut self) -> PsW<CntSpec> {
        PsW::new(self, 0)
    }
    #[doc = "Bits 8:18 - Current value of watchdog counter"]
    #[inline(always)]
    pub fn cnt(&mut self) -> CntW<CntSpec> {
        CntW::new(self, 8)
    }
}
#[doc = "Counter\n\nYou can [`read`](crate::Reg::read) this register and get [`cnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CntSpec;
impl crate::RegisterSpec for CntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cnt::R`](R) reader structure"]
impl crate::Readable for CntSpec {}
#[doc = "`write(|w| ..)` method takes [`cnt::W`](W) writer structure"]
impl crate::Writable for CntSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CNT to value 0"]
impl crate::Resettable for CntSpec {}
