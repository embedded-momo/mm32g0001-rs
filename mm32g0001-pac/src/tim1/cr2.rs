#[doc = "Register `CR2` reader"]
pub type R = crate::R<Cr2Spec>;
#[doc = "Register `CR2` writer"]
pub type W = crate::W<Cr2Spec>;
#[doc = "Field `CCPC` reader - compare preloaded control"]
pub type CcpcR = crate::BitReader;
#[doc = "Field `CCPC` writer - compare preloaded control"]
pub type CcpcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCUS` reader - compare control update selection"]
pub type CcusR = crate::BitReader;
#[doc = "Field `CCUS` writer - compare control update selection"]
pub type CcusW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MMS` reader - Master mode selection"]
pub type MmsR = crate::FieldReader;
#[doc = "Field `MMS` writer - Master mode selection"]
pub type MmsW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `OIS1` reader - Output Idle state 1"]
pub type Ois1R = crate::BitReader;
#[doc = "Field `OIS1` writer - Output Idle state 1"]
pub type Ois1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OIS1N` reader - Output Idle state 1"]
pub type Ois1nR = crate::BitReader;
#[doc = "Field `OIS1N` writer - Output Idle state 1"]
pub type Ois1nW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OIS2` reader - OIS2"]
pub type Ois2R = crate::BitReader;
#[doc = "Field `OIS2` writer - OIS2"]
pub type Ois2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OIS2N` reader - OIS2N"]
pub type Ois2nR = crate::BitReader;
#[doc = "Field `OIS2N` writer - OIS2N"]
pub type Ois2nW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OIS3` reader - OIS3"]
pub type Ois3R = crate::BitReader;
#[doc = "Field `OIS3` writer - OIS3"]
pub type Ois3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OIS3N` reader - OIS3N"]
pub type Ois3nR = crate::BitReader;
#[doc = "Field `OIS3N` writer - OIS3N"]
pub type Ois3nW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OIS4` reader - OIS4"]
pub type Ois4R = crate::BitReader;
#[doc = "Field `OIS4` writer - OIS4"]
pub type Ois4W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - compare preloaded control"]
    #[inline(always)]
    pub fn ccpc(&self) -> CcpcR {
        CcpcR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - compare control update selection"]
    #[inline(always)]
    pub fn ccus(&self) -> CcusR {
        CcusR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Master mode selection"]
    #[inline(always)]
    pub fn mms(&self) -> MmsR {
        MmsR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 8 - Output Idle state 1"]
    #[inline(always)]
    pub fn ois1(&self) -> Ois1R {
        Ois1R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Output Idle state 1"]
    #[inline(always)]
    pub fn ois1n(&self) -> Ois1nR {
        Ois1nR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - OIS2"]
    #[inline(always)]
    pub fn ois2(&self) -> Ois2R {
        Ois2R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - OIS2N"]
    #[inline(always)]
    pub fn ois2n(&self) -> Ois2nR {
        Ois2nR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - OIS3"]
    #[inline(always)]
    pub fn ois3(&self) -> Ois3R {
        Ois3R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - OIS3N"]
    #[inline(always)]
    pub fn ois3n(&self) -> Ois3nR {
        Ois3nR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - OIS4"]
    #[inline(always)]
    pub fn ois4(&self) -> Ois4R {
        Ois4R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - compare preloaded control"]
    #[inline(always)]
    pub fn ccpc(&mut self) -> CcpcW<Cr2Spec> {
        CcpcW::new(self, 0)
    }
    #[doc = "Bit 2 - compare control update selection"]
    #[inline(always)]
    pub fn ccus(&mut self) -> CcusW<Cr2Spec> {
        CcusW::new(self, 2)
    }
    #[doc = "Bits 4:6 - Master mode selection"]
    #[inline(always)]
    pub fn mms(&mut self) -> MmsW<Cr2Spec> {
        MmsW::new(self, 4)
    }
    #[doc = "Bit 8 - Output Idle state 1"]
    #[inline(always)]
    pub fn ois1(&mut self) -> Ois1W<Cr2Spec> {
        Ois1W::new(self, 8)
    }
    #[doc = "Bit 9 - Output Idle state 1"]
    #[inline(always)]
    pub fn ois1n(&mut self) -> Ois1nW<Cr2Spec> {
        Ois1nW::new(self, 9)
    }
    #[doc = "Bit 10 - OIS2"]
    #[inline(always)]
    pub fn ois2(&mut self) -> Ois2W<Cr2Spec> {
        Ois2W::new(self, 10)
    }
    #[doc = "Bit 11 - OIS2N"]
    #[inline(always)]
    pub fn ois2n(&mut self) -> Ois2nW<Cr2Spec> {
        Ois2nW::new(self, 11)
    }
    #[doc = "Bit 12 - OIS3"]
    #[inline(always)]
    pub fn ois3(&mut self) -> Ois3W<Cr2Spec> {
        Ois3W::new(self, 12)
    }
    #[doc = "Bit 13 - OIS3N"]
    #[inline(always)]
    pub fn ois3n(&mut self) -> Ois3nW<Cr2Spec> {
        Ois3nW::new(self, 13)
    }
    #[doc = "Bit 14 - OIS4"]
    #[inline(always)]
    pub fn ois4(&mut self) -> Ois4W<Cr2Spec> {
        Ois4W::new(self, 14)
    }
}
#[doc = "control register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`cr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cr2Spec;
impl crate::RegisterSpec for Cr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr2::R`](R) reader structure"]
impl crate::Readable for Cr2Spec {}
#[doc = "`write(|w| ..)` method takes [`cr2::W`](W) writer structure"]
impl crate::Writable for Cr2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR2 to value 0"]
impl crate::Resettable for Cr2Spec {}
