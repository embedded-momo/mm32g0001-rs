#[doc = "Register `OR` reader"]
pub type R = crate::R<OrSpec>;
#[doc = "Register `OR` writer"]
pub type W = crate::W<OrSpec>;
#[doc = "Field `ETR_RMP` reader - ETR remap"]
pub type EtrRmpR = crate::FieldReader;
#[doc = "Field `ETR_RMP` writer - ETR remap"]
pub type EtrRmpW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - ETR remap"]
    #[inline(always)]
    pub fn etr_rmp(&self) -> EtrRmpR {
        EtrRmpR::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - ETR remap"]
    #[inline(always)]
    pub fn etr_rmp(&mut self) -> EtrRmpW<OrSpec> {
        EtrRmpW::new(self, 0)
    }
}
#[doc = "option register\n\nYou can [`read`](crate::Reg::read) this register and get [`or::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`or::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OrSpec;
impl crate::RegisterSpec for OrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`or::R`](R) reader structure"]
impl crate::Readable for OrSpec {}
#[doc = "`write(|w| ..)` method takes [`or::W`](W) writer structure"]
impl crate::Writable for OrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OR to value 0"]
impl crate::Resettable for OrSpec {}
