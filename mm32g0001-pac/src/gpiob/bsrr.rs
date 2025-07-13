#[doc = "Register `BSRR` writer"]
pub type W = crate::W<BsrrSpec>;
#[doc = "Field `BS0` writer - Port x Set bit 0"]
pub type Bs0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BS1` writer - Port x Set bit 1"]
pub type Bs1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BR0` writer - Port x Reset bit 0"]
pub type Br0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BR1` writer - Port x Reset bit 1"]
pub type Br1W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Port x Set bit 0"]
    #[inline(always)]
    pub fn bs0(&mut self) -> Bs0W<BsrrSpec> {
        Bs0W::new(self, 0)
    }
    #[doc = "Bit 1 - Port x Set bit 1"]
    #[inline(always)]
    pub fn bs1(&mut self) -> Bs1W<BsrrSpec> {
        Bs1W::new(self, 1)
    }
    #[doc = "Bit 16 - Port x Reset bit 0"]
    #[inline(always)]
    pub fn br0(&mut self) -> Br0W<BsrrSpec> {
        Br0W::new(self, 16)
    }
    #[doc = "Bit 17 - Port x Reset bit 1"]
    #[inline(always)]
    pub fn br1(&mut self) -> Br1W<BsrrSpec> {
        Br1W::new(self, 17)
    }
}
#[doc = "bit set/reset register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bsrr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BsrrSpec;
impl crate::RegisterSpec for BsrrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`bsrr::W`](W) writer structure"]
impl crate::Writable for BsrrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BSRR to value 0"]
impl crate::Resettable for BsrrSpec {}
