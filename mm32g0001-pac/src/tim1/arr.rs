#[doc = "Register `ARR` reader"]
pub type R = crate::R<ArrSpec>;
#[doc = "Register `ARR` writer"]
pub type W = crate::W<ArrSpec>;
#[doc = "Field `ARR` reader - Auto reload value"]
pub type ArrR = crate::FieldReader<u16>;
#[doc = "Field `ARR` writer - Auto reload value"]
pub type ArrW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Auto reload value"]
    #[inline(always)]
    pub fn arr(&self) -> ArrR {
        ArrR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Auto reload value"]
    #[inline(always)]
    pub fn arr(&mut self) -> ArrW<ArrSpec> {
        ArrW::new(self, 0)
    }
}
#[doc = "auto-reload register\n\nYou can [`read`](crate::Reg::read) this register and get [`arr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`arr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ArrSpec;
impl crate::RegisterSpec for ArrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`arr::R`](R) reader structure"]
impl crate::Readable for ArrSpec {}
#[doc = "`write(|w| ..)` method takes [`arr::W`](W) writer structure"]
impl crate::Writable for ArrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ARR to value 0"]
impl crate::Resettable for ArrSpec {}
