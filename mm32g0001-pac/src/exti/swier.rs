#[doc = "Register `SWIER` reader"]
pub type R = crate::R<SwierSpec>;
#[doc = "Register `SWIER` writer"]
pub type W = crate::W<SwierSpec>;
#[doc = "Field `SWIER0` reader - Software interrupt on line 0"]
pub type Swier0R = crate::BitReader;
#[doc = "Field `SWIER0` writer - Software interrupt on line 0"]
pub type Swier0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWIER1` reader - Software interrupt on line 1"]
pub type Swier1R = crate::BitReader;
#[doc = "Field `SWIER1` writer - Software interrupt on line 1"]
pub type Swier1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWIER2` reader - Software interrupt on line 2"]
pub type Swier2R = crate::BitReader;
#[doc = "Field `SWIER2` writer - Software interrupt on line 2"]
pub type Swier2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWIER3` reader - Software interrupt on line 3"]
pub type Swier3R = crate::BitReader;
#[doc = "Field `SWIER3` writer - Software interrupt on line 3"]
pub type Swier3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWIER4` reader - Software interrupt on line 4"]
pub type Swier4R = crate::BitReader;
#[doc = "Field `SWIER4` writer - Software interrupt on line 4"]
pub type Swier4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWIER5` reader - Software interrupt on line 5"]
pub type Swier5R = crate::BitReader;
#[doc = "Field `SWIER5` writer - Software interrupt on line 5"]
pub type Swier5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWIER6` reader - Software interrupt on line 6"]
pub type Swier6R = crate::BitReader;
#[doc = "Field `SWIER6` writer - Software interrupt on line 6"]
pub type Swier6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWIER7` reader - Software interrupt on line 7"]
pub type Swier7R = crate::BitReader;
#[doc = "Field `SWIER7` writer - Software interrupt on line 7"]
pub type Swier7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWIER8` reader - Software interrupt on line 8"]
pub type Swier8R = crate::BitReader;
#[doc = "Field `SWIER8` writer - Software interrupt on line 8"]
pub type Swier8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWIER9` reader - Software interrupt on line 9"]
pub type Swier9R = crate::BitReader;
#[doc = "Field `SWIER9` writer - Software interrupt on line 9"]
pub type Swier9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWIER10` reader - Software interrupt on line 10"]
pub type Swier10R = crate::BitReader;
#[doc = "Field `SWIER10` writer - Software interrupt on line 10"]
pub type Swier10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWIER11` reader - Software interrupt on line 11"]
pub type Swier11R = crate::BitReader;
#[doc = "Field `SWIER11` writer - Software interrupt on line 11"]
pub type Swier11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWIER12` reader - Software interrupt on line 12"]
pub type Swier12R = crate::BitReader;
#[doc = "Field `SWIER12` writer - Software interrupt on line 12"]
pub type Swier12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWIER13` reader - Software interrupt on line 13"]
pub type Swier13R = crate::BitReader;
#[doc = "Field `SWIER13` writer - Software interrupt on line 13"]
pub type Swier13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWIER14` reader - Software interrupt on line 14"]
pub type Swier14R = crate::BitReader;
#[doc = "Field `SWIER14` writer - Software interrupt on line 14"]
pub type Swier14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWIER15` reader - Software interrupt on line 15"]
pub type Swier15R = crate::BitReader;
#[doc = "Field `SWIER15` writer - Software interrupt on line 15"]
pub type Swier15W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWIER16` reader - Software interrupt on line 16"]
pub type Swier16R = crate::BitReader;
#[doc = "Field `SWIER16` writer - Software interrupt on line 16"]
pub type Swier16W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWIER17` reader - Software interrupt on line 17"]
pub type Swier17R = crate::BitReader;
#[doc = "Field `SWIER17` writer - Software interrupt on line 17"]
pub type Swier17W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Software interrupt on line 0"]
    #[inline(always)]
    pub fn swier0(&self) -> Swier0R {
        Swier0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Software interrupt on line 1"]
    #[inline(always)]
    pub fn swier1(&self) -> Swier1R {
        Swier1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Software interrupt on line 2"]
    #[inline(always)]
    pub fn swier2(&self) -> Swier2R {
        Swier2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Software interrupt on line 3"]
    #[inline(always)]
    pub fn swier3(&self) -> Swier3R {
        Swier3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Software interrupt on line 4"]
    #[inline(always)]
    pub fn swier4(&self) -> Swier4R {
        Swier4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Software interrupt on line 5"]
    #[inline(always)]
    pub fn swier5(&self) -> Swier5R {
        Swier5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Software interrupt on line 6"]
    #[inline(always)]
    pub fn swier6(&self) -> Swier6R {
        Swier6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Software interrupt on line 7"]
    #[inline(always)]
    pub fn swier7(&self) -> Swier7R {
        Swier7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Software interrupt on line 8"]
    #[inline(always)]
    pub fn swier8(&self) -> Swier8R {
        Swier8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Software interrupt on line 9"]
    #[inline(always)]
    pub fn swier9(&self) -> Swier9R {
        Swier9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Software interrupt on line 10"]
    #[inline(always)]
    pub fn swier10(&self) -> Swier10R {
        Swier10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Software interrupt on line 11"]
    #[inline(always)]
    pub fn swier11(&self) -> Swier11R {
        Swier11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Software interrupt on line 12"]
    #[inline(always)]
    pub fn swier12(&self) -> Swier12R {
        Swier12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Software interrupt on line 13"]
    #[inline(always)]
    pub fn swier13(&self) -> Swier13R {
        Swier13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Software interrupt on line 14"]
    #[inline(always)]
    pub fn swier14(&self) -> Swier14R {
        Swier14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Software interrupt on line 15"]
    #[inline(always)]
    pub fn swier15(&self) -> Swier15R {
        Swier15R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Software interrupt on line 16"]
    #[inline(always)]
    pub fn swier16(&self) -> Swier16R {
        Swier16R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Software interrupt on line 17"]
    #[inline(always)]
    pub fn swier17(&self) -> Swier17R {
        Swier17R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Software interrupt on line 0"]
    #[inline(always)]
    pub fn swier0(&mut self) -> Swier0W<SwierSpec> {
        Swier0W::new(self, 0)
    }
    #[doc = "Bit 1 - Software interrupt on line 1"]
    #[inline(always)]
    pub fn swier1(&mut self) -> Swier1W<SwierSpec> {
        Swier1W::new(self, 1)
    }
    #[doc = "Bit 2 - Software interrupt on line 2"]
    #[inline(always)]
    pub fn swier2(&mut self) -> Swier2W<SwierSpec> {
        Swier2W::new(self, 2)
    }
    #[doc = "Bit 3 - Software interrupt on line 3"]
    #[inline(always)]
    pub fn swier3(&mut self) -> Swier3W<SwierSpec> {
        Swier3W::new(self, 3)
    }
    #[doc = "Bit 4 - Software interrupt on line 4"]
    #[inline(always)]
    pub fn swier4(&mut self) -> Swier4W<SwierSpec> {
        Swier4W::new(self, 4)
    }
    #[doc = "Bit 5 - Software interrupt on line 5"]
    #[inline(always)]
    pub fn swier5(&mut self) -> Swier5W<SwierSpec> {
        Swier5W::new(self, 5)
    }
    #[doc = "Bit 6 - Software interrupt on line 6"]
    #[inline(always)]
    pub fn swier6(&mut self) -> Swier6W<SwierSpec> {
        Swier6W::new(self, 6)
    }
    #[doc = "Bit 7 - Software interrupt on line 7"]
    #[inline(always)]
    pub fn swier7(&mut self) -> Swier7W<SwierSpec> {
        Swier7W::new(self, 7)
    }
    #[doc = "Bit 8 - Software interrupt on line 8"]
    #[inline(always)]
    pub fn swier8(&mut self) -> Swier8W<SwierSpec> {
        Swier8W::new(self, 8)
    }
    #[doc = "Bit 9 - Software interrupt on line 9"]
    #[inline(always)]
    pub fn swier9(&mut self) -> Swier9W<SwierSpec> {
        Swier9W::new(self, 9)
    }
    #[doc = "Bit 10 - Software interrupt on line 10"]
    #[inline(always)]
    pub fn swier10(&mut self) -> Swier10W<SwierSpec> {
        Swier10W::new(self, 10)
    }
    #[doc = "Bit 11 - Software interrupt on line 11"]
    #[inline(always)]
    pub fn swier11(&mut self) -> Swier11W<SwierSpec> {
        Swier11W::new(self, 11)
    }
    #[doc = "Bit 12 - Software interrupt on line 12"]
    #[inline(always)]
    pub fn swier12(&mut self) -> Swier12W<SwierSpec> {
        Swier12W::new(self, 12)
    }
    #[doc = "Bit 13 - Software interrupt on line 13"]
    #[inline(always)]
    pub fn swier13(&mut self) -> Swier13W<SwierSpec> {
        Swier13W::new(self, 13)
    }
    #[doc = "Bit 14 - Software interrupt on line 14"]
    #[inline(always)]
    pub fn swier14(&mut self) -> Swier14W<SwierSpec> {
        Swier14W::new(self, 14)
    }
    #[doc = "Bit 15 - Software interrupt on line 15"]
    #[inline(always)]
    pub fn swier15(&mut self) -> Swier15W<SwierSpec> {
        Swier15W::new(self, 15)
    }
    #[doc = "Bit 16 - Software interrupt on line 16"]
    #[inline(always)]
    pub fn swier16(&mut self) -> Swier16W<SwierSpec> {
        Swier16W::new(self, 16)
    }
    #[doc = "Bit 17 - Software interrupt on line 17"]
    #[inline(always)]
    pub fn swier17(&mut self) -> Swier17W<SwierSpec> {
        Swier17W::new(self, 17)
    }
}
#[doc = "Software interrupt event register\n\nYou can [`read`](crate::Reg::read) this register and get [`swier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SwierSpec;
impl crate::RegisterSpec for SwierSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swier::R`](R) reader structure"]
impl crate::Readable for SwierSpec {}
#[doc = "`write(|w| ..)` method takes [`swier::W`](W) writer structure"]
impl crate::Writable for SwierSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SWIER to value 0"]
impl crate::Resettable for SwierSpec {}
