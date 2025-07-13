#[doc = "Register `CR1` reader"]
pub type R = crate::R<Cr1Spec>;
#[doc = "Register `CR1` writer"]
pub type W = crate::W<Cr1Spec>;
#[doc = "Field `SBK` reader - "]
pub type SbkR = crate::BitReader;
#[doc = "Field `SBK` writer - "]
pub type SbkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RE` reader - "]
pub type ReR = crate::BitReader;
#[doc = "Field `RE` writer - "]
pub type ReW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TE` reader - "]
pub type TeR = crate::BitReader;
#[doc = "Field `TE` writer - "]
pub type TeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IDLEIEN` reader - "]
pub type IdleienR = crate::BitReader;
#[doc = "Field `IDLEIEN` writer - "]
pub type IdleienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXNEIEN` reader - "]
pub type RxneienR = crate::BitReader;
#[doc = "Field `RXNEIEN` writer - "]
pub type RxneienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCIEN` reader - "]
pub type TcienR = crate::BitReader;
#[doc = "Field `TCIEN` writer - "]
pub type TcienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXEIEN` reader - "]
pub type TxeienR = crate::BitReader;
#[doc = "Field `TXEIEN` writer - "]
pub type TxeienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PEIEN` reader - "]
pub type PeienR = crate::BitReader;
#[doc = "Field `PEIEN` writer - "]
pub type PeienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PS` reader - "]
pub type PsR = crate::BitReader;
#[doc = "Field `PS` writer - "]
pub type PsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCE` reader - "]
pub type PceR = crate::BitReader;
#[doc = "Field `PCE` writer - "]
pub type PceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DL` reader - "]
pub type DlR = crate::BitReader;
#[doc = "Field `DL` writer - "]
pub type DlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UE` reader - "]
pub type UeR = crate::BitReader;
#[doc = "Field `UE` writer - "]
pub type UeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVER8` reader - "]
pub type Over8R = crate::BitReader;
#[doc = "Field `OVER8` writer - "]
pub type Over8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MLS` reader - "]
pub type MlsR = crate::BitReader;
#[doc = "Field `MLS` writer - "]
pub type MlsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SAS` reader - "]
pub type SasR = crate::BitReader;
#[doc = "Field `SAS` writer - "]
pub type SasW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn sbk(&self) -> SbkR {
        SbkR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn re(&self) -> ReR {
        ReR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn te(&self) -> TeR {
        TeR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn idleien(&self) -> IdleienR {
        IdleienR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn rxneien(&self) -> RxneienR {
        RxneienR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn tcien(&self) -> TcienR {
        TcienR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn txeien(&self) -> TxeienR {
        TxeienR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn peien(&self) -> PeienR {
        PeienR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn ps(&self) -> PsR {
        PsR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn pce(&self) -> PceR {
        PceR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn dl(&self) -> DlR {
        DlR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn ue(&self) -> UeR {
        UeR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn over8(&self) -> Over8R {
        Over8R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn mls(&self) -> MlsR {
        MlsR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn sas(&self) -> SasR {
        SasR::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn sbk(&mut self) -> SbkW<Cr1Spec> {
        SbkW::new(self, 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn re(&mut self) -> ReW<Cr1Spec> {
        ReW::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn te(&mut self) -> TeW<Cr1Spec> {
        TeW::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn idleien(&mut self) -> IdleienW<Cr1Spec> {
        IdleienW::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn rxneien(&mut self) -> RxneienW<Cr1Spec> {
        RxneienW::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn tcien(&mut self) -> TcienW<Cr1Spec> {
        TcienW::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn txeien(&mut self) -> TxeienW<Cr1Spec> {
        TxeienW::new(self, 7)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn peien(&mut self) -> PeienW<Cr1Spec> {
        PeienW::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn ps(&mut self) -> PsW<Cr1Spec> {
        PsW::new(self, 9)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn pce(&mut self) -> PceW<Cr1Spec> {
        PceW::new(self, 10)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn dl(&mut self) -> DlW<Cr1Spec> {
        DlW::new(self, 12)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn ue(&mut self) -> UeW<Cr1Spec> {
        UeW::new(self, 13)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn over8(&mut self) -> Over8W<Cr1Spec> {
        Over8W::new(self, 15)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn mls(&mut self) -> MlsW<Cr1Spec> {
        MlsW::new(self, 16)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn sas(&mut self) -> SasW<Cr1Spec> {
        SasW::new(self, 17)
    }
}
#[doc = "Control register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`cr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cr1Spec;
impl crate::RegisterSpec for Cr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr1::R`](R) reader structure"]
impl crate::Readable for Cr1Spec {}
#[doc = "`write(|w| ..)` method takes [`cr1::W`](W) writer structure"]
impl crate::Writable for Cr1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR1 to value 0"]
impl crate::Resettable for Cr1Spec {}
