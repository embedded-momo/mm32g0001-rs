#[doc = "Register `ADCFGR` reader"]
pub type R = crate::R<AdcfgrSpec>;
#[doc = "Register `ADCFGR` writer"]
pub type W = crate::W<AdcfgrSpec>;
#[doc = "Field `ADEN` reader - ADC enable"]
pub type AdenR = crate::BitReader;
#[doc = "Field `ADEN` writer - ADC enable"]
pub type AdenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AWDEN` reader - ADC window comparison enable"]
pub type AwdenR = crate::BitReader;
#[doc = "Field `AWDEN` writer - ADC window comparison enable"]
pub type AwdenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VSEN` reader - Reference voltage enable"]
pub type VsenR = crate::BitReader;
#[doc = "Field `VSEN` writer - Reference voltage enable"]
pub type VsenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADCPREH` reader - ADC high prescaler coefficient"]
pub type AdcprehR = crate::FieldReader;
#[doc = "Field `ADCPREH` writer - ADC high prescaler coefficient"]
pub type AdcprehW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RSLTCTL` reader - Resolution"]
pub type RsltctlR = crate::FieldReader;
#[doc = "Field `RSLTCTL` writer - Resolution"]
pub type RsltctlW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SAMCTL` reader - Channel x Sample time selection"]
pub type SamctlR = crate::FieldReader;
#[doc = "Field `SAMCTL` writer - Channel x Sample time selection"]
pub type SamctlW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ADCPREL` reader - ADC low prescaler coefficient"]
pub type AdcprelR = crate::BitReader;
#[doc = "Field `ADCPREL` writer - ADC low prescaler coefficient"]
pub type AdcprelW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - ADC enable"]
    #[inline(always)]
    pub fn aden(&self) -> AdenR {
        AdenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ADC window comparison enable"]
    #[inline(always)]
    pub fn awden(&self) -> AwdenR {
        AwdenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Reference voltage enable"]
    #[inline(always)]
    pub fn vsen(&self) -> VsenR {
        VsenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - ADC high prescaler coefficient"]
    #[inline(always)]
    pub fn adcpreh(&self) -> AdcprehR {
        AdcprehR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 7:9 - Resolution"]
    #[inline(always)]
    pub fn rsltctl(&self) -> RsltctlR {
        RsltctlR::new(((self.bits >> 7) & 7) as u8)
    }
    #[doc = "Bits 10:13 - Channel x Sample time selection"]
    #[inline(always)]
    pub fn samctl(&self) -> SamctlR {
        SamctlR::new(((self.bits >> 10) & 0x0f) as u8)
    }
    #[doc = "Bit 14 - ADC low prescaler coefficient"]
    #[inline(always)]
    pub fn adcprel(&self) -> AdcprelR {
        AdcprelR::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ADC enable"]
    #[inline(always)]
    pub fn aden(&mut self) -> AdenW<AdcfgrSpec> {
        AdenW::new(self, 0)
    }
    #[doc = "Bit 1 - ADC window comparison enable"]
    #[inline(always)]
    pub fn awden(&mut self) -> AwdenW<AdcfgrSpec> {
        AwdenW::new(self, 1)
    }
    #[doc = "Bit 3 - Reference voltage enable"]
    #[inline(always)]
    pub fn vsen(&mut self) -> VsenW<AdcfgrSpec> {
        VsenW::new(self, 3)
    }
    #[doc = "Bits 4:6 - ADC high prescaler coefficient"]
    #[inline(always)]
    pub fn adcpreh(&mut self) -> AdcprehW<AdcfgrSpec> {
        AdcprehW::new(self, 4)
    }
    #[doc = "Bits 7:9 - Resolution"]
    #[inline(always)]
    pub fn rsltctl(&mut self) -> RsltctlW<AdcfgrSpec> {
        RsltctlW::new(self, 7)
    }
    #[doc = "Bits 10:13 - Channel x Sample time selection"]
    #[inline(always)]
    pub fn samctl(&mut self) -> SamctlW<AdcfgrSpec> {
        SamctlW::new(self, 10)
    }
    #[doc = "Bit 14 - ADC low prescaler coefficient"]
    #[inline(always)]
    pub fn adcprel(&mut self) -> AdcprelW<AdcfgrSpec> {
        AdcprelW::new(self, 14)
    }
}
#[doc = "Configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`adcfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AdcfgrSpec;
impl crate::RegisterSpec for AdcfgrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adcfgr::R`](R) reader structure"]
impl crate::Readable for AdcfgrSpec {}
#[doc = "`write(|w| ..)` method takes [`adcfgr::W`](W) writer structure"]
impl crate::Writable for AdcfgrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ADCFGR to value 0"]
impl crate::Resettable for AdcfgrSpec {}
