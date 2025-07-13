#[doc = "Register `BRR` writer"]
pub type W = crate::W<BrrSpec>;
#[doc = "Field `BR0` writer - Port x Reset bit 0"]
pub type Br0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BR1` writer - Port x Reset bit 1"]
pub type Br1W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Port x Reset bit 0"]
    #[inline(always)]
    pub fn br0(&mut self) -> Br0W<BrrSpec> {
        Br0W::new(self, 0)
    }
    #[doc = "Bit 1 - Port x Reset bit 1"]
    #[inline(always)]
    pub fn br1(&mut self) -> Br1W<BrrSpec> {
        Br1W::new(self, 1)
    }
}
#[doc = "bit reset register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`brr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BrrSpec;
impl crate::RegisterSpec for BrrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`brr::W`](W) writer structure"]
impl crate::Writable for BrrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BRR to value 0"]
impl crate::Resettable for BrrSpec {}
