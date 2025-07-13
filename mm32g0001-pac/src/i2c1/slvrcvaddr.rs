#[doc = "Register `SLVRCVADDR` reader"]
pub type R = crate::R<SlvrcvaddrSpec>;
#[doc = "Register `SLVRCVADDR` writer"]
pub type W = crate::W<SlvrcvaddrSpec>;
#[doc = "Field `ADDR` reader - Slave actual received address."]
pub type AddrR = crate::FieldReader<u16>;
#[doc = "Field `ADDR` writer - Slave actual received address."]
pub type AddrW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - Slave actual received address."]
    #[inline(always)]
    pub fn addr(&self) -> AddrR {
        AddrR::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Slave actual received address."]
    #[inline(always)]
    pub fn addr(&mut self) -> AddrW<SlvrcvaddrSpec> {
        AddrW::new(self, 0)
    }
}
#[doc = "Slave received address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`slvrcvaddr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`slvrcvaddr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SlvrcvaddrSpec;
impl crate::RegisterSpec for SlvrcvaddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`slvrcvaddr::R`](R) reader structure"]
impl crate::Readable for SlvrcvaddrSpec {}
#[doc = "`write(|w| ..)` method takes [`slvrcvaddr::W`](W) writer structure"]
impl crate::Writable for SlvrcvaddrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SLVRCVADDR to value 0"]
impl crate::Resettable for SlvrcvaddrSpec {}
