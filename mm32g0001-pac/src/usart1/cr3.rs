#[doc = "Register `CR3` reader"]
pub type R = crate::R<Cr3Spec>;
#[doc = "Register `CR3` writer"]
pub type W = crate::W<Cr3Spec>;
#[doc = "Field `ERRIEN` reader - "]
pub type ErrienR = crate::BitReader;
#[doc = "Field `ERRIEN` writer - "]
pub type ErrienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HDSEL` reader - "]
pub type HdselR = crate::BitReader;
#[doc = "Field `HDSEL` writer - "]
pub type HdselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ONEBIT` reader - "]
pub type OnebitR = crate::BitReader;
#[doc = "Field `ONEBIT` writer - "]
pub type OnebitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CKINE` reader - "]
pub type CkineR = crate::BitReader;
#[doc = "Field `CKINE` writer - "]
pub type CkineW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXTOG` reader - "]
pub type RxtogR = crate::BitReader;
#[doc = "Field `RXTOG` writer - "]
pub type RxtogW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXTOG` reader - "]
pub type TxtogR = crate::BitReader;
#[doc = "Field `TXTOG` writer - "]
pub type TxtogW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn errien(&self) -> ErrienR {
        ErrienR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn hdsel(&self) -> HdselR {
        HdselR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn onebit(&self) -> OnebitR {
        OnebitR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn ckine(&self) -> CkineR {
        CkineR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn rxtog(&self) -> RxtogR {
        RxtogR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn txtog(&self) -> TxtogR {
        TxtogR::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn errien(&mut self) -> ErrienW<Cr3Spec> {
        ErrienW::new(self, 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn hdsel(&mut self) -> HdselW<Cr3Spec> {
        HdselW::new(self, 3)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn onebit(&mut self) -> OnebitW<Cr3Spec> {
        OnebitW::new(self, 11)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn ckine(&mut self) -> CkineW<Cr3Spec> {
        CkineW::new(self, 16)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn rxtog(&mut self) -> RxtogW<Cr3Spec> {
        RxtogW::new(self, 28)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn txtog(&mut self) -> TxtogW<Cr3Spec> {
        TxtogW::new(self, 29)
    }
}
#[doc = "Control register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`cr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cr3Spec;
impl crate::RegisterSpec for Cr3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr3::R`](R) reader structure"]
impl crate::Readable for Cr3Spec {}
#[doc = "`write(|w| ..)` method takes [`cr3::W`](W) writer structure"]
impl crate::Writable for Cr3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR3 to value 0x6000"]
impl crate::Resettable for Cr3Spec {
    const RESET_VALUE: u32 = 0x6000;
}
