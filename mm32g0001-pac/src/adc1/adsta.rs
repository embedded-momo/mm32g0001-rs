#[doc = "Register `ADSTA` reader"]
pub type R = crate::R<AdstaSpec>;
#[doc = "Field `ADIF` reader - ADC interrupt flag"]
pub type AdifR = crate::BitReader;
#[doc = "Field `AWDIF` reader - ADC window comparator interrupt flag"]
pub type AwdifR = crate::BitReader;
#[doc = "Field `BUSY` reader - Busy"]
pub type BusyR = crate::BitReader;
#[doc = "Field `CH` reader - Current conversion channel"]
pub type ChR = crate::FieldReader;
#[doc = "Field `VALID` reader - Valid flag"]
pub type ValidR = crate::FieldReader<u16>;
#[doc = "Field `OVERRUN` reader - Overrun flag"]
pub type OverrunR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bit 0 - ADC interrupt flag"]
    #[inline(always)]
    pub fn adif(&self) -> AdifR {
        AdifR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ADC window comparator interrupt flag"]
    #[inline(always)]
    pub fn awdif(&self) -> AwdifR {
        AwdifR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Busy"]
    #[inline(always)]
    pub fn busy(&self) -> BusyR {
        BusyR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:7 - Current conversion channel"]
    #[inline(always)]
    pub fn ch(&self) -> ChR {
        ChR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:16 - Valid flag"]
    #[inline(always)]
    pub fn valid(&self) -> ValidR {
        ValidR::new(((self.bits >> 8) & 0x01ff) as u16)
    }
    #[doc = "Bits 20:29 - Overrun flag"]
    #[inline(always)]
    pub fn overrun(&self) -> OverrunR {
        OverrunR::new(((self.bits >> 20) & 0x03ff) as u16)
    }
}
#[doc = "Status register\n\nYou can [`read`](crate::Reg::read) this register and get [`adsta::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AdstaSpec;
impl crate::RegisterSpec for AdstaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adsta::R`](R) reader structure"]
impl crate::Readable for AdstaSpec {}
#[doc = "`reset()` method sets ADSTA to value 0"]
impl crate::Resettable for AdstaSpec {}
