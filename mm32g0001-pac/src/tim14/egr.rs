#[doc = "Register `EGR` writer"]
pub type W = crate::W<EgrSpec>;
#[doc = "Field `UG` writer - Update generation"]
pub type UgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC1G` writer - Compare 1 generation"]
pub type Cc1gW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Update generation"]
    #[inline(always)]
    pub fn ug(&mut self) -> UgW<EgrSpec> {
        UgW::new(self, 0)
    }
    #[doc = "Bit 1 - Compare 1 generation"]
    #[inline(always)]
    pub fn cc1g(&mut self) -> Cc1gW<EgrSpec> {
        Cc1gW::new(self, 1)
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
