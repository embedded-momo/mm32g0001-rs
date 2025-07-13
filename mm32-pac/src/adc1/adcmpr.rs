#[doc = "Register `ADCMPR` reader"]
pub type R = crate::R<AdcmprSpec>;
#[doc = "Register `ADCMPR` writer"]
pub type W = crate::W<AdcmprSpec>;
#[doc = "Field `CMPLDATA` reader - Compare data low limit"]
pub type CmpldataR = crate::FieldReader<u16>;
#[doc = "Field `CMPLDATA` writer - Compare data low limit"]
pub type CmpldataW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `CMPHDATA` reader - Compare data high limit"]
pub type CmphdataR = crate::FieldReader<u16>;
#[doc = "Field `CMPHDATA` writer - Compare data high limit"]
pub type CmphdataW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - Compare data low limit"]
    #[inline(always)]
    pub fn cmpldata(&self) -> CmpldataR {
        CmpldataR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - Compare data high limit"]
    #[inline(always)]
    pub fn cmphdata(&self) -> CmphdataR {
        CmphdataR::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Compare data low limit"]
    #[inline(always)]
    pub fn cmpldata(&mut self) -> CmpldataW<AdcmprSpec> {
        CmpldataW::new(self, 0)
    }
    #[doc = "Bits 16:27 - Compare data high limit"]
    #[inline(always)]
    pub fn cmphdata(&mut self) -> CmphdataW<AdcmprSpec> {
        CmphdataW::new(self, 16)
    }
}
#[doc = "Compare register\n\nYou can [`read`](crate::Reg::read) this register and get [`adcmpr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcmpr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AdcmprSpec;
impl crate::RegisterSpec for AdcmprSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adcmpr::R`](R) reader structure"]
impl crate::Readable for AdcmprSpec {}
#[doc = "`write(|w| ..)` method takes [`adcmpr::W`](W) writer structure"]
impl crate::Writable for AdcmprSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ADCMPR to value 0"]
impl crate::Resettable for AdcmprSpec {}
