#[doc = "Register `BRR` reader"]
pub type R = crate::R<BrrSpec>;
#[doc = "Register `BRR` writer"]
pub type W = crate::W<BrrSpec>;
#[doc = "Field `FFD` reader - "]
pub type FfdR = crate::FieldReader;
#[doc = "Field `FFD` writer - "]
pub type FfdW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `MFD` reader - "]
pub type MfdR = crate::FieldReader<u16>;
#[doc = "Field `MFD` writer - "]
pub type MfdW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn ffd(&self) -> FfdR {
        FfdR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:19"]
    #[inline(always)]
    pub fn mfd(&self) -> MfdR {
        MfdR::new(((self.bits >> 4) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn ffd(&mut self) -> FfdW<BrrSpec> {
        FfdW::new(self, 0)
    }
    #[doc = "Bits 4:19"]
    #[inline(always)]
    pub fn mfd(&mut self) -> MfdW<BrrSpec> {
        MfdW::new(self, 4)
    }
}
#[doc = "Baud rate register\n\nYou can [`read`](crate::Reg::read) this register and get [`brr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`brr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BrrSpec;
impl crate::RegisterSpec for BrrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`brr::R`](R) reader structure"]
impl crate::Readable for BrrSpec {}
#[doc = "`write(|w| ..)` method takes [`brr::W`](W) writer structure"]
impl crate::Writable for BrrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BRR to value 0"]
impl crate::Resettable for BrrSpec {}
