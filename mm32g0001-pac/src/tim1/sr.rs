#[doc = "Register `SR` reader"]
pub type R = crate::R<SrSpec>;
#[doc = "Register `SR` writer"]
pub type W = crate::W<SrSpec>;
#[doc = "Field `UIF` reader - Update interrupt flag"]
pub type UifR = crate::BitReader;
#[doc = "Field `UIF` writer - Update interrupt flag"]
pub type UifW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC1IF` reader - Compare 1 interrupt flag"]
pub type Cc1ifR = crate::BitReader;
#[doc = "Field `CC1IF` writer - Compare 1 interrupt flag"]
pub type Cc1ifW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC2IF` reader - Compare 2 interrupt flag"]
pub type Cc2ifR = crate::BitReader;
#[doc = "Field `CC2IF` writer - Compare 2 interrupt flag"]
pub type Cc2ifW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC3IF` reader - Compare 3 interrupt flag"]
pub type Cc3ifR = crate::BitReader;
#[doc = "Field `CC3IF` writer - Compare 3 interrupt flag"]
pub type Cc3ifW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC4IF` reader - Compare 4 interrupt flag"]
pub type Cc4ifR = crate::BitReader;
#[doc = "Field `CC4IF` writer - Compare 4 interrupt flag"]
pub type Cc4ifW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMIF` reader - COM interrupt flag"]
pub type ComifR = crate::BitReader;
#[doc = "Field `COMIF` writer - COM interrupt flag"]
pub type ComifW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIF` reader - Trigger interrupt flag"]
pub type TifR = crate::BitReader;
#[doc = "Field `TIF` writer - Trigger interrupt flag"]
pub type TifW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIF` reader - Break interrupt flag"]
pub type BifR = crate::BitReader;
#[doc = "Field `BIF` writer - Break interrupt flag"]
pub type BifW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC5IF` reader - Compare 5 interrupt flag"]
pub type Cc5ifR = crate::BitReader;
#[doc = "Field `CC5IF` writer - Compare 5 interrupt flag"]
pub type Cc5ifW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Update interrupt flag"]
    #[inline(always)]
    pub fn uif(&self) -> UifR {
        UifR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Compare 1 interrupt flag"]
    #[inline(always)]
    pub fn cc1if(&self) -> Cc1ifR {
        Cc1ifR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Compare 2 interrupt flag"]
    #[inline(always)]
    pub fn cc2if(&self) -> Cc2ifR {
        Cc2ifR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Compare 3 interrupt flag"]
    #[inline(always)]
    pub fn cc3if(&self) -> Cc3ifR {
        Cc3ifR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Compare 4 interrupt flag"]
    #[inline(always)]
    pub fn cc4if(&self) -> Cc4ifR {
        Cc4ifR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - COM interrupt flag"]
    #[inline(always)]
    pub fn comif(&self) -> ComifR {
        ComifR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Trigger interrupt flag"]
    #[inline(always)]
    pub fn tif(&self) -> TifR {
        TifR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Break interrupt flag"]
    #[inline(always)]
    pub fn bif(&self) -> BifR {
        BifR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 16 - Compare 5 interrupt flag"]
    #[inline(always)]
    pub fn cc5if(&self) -> Cc5ifR {
        Cc5ifR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Update interrupt flag"]
    #[inline(always)]
    pub fn uif(&mut self) -> UifW<SrSpec> {
        UifW::new(self, 0)
    }
    #[doc = "Bit 1 - Compare 1 interrupt flag"]
    #[inline(always)]
    pub fn cc1if(&mut self) -> Cc1ifW<SrSpec> {
        Cc1ifW::new(self, 1)
    }
    #[doc = "Bit 2 - Compare 2 interrupt flag"]
    #[inline(always)]
    pub fn cc2if(&mut self) -> Cc2ifW<SrSpec> {
        Cc2ifW::new(self, 2)
    }
    #[doc = "Bit 3 - Compare 3 interrupt flag"]
    #[inline(always)]
    pub fn cc3if(&mut self) -> Cc3ifW<SrSpec> {
        Cc3ifW::new(self, 3)
    }
    #[doc = "Bit 4 - Compare 4 interrupt flag"]
    #[inline(always)]
    pub fn cc4if(&mut self) -> Cc4ifW<SrSpec> {
        Cc4ifW::new(self, 4)
    }
    #[doc = "Bit 5 - COM interrupt flag"]
    #[inline(always)]
    pub fn comif(&mut self) -> ComifW<SrSpec> {
        ComifW::new(self, 5)
    }
    #[doc = "Bit 6 - Trigger interrupt flag"]
    #[inline(always)]
    pub fn tif(&mut self) -> TifW<SrSpec> {
        TifW::new(self, 6)
    }
    #[doc = "Bit 7 - Break interrupt flag"]
    #[inline(always)]
    pub fn bif(&mut self) -> BifW<SrSpec> {
        BifW::new(self, 7)
    }
    #[doc = "Bit 16 - Compare 5 interrupt flag"]
    #[inline(always)]
    pub fn cc5if(&mut self) -> Cc5ifW<SrSpec> {
        Cc5ifW::new(self, 16)
    }
}
#[doc = "status register\n\nYou can [`read`](crate::Reg::read) this register and get [`sr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrSpec;
impl crate::RegisterSpec for SrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr::R`](R) reader structure"]
impl crate::Readable for SrSpec {}
#[doc = "`write(|w| ..)` method takes [`sr::W`](W) writer structure"]
impl crate::Writable for SrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SR to value 0"]
impl crate::Resettable for SrSpec {}
