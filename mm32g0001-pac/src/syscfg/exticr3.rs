#[doc = "Register `EXTICR3` reader"]
pub type R = crate::R<Exticr3Spec>;
#[doc = "Register `EXTICR3` writer"]
pub type W = crate::W<Exticr3Spec>;
#[doc = "Field `EXTI8` reader - EXTI 8 configuration"]
pub type Exti8R = crate::FieldReader;
#[doc = "Field `EXTI8` writer - EXTI 8 configuration"]
pub type Exti8W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `EXTI9` reader - EXTI 9 configuration"]
pub type Exti9R = crate::FieldReader;
#[doc = "Field `EXTI9` writer - EXTI 9 configuration"]
pub type Exti9W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `EXTI10` reader - EXTI 10 configuration"]
pub type Exti10R = crate::FieldReader;
#[doc = "Field `EXTI10` writer - EXTI 10 configuration"]
pub type Exti10W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `EXTI11` reader - EXTI 11 configuration"]
pub type Exti11R = crate::FieldReader;
#[doc = "Field `EXTI11` writer - EXTI 11 configuration"]
pub type Exti11W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - EXTI 8 configuration"]
    #[inline(always)]
    pub fn exti8(&self) -> Exti8R {
        Exti8R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - EXTI 9 configuration"]
    #[inline(always)]
    pub fn exti9(&self) -> Exti9R {
        Exti9R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - EXTI 10 configuration"]
    #[inline(always)]
    pub fn exti10(&self) -> Exti10R {
        Exti10R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - EXTI 11 configuration"]
    #[inline(always)]
    pub fn exti11(&self) -> Exti11R {
        Exti11R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - EXTI 8 configuration"]
    #[inline(always)]
    pub fn exti8(&mut self) -> Exti8W<Exticr3Spec> {
        Exti8W::new(self, 0)
    }
    #[doc = "Bits 4:7 - EXTI 9 configuration"]
    #[inline(always)]
    pub fn exti9(&mut self) -> Exti9W<Exticr3Spec> {
        Exti9W::new(self, 4)
    }
    #[doc = "Bits 8:11 - EXTI 10 configuration"]
    #[inline(always)]
    pub fn exti10(&mut self) -> Exti10W<Exticr3Spec> {
        Exti10W::new(self, 8)
    }
    #[doc = "Bits 12:15 - EXTI 11 configuration"]
    #[inline(always)]
    pub fn exti11(&mut self) -> Exti11W<Exticr3Spec> {
        Exti11W::new(self, 12)
    }
}
#[doc = "External interrupt configuration register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`exticr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`exticr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Exticr3Spec;
impl crate::RegisterSpec for Exticr3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`exticr3::R`](R) reader structure"]
impl crate::Readable for Exticr3Spec {}
#[doc = "`write(|w| ..)` method takes [`exticr3::W`](W) writer structure"]
impl crate::Writable for Exticr3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EXTICR3 to value 0"]
impl crate::Resettable for Exticr3Spec {}
