#[doc = "Register `PR` reader"]
pub type R = crate::R<PrSpec>;
#[doc = "Register `PR` writer"]
pub type W = crate::W<PrSpec>;
#[doc = "Field `PR0` reader - Pending bit"]
pub type Pr0R = crate::BitReader;
#[doc = "Field `PR0` writer - Pending bit"]
pub type Pr0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PR1` reader - Pending bit"]
pub type Pr1R = crate::BitReader;
#[doc = "Field `PR1` writer - Pending bit"]
pub type Pr1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PR2` reader - Pending bit"]
pub type Pr2R = crate::BitReader;
#[doc = "Field `PR2` writer - Pending bit"]
pub type Pr2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PR3` reader - Pending bit"]
pub type Pr3R = crate::BitReader;
#[doc = "Field `PR3` writer - Pending bit"]
pub type Pr3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PR4` reader - Pending bit"]
pub type Pr4R = crate::BitReader;
#[doc = "Field `PR4` writer - Pending bit"]
pub type Pr4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PR5` reader - Pending bit"]
pub type Pr5R = crate::BitReader;
#[doc = "Field `PR5` writer - Pending bit"]
pub type Pr5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PR6` reader - Pending bit"]
pub type Pr6R = crate::BitReader;
#[doc = "Field `PR6` writer - Pending bit"]
pub type Pr6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PR7` reader - Pending bit"]
pub type Pr7R = crate::BitReader;
#[doc = "Field `PR7` writer - Pending bit"]
pub type Pr7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PR8` reader - Pending bit"]
pub type Pr8R = crate::BitReader;
#[doc = "Field `PR8` writer - Pending bit"]
pub type Pr8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PR9` reader - Pending bit"]
pub type Pr9R = crate::BitReader;
#[doc = "Field `PR9` writer - Pending bit"]
pub type Pr9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PR10` reader - Pending bit"]
pub type Pr10R = crate::BitReader;
#[doc = "Field `PR10` writer - Pending bit"]
pub type Pr10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PR11` reader - Pending bit"]
pub type Pr11R = crate::BitReader;
#[doc = "Field `PR11` writer - Pending bit"]
pub type Pr11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PR12` reader - Pending bit"]
pub type Pr12R = crate::BitReader;
#[doc = "Field `PR12` writer - Pending bit"]
pub type Pr12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PR13` reader - Pending bit"]
pub type Pr13R = crate::BitReader;
#[doc = "Field `PR13` writer - Pending bit"]
pub type Pr13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PR14` reader - Pending bit"]
pub type Pr14R = crate::BitReader;
#[doc = "Field `PR14` writer - Pending bit"]
pub type Pr14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PR15` reader - Pending bit"]
pub type Pr15R = crate::BitReader;
#[doc = "Field `PR15` writer - Pending bit"]
pub type Pr15W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PR16` reader - Pending bit"]
pub type Pr16R = crate::BitReader;
#[doc = "Field `PR16` writer - Pending bit"]
pub type Pr16W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PR17` reader - Pending bit"]
pub type Pr17R = crate::BitReader;
#[doc = "Field `PR17` writer - Pending bit"]
pub type Pr17W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Pending bit"]
    #[inline(always)]
    pub fn pr0(&self) -> Pr0R {
        Pr0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Pending bit"]
    #[inline(always)]
    pub fn pr1(&self) -> Pr1R {
        Pr1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Pending bit"]
    #[inline(always)]
    pub fn pr2(&self) -> Pr2R {
        Pr2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Pending bit"]
    #[inline(always)]
    pub fn pr3(&self) -> Pr3R {
        Pr3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Pending bit"]
    #[inline(always)]
    pub fn pr4(&self) -> Pr4R {
        Pr4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Pending bit"]
    #[inline(always)]
    pub fn pr5(&self) -> Pr5R {
        Pr5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Pending bit"]
    #[inline(always)]
    pub fn pr6(&self) -> Pr6R {
        Pr6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Pending bit"]
    #[inline(always)]
    pub fn pr7(&self) -> Pr7R {
        Pr7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Pending bit"]
    #[inline(always)]
    pub fn pr8(&self) -> Pr8R {
        Pr8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Pending bit"]
    #[inline(always)]
    pub fn pr9(&self) -> Pr9R {
        Pr9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Pending bit"]
    #[inline(always)]
    pub fn pr10(&self) -> Pr10R {
        Pr10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Pending bit"]
    #[inline(always)]
    pub fn pr11(&self) -> Pr11R {
        Pr11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Pending bit"]
    #[inline(always)]
    pub fn pr12(&self) -> Pr12R {
        Pr12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Pending bit"]
    #[inline(always)]
    pub fn pr13(&self) -> Pr13R {
        Pr13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Pending bit"]
    #[inline(always)]
    pub fn pr14(&self) -> Pr14R {
        Pr14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Pending bit"]
    #[inline(always)]
    pub fn pr15(&self) -> Pr15R {
        Pr15R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Pending bit"]
    #[inline(always)]
    pub fn pr16(&self) -> Pr16R {
        Pr16R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Pending bit"]
    #[inline(always)]
    pub fn pr17(&self) -> Pr17R {
        Pr17R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Pending bit"]
    #[inline(always)]
    pub fn pr0(&mut self) -> Pr0W<PrSpec> {
        Pr0W::new(self, 0)
    }
    #[doc = "Bit 1 - Pending bit"]
    #[inline(always)]
    pub fn pr1(&mut self) -> Pr1W<PrSpec> {
        Pr1W::new(self, 1)
    }
    #[doc = "Bit 2 - Pending bit"]
    #[inline(always)]
    pub fn pr2(&mut self) -> Pr2W<PrSpec> {
        Pr2W::new(self, 2)
    }
    #[doc = "Bit 3 - Pending bit"]
    #[inline(always)]
    pub fn pr3(&mut self) -> Pr3W<PrSpec> {
        Pr3W::new(self, 3)
    }
    #[doc = "Bit 4 - Pending bit"]
    #[inline(always)]
    pub fn pr4(&mut self) -> Pr4W<PrSpec> {
        Pr4W::new(self, 4)
    }
    #[doc = "Bit 5 - Pending bit"]
    #[inline(always)]
    pub fn pr5(&mut self) -> Pr5W<PrSpec> {
        Pr5W::new(self, 5)
    }
    #[doc = "Bit 6 - Pending bit"]
    #[inline(always)]
    pub fn pr6(&mut self) -> Pr6W<PrSpec> {
        Pr6W::new(self, 6)
    }
    #[doc = "Bit 7 - Pending bit"]
    #[inline(always)]
    pub fn pr7(&mut self) -> Pr7W<PrSpec> {
        Pr7W::new(self, 7)
    }
    #[doc = "Bit 8 - Pending bit"]
    #[inline(always)]
    pub fn pr8(&mut self) -> Pr8W<PrSpec> {
        Pr8W::new(self, 8)
    }
    #[doc = "Bit 9 - Pending bit"]
    #[inline(always)]
    pub fn pr9(&mut self) -> Pr9W<PrSpec> {
        Pr9W::new(self, 9)
    }
    #[doc = "Bit 10 - Pending bit"]
    #[inline(always)]
    pub fn pr10(&mut self) -> Pr10W<PrSpec> {
        Pr10W::new(self, 10)
    }
    #[doc = "Bit 11 - Pending bit"]
    #[inline(always)]
    pub fn pr11(&mut self) -> Pr11W<PrSpec> {
        Pr11W::new(self, 11)
    }
    #[doc = "Bit 12 - Pending bit"]
    #[inline(always)]
    pub fn pr12(&mut self) -> Pr12W<PrSpec> {
        Pr12W::new(self, 12)
    }
    #[doc = "Bit 13 - Pending bit"]
    #[inline(always)]
    pub fn pr13(&mut self) -> Pr13W<PrSpec> {
        Pr13W::new(self, 13)
    }
    #[doc = "Bit 14 - Pending bit"]
    #[inline(always)]
    pub fn pr14(&mut self) -> Pr14W<PrSpec> {
        Pr14W::new(self, 14)
    }
    #[doc = "Bit 15 - Pending bit"]
    #[inline(always)]
    pub fn pr15(&mut self) -> Pr15W<PrSpec> {
        Pr15W::new(self, 15)
    }
    #[doc = "Bit 16 - Pending bit"]
    #[inline(always)]
    pub fn pr16(&mut self) -> Pr16W<PrSpec> {
        Pr16W::new(self, 16)
    }
    #[doc = "Bit 17 - Pending bit"]
    #[inline(always)]
    pub fn pr17(&mut self) -> Pr17W<PrSpec> {
        Pr17W::new(self, 17)
    }
}
#[doc = "Pending register\n\nYou can [`read`](crate::Reg::read) this register and get [`pr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PrSpec;
impl crate::RegisterSpec for PrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr::R`](R) reader structure"]
impl crate::Readable for PrSpec {}
#[doc = "`write(|w| ..)` method takes [`pr::W`](W) writer structure"]
impl crate::Writable for PrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PR to value 0"]
impl crate::Resettable for PrSpec {}
