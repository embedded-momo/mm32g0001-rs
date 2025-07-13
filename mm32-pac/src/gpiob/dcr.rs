#[doc = "Register `DCR` reader"]
pub type R = crate::R<DcrSpec>;
#[doc = "Register `DCR` writer"]
pub type W = crate::W<DcrSpec>;
#[doc = "Field `PX0` reader - PX0"]
pub type Px0R = crate::FieldReader;
#[doc = "Field `PX0` writer - PX0"]
pub type Px0W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PX1` reader - PX1"]
pub type Px1R = crate::FieldReader;
#[doc = "Field `PX1` writer - PX1"]
pub type Px1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - PX0"]
    #[inline(always)]
    pub fn px0(&self) -> Px0R {
        Px0R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - PX1"]
    #[inline(always)]
    pub fn px1(&self) -> Px1R {
        Px1R::new(((self.bits >> 2) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - PX0"]
    #[inline(always)]
    pub fn px0(&mut self) -> Px0W<DcrSpec> {
        Px0W::new(self, 0)
    }
    #[doc = "Bits 2:3 - PX1"]
    #[inline(always)]
    pub fn px1(&mut self) -> Px1W<DcrSpec> {
        Px1W::new(self, 2)
    }
}
#[doc = "Port output open drain control register\n\nYou can [`read`](crate::Reg::read) this register and get [`dcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DcrSpec;
impl crate::RegisterSpec for DcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcr::R`](R) reader structure"]
impl crate::Readable for DcrSpec {}
#[doc = "`write(|w| ..)` method takes [`dcr::W`](W) writer structure"]
impl crate::Writable for DcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DCR to value 0"]
impl crate::Resettable for DcrSpec {}
