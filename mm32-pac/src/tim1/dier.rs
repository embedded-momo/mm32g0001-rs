#[doc = "Register `DIER` reader"]
pub type R = crate::R<DierSpec>;
#[doc = "Register `DIER` writer"]
pub type W = crate::W<DierSpec>;
#[doc = "Field `UIE` reader - Update interrupt enable"]
pub type UieR = crate::BitReader;
#[doc = "Field `UIE` writer - Update interrupt enable"]
pub type UieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC1IE` reader - Compare 1 interrupt enable"]
pub type Cc1ieR = crate::BitReader;
#[doc = "Field `CC1IE` writer - Compare 1 interrupt enable"]
pub type Cc1ieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC2IE` reader - Compare 2 interrupt enable"]
pub type Cc2ieR = crate::BitReader;
#[doc = "Field `CC2IE` writer - Compare 2 interrupt enable"]
pub type Cc2ieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC3IE` reader - Compare 3 interrupt enable"]
pub type Cc3ieR = crate::BitReader;
#[doc = "Field `CC3IE` writer - Compare 3 interrupt enable"]
pub type Cc3ieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC4IE` reader - Compare 4 interrupt enable"]
pub type Cc4ieR = crate::BitReader;
#[doc = "Field `CC4IE` writer - Compare 4 interrupt enable"]
pub type Cc4ieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMIE` reader - COM interrupt enable"]
pub type ComieR = crate::BitReader;
#[doc = "Field `COMIE` writer - COM interrupt enable"]
pub type ComieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIE` reader - Trigger interrupt enable"]
pub type TieR = crate::BitReader;
#[doc = "Field `TIE` writer - Trigger interrupt enable"]
pub type TieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIE` reader - Break interrupt enable"]
pub type BieR = crate::BitReader;
#[doc = "Field `BIE` writer - Break interrupt enable"]
pub type BieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC5IE` reader - Compare 5 interrupt enable"]
pub type Cc5ieR = crate::BitReader;
#[doc = "Field `CC5IE` writer - Compare 5 interrupt enable"]
pub type Cc5ieW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Update interrupt enable"]
    #[inline(always)]
    pub fn uie(&self) -> UieR {
        UieR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Compare 1 interrupt enable"]
    #[inline(always)]
    pub fn cc1ie(&self) -> Cc1ieR {
        Cc1ieR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Compare 2 interrupt enable"]
    #[inline(always)]
    pub fn cc2ie(&self) -> Cc2ieR {
        Cc2ieR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Compare 3 interrupt enable"]
    #[inline(always)]
    pub fn cc3ie(&self) -> Cc3ieR {
        Cc3ieR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Compare 4 interrupt enable"]
    #[inline(always)]
    pub fn cc4ie(&self) -> Cc4ieR {
        Cc4ieR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - COM interrupt enable"]
    #[inline(always)]
    pub fn comie(&self) -> ComieR {
        ComieR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Trigger interrupt enable"]
    #[inline(always)]
    pub fn tie(&self) -> TieR {
        TieR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Break interrupt enable"]
    #[inline(always)]
    pub fn bie(&self) -> BieR {
        BieR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 16 - Compare 5 interrupt enable"]
    #[inline(always)]
    pub fn cc5ie(&self) -> Cc5ieR {
        Cc5ieR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Update interrupt enable"]
    #[inline(always)]
    pub fn uie(&mut self) -> UieW<DierSpec> {
        UieW::new(self, 0)
    }
    #[doc = "Bit 1 - Compare 1 interrupt enable"]
    #[inline(always)]
    pub fn cc1ie(&mut self) -> Cc1ieW<DierSpec> {
        Cc1ieW::new(self, 1)
    }
    #[doc = "Bit 2 - Compare 2 interrupt enable"]
    #[inline(always)]
    pub fn cc2ie(&mut self) -> Cc2ieW<DierSpec> {
        Cc2ieW::new(self, 2)
    }
    #[doc = "Bit 3 - Compare 3 interrupt enable"]
    #[inline(always)]
    pub fn cc3ie(&mut self) -> Cc3ieW<DierSpec> {
        Cc3ieW::new(self, 3)
    }
    #[doc = "Bit 4 - Compare 4 interrupt enable"]
    #[inline(always)]
    pub fn cc4ie(&mut self) -> Cc4ieW<DierSpec> {
        Cc4ieW::new(self, 4)
    }
    #[doc = "Bit 5 - COM interrupt enable"]
    #[inline(always)]
    pub fn comie(&mut self) -> ComieW<DierSpec> {
        ComieW::new(self, 5)
    }
    #[doc = "Bit 6 - Trigger interrupt enable"]
    #[inline(always)]
    pub fn tie(&mut self) -> TieW<DierSpec> {
        TieW::new(self, 6)
    }
    #[doc = "Bit 7 - Break interrupt enable"]
    #[inline(always)]
    pub fn bie(&mut self) -> BieW<DierSpec> {
        BieW::new(self, 7)
    }
    #[doc = "Bit 16 - Compare 5 interrupt enable"]
    #[inline(always)]
    pub fn cc5ie(&mut self) -> Cc5ieW<DierSpec> {
        Cc5ieW::new(self, 16)
    }
}
#[doc = "DMA/Interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`dier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DierSpec;
impl crate::RegisterSpec for DierSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dier::R`](R) reader structure"]
impl crate::Readable for DierSpec {}
#[doc = "`write(|w| ..)` method takes [`dier::W`](W) writer structure"]
impl crate::Writable for DierSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DIER to value 0"]
impl crate::Resettable for DierSpec {}
