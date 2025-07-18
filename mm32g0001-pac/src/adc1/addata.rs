#[doc = "Register `ADDATA` reader"]
pub type R = crate::R<AddataSpec>;
#[doc = "Field `DATA` reader - Transfer data"]
pub type DataR = crate::FieldReader<u16>;
#[doc = "Field `CH` reader - Channel selection"]
pub type ChR = crate::FieldReader;
#[doc = "Field `OVERRUN` reader - Overrun flag"]
pub type OverrunR = crate::BitReader;
#[doc = "Field `VALID` reader - Valid flag"]
pub type ValidR = crate::BitReader;
impl R {
    #[doc = "Bits 0:15 - Transfer data"]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:19 - Channel selection"]
    #[inline(always)]
    pub fn ch(&self) -> ChR {
        ChR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 20 - Overrun flag"]
    #[inline(always)]
    pub fn overrun(&self) -> OverrunR {
        OverrunR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Valid flag"]
    #[inline(always)]
    pub fn valid(&self) -> ValidR {
        ValidR::new(((self.bits >> 21) & 1) != 0)
    }
}
#[doc = "Data register\n\nYou can [`read`](crate::Reg::read) this register and get [`addata::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AddataSpec;
impl crate::RegisterSpec for AddataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`addata::R`](R) reader structure"]
impl crate::Readable for AddataSpec {}
#[doc = "`reset()` method sets ADDATA to value 0"]
impl crate::Resettable for AddataSpec {}
