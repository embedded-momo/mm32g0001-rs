#[doc = "Register `OBR` reader"]
pub type R = crate::R<ObrSpec>;
#[doc = "Field `OPTERR` reader - Option byte error"]
pub type OpterrR = crate::BitReader;
#[doc = "Field `RDPRT` reader - Read protection level status"]
pub type RdprtR = crate::BitReader;
#[doc = "Field `WDG_SW` reader - User option bytes"]
pub type WdgSwR = crate::BitReader;
#[doc = "Field `nRST_STOP` reader - User option bytes"]
pub type NRstStopR = crate::BitReader;
#[doc = "Field `nRST_STDBY` reader - User option bytes"]
pub type NRstStdbyR = crate::BitReader;
#[doc = "Field `nBOOT1` reader - User option bytes"]
pub type NBoot1R = crate::BitReader;
#[doc = "Field `Data0` reader - Data0"]
pub type Data0R = crate::FieldReader;
#[doc = "Field `Data1` reader - Data1"]
pub type Data1R = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - Option byte error"]
    #[inline(always)]
    pub fn opterr(&self) -> OpterrR {
        OpterrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Read protection level status"]
    #[inline(always)]
    pub fn rdprt(&self) -> RdprtR {
        RdprtR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - User option bytes"]
    #[inline(always)]
    pub fn wdg_sw(&self) -> WdgSwR {
        WdgSwR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - User option bytes"]
    #[inline(always)]
    pub fn n_rst_stop(&self) -> NRstStopR {
        NRstStopR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - User option bytes"]
    #[inline(always)]
    pub fn n_rst_stdby(&self) -> NRstStdbyR {
        NRstStdbyR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - User option bytes"]
    #[inline(always)]
    pub fn n_boot1(&self) -> NBoot1R {
        NBoot1R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 10:17 - Data0"]
    #[inline(always)]
    pub fn data0(&self) -> Data0R {
        Data0R::new(((self.bits >> 10) & 0xff) as u8)
    }
    #[doc = "Bits 18:25 - Data1"]
    #[inline(always)]
    pub fn data1(&self) -> Data1R {
        Data1R::new(((self.bits >> 18) & 0xff) as u8)
    }
}
#[doc = "Option byte register\n\nYou can [`read`](crate::Reg::read) this register and get [`obr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ObrSpec;
impl crate::RegisterSpec for ObrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`obr::R`](R) reader structure"]
impl crate::Readable for ObrSpec {}
#[doc = "`reset()` method sets OBR to value 0x03ff_fc6c"]
impl crate::Resettable for ObrSpec {
    const RESET_VALUE: u32 = 0x03ff_fc6c;
}
