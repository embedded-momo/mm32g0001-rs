#[doc = "Register `EGR` writer"]
pub type W = crate::W<EgrSpec>;
#[doc = "Field `UG` writer - Update generation"]
pub type UgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC1G` writer - compare 1 generation"]
pub type Cc1gW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC2G` writer - compare 2 generation"]
pub type Cc2gW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC3G` writer - compare 3 generation"]
pub type Cc3gW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC4G` writer - compare 4 generation"]
pub type Cc4gW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TG` writer - Trigger generation"]
pub type TgW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Update generation"]
    #[inline(always)]
    pub fn ug(&mut self) -> UgW<EgrSpec> {
        UgW::new(self, 0)
    }
    #[doc = "Bit 1 - compare 1 generation"]
    #[inline(always)]
    pub fn cc1g(&mut self) -> Cc1gW<EgrSpec> {
        Cc1gW::new(self, 1)
    }
    #[doc = "Bit 2 - compare 2 generation"]
    #[inline(always)]
    pub fn cc2g(&mut self) -> Cc2gW<EgrSpec> {
        Cc2gW::new(self, 2)
    }
    #[doc = "Bit 3 - compare 3 generation"]
    #[inline(always)]
    pub fn cc3g(&mut self) -> Cc3gW<EgrSpec> {
        Cc3gW::new(self, 3)
    }
    #[doc = "Bit 4 - compare 4 generation"]
    #[inline(always)]
    pub fn cc4g(&mut self) -> Cc4gW<EgrSpec> {
        Cc4gW::new(self, 4)
    }
    #[doc = "Bit 6 - Trigger generation"]
    #[inline(always)]
    pub fn tg(&mut self) -> TgW<EgrSpec> {
        TgW::new(self, 6)
    }
}
#[doc = "event generation register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`egr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EgrSpec;
impl crate::RegisterSpec for EgrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`egr::W`](W) writer structure"]
impl crate::Writable for EgrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EGR to value 0"]
impl crate::Resettable for EgrSpec {}
