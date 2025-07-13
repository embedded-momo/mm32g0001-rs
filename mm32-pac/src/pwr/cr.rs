#[doc = "Register `CR` reader"]
pub type R = crate::R<CrSpec>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CrSpec>;
#[doc = "Field `LPDS` reader - Power down deepsleep"]
pub type LpdsR = crate::BitReader;
#[doc = "Field `LPDS` writer - Power down deepsleep"]
pub type LpdsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PVDE` reader - Power voltage detector enable"]
pub type PvdeR = crate::BitReader;
#[doc = "Field `PVDE` writer - Power voltage detector enable"]
pub type PvdeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLS` reader - PVD level selection"]
pub type PlsR = crate::FieldReader;
#[doc = "Field `PLS` writer - PVD level selection"]
pub type PlsW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 1 - Power down deepsleep"]
    #[inline(always)]
    pub fn lpds(&self) -> LpdsR {
        LpdsR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - Power voltage detector enable"]
    #[inline(always)]
    pub fn pvde(&self) -> PvdeR {
        PvdeR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 9:12 - PVD level selection"]
    #[inline(always)]
    pub fn pls(&self) -> PlsR {
        PlsR::new(((self.bits >> 9) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 1 - Power down deepsleep"]
    #[inline(always)]
    pub fn lpds(&mut self) -> LpdsW<CrSpec> {
        LpdsW::new(self, 1)
    }
    #[doc = "Bit 4 - Power voltage detector enable"]
    #[inline(always)]
    pub fn pvde(&mut self) -> PvdeW<CrSpec> {
        PvdeW::new(self, 4)
    }
    #[doc = "Bits 9:12 - PVD level selection"]
    #[inline(always)]
    pub fn pls(&mut self) -> PlsW<CrSpec> {
        PlsW::new(self, 9)
    }
}
#[doc = "CR\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrSpec;
impl crate::RegisterSpec for CrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr::R`](R) reader structure"]
impl crate::Readable for CrSpec {}
#[doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"]
impl crate::Writable for CrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CrSpec {}
