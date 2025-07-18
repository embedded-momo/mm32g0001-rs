#[doc = "Register `EMR` reader"]
pub type R = crate::R<EmrSpec>;
#[doc = "Register `EMR` writer"]
pub type W = crate::W<EmrSpec>;
#[doc = "Field `EMR0` reader - Event Mask on line 0"]
pub type Emr0R = crate::BitReader;
#[doc = "Field `EMR0` writer - Event Mask on line 0"]
pub type Emr0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EMR1` reader - Event Mask on line 1"]
pub type Emr1R = crate::BitReader;
#[doc = "Field `EMR1` writer - Event Mask on line 1"]
pub type Emr1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EMR2` reader - Event Mask on line 2"]
pub type Emr2R = crate::BitReader;
#[doc = "Field `EMR2` writer - Event Mask on line 2"]
pub type Emr2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EMR3` reader - Event Mask on line 3"]
pub type Emr3R = crate::BitReader;
#[doc = "Field `EMR3` writer - Event Mask on line 3"]
pub type Emr3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EMR4` reader - Event Mask on line 4"]
pub type Emr4R = crate::BitReader;
#[doc = "Field `EMR4` writer - Event Mask on line 4"]
pub type Emr4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EMR5` reader - Event Mask on line 5"]
pub type Emr5R = crate::BitReader;
#[doc = "Field `EMR5` writer - Event Mask on line 5"]
pub type Emr5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EMR6` reader - Event Mask on line 6"]
pub type Emr6R = crate::BitReader;
#[doc = "Field `EMR6` writer - Event Mask on line 6"]
pub type Emr6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EMR7` reader - Event Mask on line 7"]
pub type Emr7R = crate::BitReader;
#[doc = "Field `EMR7` writer - Event Mask on line 7"]
pub type Emr7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EMR8` reader - Event Mask on line 8"]
pub type Emr8R = crate::BitReader;
#[doc = "Field `EMR8` writer - Event Mask on line 8"]
pub type Emr8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EMR9` reader - Event Mask on line 9"]
pub type Emr9R = crate::BitReader;
#[doc = "Field `EMR9` writer - Event Mask on line 9"]
pub type Emr9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EMR10` reader - Event Mask on line 10"]
pub type Emr10R = crate::BitReader;
#[doc = "Field `EMR10` writer - Event Mask on line 10"]
pub type Emr10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EMR11` reader - Event Mask on line 11"]
pub type Emr11R = crate::BitReader;
#[doc = "Field `EMR11` writer - Event Mask on line 11"]
pub type Emr11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EMR12` reader - Event Mask on line 12"]
pub type Emr12R = crate::BitReader;
#[doc = "Field `EMR12` writer - Event Mask on line 12"]
pub type Emr12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EMR13` reader - Event Mask on line 13"]
pub type Emr13R = crate::BitReader;
#[doc = "Field `EMR13` writer - Event Mask on line 13"]
pub type Emr13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EMR14` reader - Event Mask on line 14"]
pub type Emr14R = crate::BitReader;
#[doc = "Field `EMR14` writer - Event Mask on line 14"]
pub type Emr14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EMR15` reader - Event Mask on line 15"]
pub type Emr15R = crate::BitReader;
#[doc = "Field `EMR15` writer - Event Mask on line 15"]
pub type Emr15W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EMR16` reader - Event Mask on line 16"]
pub type Emr16R = crate::BitReader;
#[doc = "Field `EMR16` writer - Event Mask on line 16"]
pub type Emr16W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EMR17` reader - Event Mask on line 17"]
pub type Emr17R = crate::BitReader;
#[doc = "Field `EMR17` writer - Event Mask on line 17"]
pub type Emr17W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Event Mask on line 0"]
    #[inline(always)]
    pub fn emr0(&self) -> Emr0R {
        Emr0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Event Mask on line 1"]
    #[inline(always)]
    pub fn emr1(&self) -> Emr1R {
        Emr1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Event Mask on line 2"]
    #[inline(always)]
    pub fn emr2(&self) -> Emr2R {
        Emr2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Event Mask on line 3"]
    #[inline(always)]
    pub fn emr3(&self) -> Emr3R {
        Emr3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Event Mask on line 4"]
    #[inline(always)]
    pub fn emr4(&self) -> Emr4R {
        Emr4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Event Mask on line 5"]
    #[inline(always)]
    pub fn emr5(&self) -> Emr5R {
        Emr5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Event Mask on line 6"]
    #[inline(always)]
    pub fn emr6(&self) -> Emr6R {
        Emr6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Event Mask on line 7"]
    #[inline(always)]
    pub fn emr7(&self) -> Emr7R {
        Emr7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Event Mask on line 8"]
    #[inline(always)]
    pub fn emr8(&self) -> Emr8R {
        Emr8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Event Mask on line 9"]
    #[inline(always)]
    pub fn emr9(&self) -> Emr9R {
        Emr9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Event Mask on line 10"]
    #[inline(always)]
    pub fn emr10(&self) -> Emr10R {
        Emr10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Event Mask on line 11"]
    #[inline(always)]
    pub fn emr11(&self) -> Emr11R {
        Emr11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Event Mask on line 12"]
    #[inline(always)]
    pub fn emr12(&self) -> Emr12R {
        Emr12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Event Mask on line 13"]
    #[inline(always)]
    pub fn emr13(&self) -> Emr13R {
        Emr13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Event Mask on line 14"]
    #[inline(always)]
    pub fn emr14(&self) -> Emr14R {
        Emr14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Event Mask on line 15"]
    #[inline(always)]
    pub fn emr15(&self) -> Emr15R {
        Emr15R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Event Mask on line 16"]
    #[inline(always)]
    pub fn emr16(&self) -> Emr16R {
        Emr16R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Event Mask on line 17"]
    #[inline(always)]
    pub fn emr17(&self) -> Emr17R {
        Emr17R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Event Mask on line 0"]
    #[inline(always)]
    pub fn emr0(&mut self) -> Emr0W<EmrSpec> {
        Emr0W::new(self, 0)
    }
    #[doc = "Bit 1 - Event Mask on line 1"]
    #[inline(always)]
    pub fn emr1(&mut self) -> Emr1W<EmrSpec> {
        Emr1W::new(self, 1)
    }
    #[doc = "Bit 2 - Event Mask on line 2"]
    #[inline(always)]
    pub fn emr2(&mut self) -> Emr2W<EmrSpec> {
        Emr2W::new(self, 2)
    }
    #[doc = "Bit 3 - Event Mask on line 3"]
    #[inline(always)]
    pub fn emr3(&mut self) -> Emr3W<EmrSpec> {
        Emr3W::new(self, 3)
    }
    #[doc = "Bit 4 - Event Mask on line 4"]
    #[inline(always)]
    pub fn emr4(&mut self) -> Emr4W<EmrSpec> {
        Emr4W::new(self, 4)
    }
    #[doc = "Bit 5 - Event Mask on line 5"]
    #[inline(always)]
    pub fn emr5(&mut self) -> Emr5W<EmrSpec> {
        Emr5W::new(self, 5)
    }
    #[doc = "Bit 6 - Event Mask on line 6"]
    #[inline(always)]
    pub fn emr6(&mut self) -> Emr6W<EmrSpec> {
        Emr6W::new(self, 6)
    }
    #[doc = "Bit 7 - Event Mask on line 7"]
    #[inline(always)]
    pub fn emr7(&mut self) -> Emr7W<EmrSpec> {
        Emr7W::new(self, 7)
    }
    #[doc = "Bit 8 - Event Mask on line 8"]
    #[inline(always)]
    pub fn emr8(&mut self) -> Emr8W<EmrSpec> {
        Emr8W::new(self, 8)
    }
    #[doc = "Bit 9 - Event Mask on line 9"]
    #[inline(always)]
    pub fn emr9(&mut self) -> Emr9W<EmrSpec> {
        Emr9W::new(self, 9)
    }
    #[doc = "Bit 10 - Event Mask on line 10"]
    #[inline(always)]
    pub fn emr10(&mut self) -> Emr10W<EmrSpec> {
        Emr10W::new(self, 10)
    }
    #[doc = "Bit 11 - Event Mask on line 11"]
    #[inline(always)]
    pub fn emr11(&mut self) -> Emr11W<EmrSpec> {
        Emr11W::new(self, 11)
    }
    #[doc = "Bit 12 - Event Mask on line 12"]
    #[inline(always)]
    pub fn emr12(&mut self) -> Emr12W<EmrSpec> {
        Emr12W::new(self, 12)
    }
    #[doc = "Bit 13 - Event Mask on line 13"]
    #[inline(always)]
    pub fn emr13(&mut self) -> Emr13W<EmrSpec> {
        Emr13W::new(self, 13)
    }
    #[doc = "Bit 14 - Event Mask on line 14"]
    #[inline(always)]
    pub fn emr14(&mut self) -> Emr14W<EmrSpec> {
        Emr14W::new(self, 14)
    }
    #[doc = "Bit 15 - Event Mask on line 15"]
    #[inline(always)]
    pub fn emr15(&mut self) -> Emr15W<EmrSpec> {
        Emr15W::new(self, 15)
    }
    #[doc = "Bit 16 - Event Mask on line 16"]
    #[inline(always)]
    pub fn emr16(&mut self) -> Emr16W<EmrSpec> {
        Emr16W::new(self, 16)
    }
    #[doc = "Bit 17 - Event Mask on line 17"]
    #[inline(always)]
    pub fn emr17(&mut self) -> Emr17W<EmrSpec> {
        Emr17W::new(self, 17)
    }
}
#[doc = "Event mask register\n\nYou can [`read`](crate::Reg::read) this register and get [`emr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`emr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EmrSpec;
impl crate::RegisterSpec for EmrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`emr::R`](R) reader structure"]
impl crate::Readable for EmrSpec {}
#[doc = "`write(|w| ..)` method takes [`emr::W`](W) writer structure"]
impl crate::Writable for EmrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EMR to value 0"]
impl crate::Resettable for EmrSpec {}
