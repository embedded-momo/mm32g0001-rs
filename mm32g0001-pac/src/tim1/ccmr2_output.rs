#[doc = "Register `CCMR2_Output` reader"]
pub type R = crate::R<Ccmr2OutputSpec>;
#[doc = "Register `CCMR2_Output` writer"]
pub type W = crate::W<Ccmr2OutputSpec>;
#[doc = "Field `OC3FE` reader - Output compare 3 fast enable"]
pub type Oc3feR = crate::BitReader;
#[doc = "Field `OC3FE` writer - Output compare 3 fast enable"]
pub type Oc3feW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OC3PE` reader - Output compare 3 preload enable"]
pub type Oc3peR = crate::BitReader;
#[doc = "Field `OC3PE` writer - Output compare 3 preload enable"]
pub type Oc3peW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OC3M` reader - Output compare 3 mode"]
pub type Oc3mR = crate::FieldReader;
#[doc = "Field `OC3M` writer - Output compare 3 mode"]
pub type Oc3mW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `OC3CE` reader - Output compare 3 clear enable"]
pub type Oc3ceR = crate::BitReader;
#[doc = "Field `OC3CE` writer - Output compare 3 clear enable"]
pub type Oc3ceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OC4FE` reader - Output compare 4 fast enable"]
pub type Oc4feR = crate::BitReader;
#[doc = "Field `OC4FE` writer - Output compare 4 fast enable"]
pub type Oc4feW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OC4PE` reader - Output compare 4 preload enable"]
pub type Oc4peR = crate::BitReader;
#[doc = "Field `OC4PE` writer - Output compare 4 preload enable"]
pub type Oc4peW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OC4M` reader - Output compare 4 mode"]
pub type Oc4mR = crate::FieldReader;
#[doc = "Field `OC4M` writer - Output compare 4 mode"]
pub type Oc4mW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `OC4CE` reader - Output compare 4 clear enable"]
pub type Oc4ceR = crate::BitReader;
#[doc = "Field `OC4CE` writer - Output compare 4 clear enable"]
pub type Oc4ceW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 2 - Output compare 3 fast enable"]
    #[inline(always)]
    pub fn oc3fe(&self) -> Oc3feR {
        Oc3feR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Output compare 3 preload enable"]
    #[inline(always)]
    pub fn oc3pe(&self) -> Oc3peR {
        Oc3peR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Output compare 3 mode"]
    #[inline(always)]
    pub fn oc3m(&self) -> Oc3mR {
        Oc3mR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - Output compare 3 clear enable"]
    #[inline(always)]
    pub fn oc3ce(&self) -> Oc3ceR {
        Oc3ceR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 10 - Output compare 4 fast enable"]
    #[inline(always)]
    pub fn oc4fe(&self) -> Oc4feR {
        Oc4feR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Output compare 4 preload enable"]
    #[inline(always)]
    pub fn oc4pe(&self) -> Oc4peR {
        Oc4peR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:14 - Output compare 4 mode"]
    #[inline(always)]
    pub fn oc4m(&self) -> Oc4mR {
        Oc4mR::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - Output compare 4 clear enable"]
    #[inline(always)]
    pub fn oc4ce(&self) -> Oc4ceR {
        Oc4ceR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Output compare 3 fast enable"]
    #[inline(always)]
    pub fn oc3fe(&mut self) -> Oc3feW<Ccmr2OutputSpec> {
        Oc3feW::new(self, 2)
    }
    #[doc = "Bit 3 - Output compare 3 preload enable"]
    #[inline(always)]
    pub fn oc3pe(&mut self) -> Oc3peW<Ccmr2OutputSpec> {
        Oc3peW::new(self, 3)
    }
    #[doc = "Bits 4:6 - Output compare 3 mode"]
    #[inline(always)]
    pub fn oc3m(&mut self) -> Oc3mW<Ccmr2OutputSpec> {
        Oc3mW::new(self, 4)
    }
    #[doc = "Bit 7 - Output compare 3 clear enable"]
    #[inline(always)]
    pub fn oc3ce(&mut self) -> Oc3ceW<Ccmr2OutputSpec> {
        Oc3ceW::new(self, 7)
    }
    #[doc = "Bit 10 - Output compare 4 fast enable"]
    #[inline(always)]
    pub fn oc4fe(&mut self) -> Oc4feW<Ccmr2OutputSpec> {
        Oc4feW::new(self, 10)
    }
    #[doc = "Bit 11 - Output compare 4 preload enable"]
    #[inline(always)]
    pub fn oc4pe(&mut self) -> Oc4peW<Ccmr2OutputSpec> {
        Oc4peW::new(self, 11)
    }
    #[doc = "Bits 12:14 - Output compare 4 mode"]
    #[inline(always)]
    pub fn oc4m(&mut self) -> Oc4mW<Ccmr2OutputSpec> {
        Oc4mW::new(self, 12)
    }
    #[doc = "Bit 15 - Output compare 4 clear enable"]
    #[inline(always)]
    pub fn oc4ce(&mut self) -> Oc4ceW<Ccmr2OutputSpec> {
        Oc4ceW::new(self, 15)
    }
}
#[doc = "compare mode register 2(output mode)\n\nYou can [`read`](crate::Reg::read) this register and get [`ccmr2_output::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccmr2_output::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ccmr2OutputSpec;
impl crate::RegisterSpec for Ccmr2OutputSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccmr2_output::R`](R) reader structure"]
impl crate::Readable for Ccmr2OutputSpec {}
#[doc = "`write(|w| ..)` method takes [`ccmr2_output::W`](W) writer structure"]
impl crate::Writable for Ccmr2OutputSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CCMR2_Output to value 0"]
impl crate::Resettable for Ccmr2OutputSpec {}
