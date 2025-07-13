#[doc = "Register `CCMR1_Output` reader"]
pub type R = crate::R<Ccmr1OutputSpec>;
#[doc = "Register `CCMR1_Output` writer"]
pub type W = crate::W<Ccmr1OutputSpec>;
#[doc = "Field `OC1FE` reader - Output compare 1 fast enable"]
pub type Oc1feR = crate::BitReader;
#[doc = "Field `OC1FE` writer - Output compare 1 fast enable"]
pub type Oc1feW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OC1PE` reader - Output compare 1 preload enable"]
pub type Oc1peR = crate::BitReader;
#[doc = "Field `OC1PE` writer - Output compare 1 preload enable"]
pub type Oc1peW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OC1M` reader - Output compare 1 mode"]
pub type Oc1mR = crate::FieldReader;
#[doc = "Field `OC1M` writer - Output compare 1 mode"]
pub type Oc1mW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `OC2FE` reader - Output compare 4 fast enable"]
pub type Oc2feR = crate::BitReader;
#[doc = "Field `OC2FE` writer - Output compare 4 fast enable"]
pub type Oc2feW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OC2PE` reader - Output compare 2 preload enable"]
pub type Oc2peR = crate::BitReader;
#[doc = "Field `OC2PE` writer - Output compare 2 preload enable"]
pub type Oc2peW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OC2M` reader - Output compare 2 mode"]
pub type Oc2mR = crate::FieldReader;
#[doc = "Field `OC2M` writer - Output compare 2 mode"]
pub type Oc2mW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 2 - Output compare 1 fast enable"]
    #[inline(always)]
    pub fn oc1fe(&self) -> Oc1feR {
        Oc1feR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Output compare 1 preload enable"]
    #[inline(always)]
    pub fn oc1pe(&self) -> Oc1peR {
        Oc1peR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Output compare 1 mode"]
    #[inline(always)]
    pub fn oc1m(&self) -> Oc1mR {
        Oc1mR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 10 - Output compare 4 fast enable"]
    #[inline(always)]
    pub fn oc2fe(&self) -> Oc2feR {
        Oc2feR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Output compare 2 preload enable"]
    #[inline(always)]
    pub fn oc2pe(&self) -> Oc2peR {
        Oc2peR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:14 - Output compare 2 mode"]
    #[inline(always)]
    pub fn oc2m(&self) -> Oc2mR {
        Oc2mR::new(((self.bits >> 12) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 2 - Output compare 1 fast enable"]
    #[inline(always)]
    pub fn oc1fe(&mut self) -> Oc1feW<Ccmr1OutputSpec> {
        Oc1feW::new(self, 2)
    }
    #[doc = "Bit 3 - Output compare 1 preload enable"]
    #[inline(always)]
    pub fn oc1pe(&mut self) -> Oc1peW<Ccmr1OutputSpec> {
        Oc1peW::new(self, 3)
    }
    #[doc = "Bits 4:6 - Output compare 1 mode"]
    #[inline(always)]
    pub fn oc1m(&mut self) -> Oc1mW<Ccmr1OutputSpec> {
        Oc1mW::new(self, 4)
    }
    #[doc = "Bit 10 - Output compare 4 fast enable"]
    #[inline(always)]
    pub fn oc2fe(&mut self) -> Oc2feW<Ccmr1OutputSpec> {
        Oc2feW::new(self, 10)
    }
    #[doc = "Bit 11 - Output compare 2 preload enable"]
    #[inline(always)]
    pub fn oc2pe(&mut self) -> Oc2peW<Ccmr1OutputSpec> {
        Oc2peW::new(self, 11)
    }
    #[doc = "Bits 12:14 - Output compare 2 mode"]
    #[inline(always)]
    pub fn oc2m(&mut self) -> Oc2mW<Ccmr1OutputSpec> {
        Oc2mW::new(self, 12)
    }
}
#[doc = "compare mode register 1 (output mode)\n\nYou can [`read`](crate::Reg::read) this register and get [`ccmr1_output::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccmr1_output::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ccmr1OutputSpec;
impl crate::RegisterSpec for Ccmr1OutputSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccmr1_output::R`](R) reader structure"]
impl crate::Readable for Ccmr1OutputSpec {}
#[doc = "`write(|w| ..)` method takes [`ccmr1_output::W`](W) writer structure"]
impl crate::Writable for Ccmr1OutputSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CCMR1_Output to value 0"]
impl crate::Resettable for Ccmr1OutputSpec {}
