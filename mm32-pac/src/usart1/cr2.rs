#[doc = "Register `CR2` reader"]
pub type R = crate::R<Cr2Spec>;
#[doc = "Register `CR2` writer"]
pub type W = crate::W<Cr2Spec>;
#[doc = "Field `CPHA` reader - "]
pub type CphaR = crate::BitReader;
#[doc = "Field `CPHA` writer - "]
pub type CphaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPOL` reader - "]
pub type CpolR = crate::BitReader;
#[doc = "Field `CPOL` writer - "]
pub type CpolW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STOP` reader - "]
pub type StopR = crate::FieldReader;
#[doc = "Field `STOP` writer - "]
pub type StopW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SWAP` reader - "]
pub type SwapR = crate::BitReader;
#[doc = "Field `SWAP` writer - "]
pub type SwapW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn cpha(&self) -> CphaR {
        CphaR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn cpol(&self) -> CpolR {
        CpolR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn stop(&self) -> StopR {
        StopR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn swap(&self) -> SwapR {
        SwapR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn cpha(&mut self) -> CphaW<Cr2Spec> {
        CphaW::new(self, 9)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn cpol(&mut self) -> CpolW<Cr2Spec> {
        CpolW::new(self, 10)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn stop(&mut self) -> StopW<Cr2Spec> {
        StopW::new(self, 12)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn swap(&mut self) -> SwapW<Cr2Spec> {
        SwapW::new(self, 15)
    }
}
#[doc = "Control register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`cr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
