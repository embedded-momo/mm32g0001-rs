#[doc = "Register `RTSR` reader"]
pub type R = crate::R<RtsrSpec>;
#[doc = "Register `RTSR` writer"]
pub type W = crate::W<RtsrSpec>;
#[doc = "Field `TR0` reader - Rising trigger event configuration bit of line 0"]
pub type Tr0R = crate::BitReader;
#[doc = "Field `TR0` writer - Rising trigger event configuration bit of line 0"]
pub type Tr0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TR1` reader - Rising trigger event configuration bit of line 1"]
pub type Tr1R = crate::BitReader;
#[doc = "Field `TR1` writer - Rising trigger event configuration bit of line 1"]
pub type Tr1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TR2` reader - Rising trigger event configuration bit of line 2"]
pub type Tr2R = crate::BitReader;
#[doc = "Field `TR2` writer - Rising trigger event configuration bit of line 2"]
pub type Tr2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TR3` reader - Rising trigger event configuration bit of line 3"]
pub type Tr3R = crate::BitReader;
#[doc = "Field `TR3` writer - Rising trigger event configuration bit of line 3"]
pub type Tr3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TR4` reader - Rising trigger event configuration bit of line 4"]
pub type Tr4R = crate::BitReader;
#[doc = "Field `TR4` writer - Rising trigger event configuration bit of line 4"]
pub type Tr4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TR5` reader - Rising trigger event configuration bit of line 5"]
pub type Tr5R = crate::BitReader;
#[doc = "Field `TR5` writer - Rising trigger event configuration bit of line 5"]
pub type Tr5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TR6` reader - Rising trigger event configuration bit of line 6"]
pub type Tr6R = crate::BitReader;
#[doc = "Field `TR6` writer - Rising trigger event configuration bit of line 6"]
pub type Tr6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TR7` reader - Rising trigger event configuration bit of line 7"]
pub type Tr7R = crate::BitReader;
#[doc = "Field `TR7` writer - Rising trigger event configuration bit of line 7"]
pub type Tr7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TR8` reader - Rising trigger event configuration bit of line 8"]
pub type Tr8R = crate::BitReader;
#[doc = "Field `TR8` writer - Rising trigger event configuration bit of line 8"]
pub type Tr8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TR9` reader - Rising trigger event configuration bit of line 9"]
pub type Tr9R = crate::BitReader;
#[doc = "Field `TR9` writer - Rising trigger event configuration bit of line 9"]
pub type Tr9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TR10` reader - Rising trigger event configuration bit of line 10"]
pub type Tr10R = crate::BitReader;
#[doc = "Field `TR10` writer - Rising trigger event configuration bit of line 10"]
pub type Tr10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TR11` reader - Rising trigger event configuration bit of line 11"]
pub type Tr11R = crate::BitReader;
#[doc = "Field `TR11` writer - Rising trigger event configuration bit of line 11"]
pub type Tr11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TR12` reader - Rising trigger event configuration bit of line 12"]
pub type Tr12R = crate::BitReader;
#[doc = "Field `TR12` writer - Rising trigger event configuration bit of line 12"]
pub type Tr12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TR13` reader - Rising trigger event configuration bit of line 13"]
pub type Tr13R = crate::BitReader;
#[doc = "Field `TR13` writer - Rising trigger event configuration bit of line 13"]
pub type Tr13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TR14` reader - Rising trigger event configuration bit of line 14"]
pub type Tr14R = crate::BitReader;
#[doc = "Field `TR14` writer - Rising trigger event configuration bit of line 14"]
pub type Tr14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TR15` reader - Rising trigger event configuration bit of line 15"]
pub type Tr15R = crate::BitReader;
#[doc = "Field `TR15` writer - Rising trigger event configuration bit of line 15"]
pub type Tr15W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TR16` reader - Rising trigger event configuration bit of line 16"]
pub type Tr16R = crate::BitReader;
#[doc = "Field `TR16` writer - Rising trigger event configuration bit of line 16"]
pub type Tr16W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TR17` reader - Rising trigger event configuration bit of line 17"]
pub type Tr17R = crate::BitReader;
#[doc = "Field `TR17` writer - Rising trigger event configuration bit of line 17"]
pub type Tr17W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Rising trigger event configuration bit of line 0"]
    #[inline(always)]
    pub fn tr0(&self) -> Tr0R {
        Tr0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Rising trigger event configuration bit of line 1"]
    #[inline(always)]
    pub fn tr1(&self) -> Tr1R {
        Tr1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Rising trigger event configuration bit of line 2"]
    #[inline(always)]
    pub fn tr2(&self) -> Tr2R {
        Tr2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Rising trigger event configuration bit of line 3"]
    #[inline(always)]
    pub fn tr3(&self) -> Tr3R {
        Tr3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Rising trigger event configuration bit of line 4"]
    #[inline(always)]
    pub fn tr4(&self) -> Tr4R {
        Tr4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Rising trigger event configuration bit of line 5"]
    #[inline(always)]
    pub fn tr5(&self) -> Tr5R {
        Tr5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Rising trigger event configuration bit of line 6"]
    #[inline(always)]
    pub fn tr6(&self) -> Tr6R {
        Tr6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Rising trigger event configuration bit of line 7"]
    #[inline(always)]
    pub fn tr7(&self) -> Tr7R {
        Tr7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Rising trigger event configuration bit of line 8"]
    #[inline(always)]
    pub fn tr8(&self) -> Tr8R {
        Tr8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Rising trigger event configuration bit of line 9"]
    #[inline(always)]
    pub fn tr9(&self) -> Tr9R {
        Tr9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Rising trigger event configuration bit of line 10"]
    #[inline(always)]
    pub fn tr10(&self) -> Tr10R {
        Tr10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Rising trigger event configuration bit of line 11"]
    #[inline(always)]
    pub fn tr11(&self) -> Tr11R {
        Tr11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Rising trigger event configuration bit of line 12"]
    #[inline(always)]
    pub fn tr12(&self) -> Tr12R {
        Tr12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Rising trigger event configuration bit of line 13"]
    #[inline(always)]
    pub fn tr13(&self) -> Tr13R {
        Tr13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Rising trigger event configuration bit of line 14"]
    #[inline(always)]
    pub fn tr14(&self) -> Tr14R {
        Tr14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Rising trigger event configuration bit of line 15"]
    #[inline(always)]
    pub fn tr15(&self) -> Tr15R {
        Tr15R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Rising trigger event configuration bit of line 16"]
    #[inline(always)]
    pub fn tr16(&self) -> Tr16R {
        Tr16R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Rising trigger event configuration bit of line 17"]
    #[inline(always)]
    pub fn tr17(&self) -> Tr17R {
        Tr17R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Rising trigger event configuration bit of line 0"]
    #[inline(always)]
    pub fn tr0(&mut self) -> Tr0W<RtsrSpec> {
        Tr0W::new(self, 0)
    }
    #[doc = "Bit 1 - Rising trigger event configuration bit of line 1"]
    #[inline(always)]
    pub fn tr1(&mut self) -> Tr1W<RtsrSpec> {
        Tr1W::new(self, 1)
    }
    #[doc = "Bit 2 - Rising trigger event configuration bit of line 2"]
    #[inline(always)]
    pub fn tr2(&mut self) -> Tr2W<RtsrSpec> {
        Tr2W::new(self, 2)
    }
    #[doc = "Bit 3 - Rising trigger event configuration bit of line 3"]
    #[inline(always)]
    pub fn tr3(&mut self) -> Tr3W<RtsrSpec> {
        Tr3W::new(self, 3)
    }
    #[doc = "Bit 4 - Rising trigger event configuration bit of line 4"]
    #[inline(always)]
    pub fn tr4(&mut self) -> Tr4W<RtsrSpec> {
        Tr4W::new(self, 4)
    }
    #[doc = "Bit 5 - Rising trigger event configuration bit of line 5"]
    #[inline(always)]
    pub fn tr5(&mut self) -> Tr5W<RtsrSpec> {
        Tr5W::new(self, 5)
    }
    #[doc = "Bit 6 - Rising trigger event configuration bit of line 6"]
    #[inline(always)]
    pub fn tr6(&mut self) -> Tr6W<RtsrSpec> {
        Tr6W::new(self, 6)
    }
    #[doc = "Bit 7 - Rising trigger event configuration bit of line 7"]
    #[inline(always)]
    pub fn tr7(&mut self) -> Tr7W<RtsrSpec> {
        Tr7W::new(self, 7)
    }
    #[doc = "Bit 8 - Rising trigger event configuration bit of line 8"]
    #[inline(always)]
    pub fn tr8(&mut self) -> Tr8W<RtsrSpec> {
        Tr8W::new(self, 8)
    }
    #[doc = "Bit 9 - Rising trigger event configuration bit of line 9"]
    #[inline(always)]
    pub fn tr9(&mut self) -> Tr9W<RtsrSpec> {
        Tr9W::new(self, 9)
    }
    #[doc = "Bit 10 - Rising trigger event configuration bit of line 10"]
    #[inline(always)]
    pub fn tr10(&mut self) -> Tr10W<RtsrSpec> {
        Tr10W::new(self, 10)
    }
    #[doc = "Bit 11 - Rising trigger event configuration bit of line 11"]
    #[inline(always)]
    pub fn tr11(&mut self) -> Tr11W<RtsrSpec> {
        Tr11W::new(self, 11)
    }
    #[doc = "Bit 12 - Rising trigger event configuration bit of line 12"]
    #[inline(always)]
    pub fn tr12(&mut self) -> Tr12W<RtsrSpec> {
        Tr12W::new(self, 12)
    }
    #[doc = "Bit 13 - Rising trigger event configuration bit of line 13"]
    #[inline(always)]
    pub fn tr13(&mut self) -> Tr13W<RtsrSpec> {
        Tr13W::new(self, 13)
    }
    #[doc = "Bit 14 - Rising trigger event configuration bit of line 14"]
    #[inline(always)]
    pub fn tr14(&mut self) -> Tr14W<RtsrSpec> {
        Tr14W::new(self, 14)
    }
    #[doc = "Bit 15 - Rising trigger event configuration bit of line 15"]
    #[inline(always)]
    pub fn tr15(&mut self) -> Tr15W<RtsrSpec> {
        Tr15W::new(self, 15)
    }
    #[doc = "Bit 16 - Rising trigger event configuration bit of line 16"]
    #[inline(always)]
    pub fn tr16(&mut self) -> Tr16W<RtsrSpec> {
        Tr16W::new(self, 16)
    }
    #[doc = "Bit 17 - Rising trigger event configuration bit of line 17"]
    #[inline(always)]
    pub fn tr17(&mut self) -> Tr17W<RtsrSpec> {
        Tr17W::new(self, 17)
    }
}
#[doc = "Rising trigger selection register\n\nYou can [`read`](crate::Reg::read) this register and get [`rtsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RtsrSpec;
impl crate::RegisterSpec for RtsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rtsr::R`](R) reader structure"]
impl crate::Readable for RtsrSpec {}
#[doc = "`write(|w| ..)` method takes [`rtsr::W`](W) writer structure"]
impl crate::Writable for RtsrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RTSR to value 0"]
impl crate::Resettable for RtsrSpec {}
