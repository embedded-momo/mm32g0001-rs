#[doc = "Register `CCMR3_Output` reader"]
pub type R = crate::R<Ccmr3OutputSpec>;
#[doc = "Register `CCMR3_Output` writer"]
pub type W = crate::W<Ccmr3OutputSpec>;
#[doc = "Field `OC5FE` reader - Output compare 5 fast enable"]
pub type Oc5feR = crate::BitReader;
#[doc = "Field `OC5FE` writer - Output compare 5 fast enable"]
pub type Oc5feW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OC5PE` reader - Output compare 5 preload enable"]
pub type Oc5peR = crate::BitReader;
#[doc = "Field `OC5PE` writer - Output compare 5 preload enable"]
pub type Oc5peW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OC5M` reader - Output compare 5 mode"]
pub type Oc5mR = crate::FieldReader;
#[doc = "Field `OC5M` writer - Output compare 5 mode"]
pub type Oc5mW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 2 - Output compare 5 fast enable"]
    #[inline(always)]
    pub fn oc5fe(&self) -> Oc5feR {
        Oc5feR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Output compare 5 preload enable"]
    #[inline(always)]
    pub fn oc5pe(&self) -> Oc5peR {
        Oc5peR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Output compare 5 mode"]
    #[inline(always)]
    pub fn oc5m(&self) -> Oc5mR {
        Oc5mR::new(((self.bits >> 4) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 2 - Output compare 5 fast enable"]
    #[inline(always)]
    pub fn oc5fe(&mut self) -> Oc5feW<Ccmr3OutputSpec> {
        Oc5feW::new(self, 2)
    }
    #[doc = "Bit 3 - Output compare 5 preload enable"]
    #[inline(always)]
    pub fn oc5pe(&mut self) -> Oc5peW<Ccmr3OutputSpec> {
        Oc5peW::new(self, 3)
    }
    #[doc = "Bits 4:6 - Output compare 5 mode"]
    #[inline(always)]
    pub fn oc5m(&mut self) -> Oc5mW<Ccmr3OutputSpec> {
        Oc5mW::new(self, 4)
    }
}
#[doc = "compare mode register 3 (output mode)\n\nYou can [`read`](crate::Reg::read) this register and get [`ccmr3_output::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccmr3_output::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ccmr3OutputSpec;
impl crate::RegisterSpec for Ccmr3OutputSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccmr3_output::R`](R) reader structure"]
impl crate::Readable for Ccmr3OutputSpec {}
#[doc = "`write(|w| ..)` method takes [`ccmr3_output::W`](W) writer structure"]
impl crate::Writable for Ccmr3OutputSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CCMR3_Output to value 0"]
impl crate::Resettable for Ccmr3OutputSpec {}
