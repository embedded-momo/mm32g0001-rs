#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    addata: Addata,
    adcfgr: Adcfgr,
    adcr: Adcr,
    _reserved3: [u8; 0x04],
    adcmpr: Adcmpr,
    adsta: Adsta,
    dr0: Dr0,
    dr1: Dr1,
    dr2: Dr2,
    dr3: Dr3,
    dr4: Dr4,
    dr5: Dr5,
    dr6: Dr6,
    dr7: Dr7,
    dr8: Dr8,
    _reserved14: [u8; 0x20],
    chany0: Chany0,
    chany1: Chany1,
    any_cfg: AnyCfg,
    any_cr: AnyCr,
}
impl RegisterBlock {
    #[doc = "0x00 - Data register"]
    #[inline(always)]
    pub const fn addata(&self) -> &Addata {
        &self.addata
    }
    #[doc = "0x04 - Configure register"]
    #[inline(always)]
    pub const fn adcfgr(&self) -> &Adcfgr {
        &self.adcfgr
    }
    #[doc = "0x08 - Control register"]
    #[inline(always)]
    pub const fn adcr(&self) -> &Adcr {
        &self.adcr
    }
    #[doc = "0x10 - Compare register"]
    #[inline(always)]
    pub const fn adcmpr(&self) -> &Adcmpr {
        &self.adcmpr
    }
    #[doc = "0x14 - Status register"]
    #[inline(always)]
    pub const fn adsta(&self) -> &Adsta {
        &self.adsta
    }
    #[doc = "0x18 - Channel 0 data register"]
    #[inline(always)]
    pub const fn dr0(&self) -> &Dr0 {
        &self.dr0
    }
    #[doc = "0x1c - Channel 1 data register"]
    #[inline(always)]
    pub const fn dr1(&self) -> &Dr1 {
        &self.dr1
    }
    #[doc = "0x20 - Channel 2 data register"]
    #[inline(always)]
    pub const fn dr2(&self) -> &Dr2 {
        &self.dr2
    }
    #[doc = "0x24 - Channel 3 data register"]
    #[inline(always)]
    pub const fn dr3(&self) -> &Dr3 {
        &self.dr3
    }
    #[doc = "0x28 - Channel 4 data register"]
    #[inline(always)]
    pub const fn dr4(&self) -> &Dr4 {
        &self.dr4
    }
    #[doc = "0x2c - Channel 5 data register"]
    #[inline(always)]
    pub const fn dr5(&self) -> &Dr5 {
        &self.dr5
    }
    #[doc = "0x30 - Channel 6 data register"]
    #[inline(always)]
    pub const fn dr6(&self) -> &Dr6 {
        &self.dr6
    }
    #[doc = "0x34 - Channel 7 data register"]
    #[inline(always)]
    pub const fn dr7(&self) -> &Dr7 {
        &self.dr7
    }
    #[doc = "0x38 - Channel 8 data register"]
    #[inline(always)]
    pub const fn dr8(&self) -> &Dr8 {
        &self.dr8
    }
    #[doc = "0x5c - Arbitrary channel channel selection register 0"]
    #[inline(always)]
    pub const fn chany0(&self) -> &Chany0 {
        &self.chany0
    }
    #[doc = "0x60 - Arbitrary channel channel selection register 1"]
    #[inline(always)]
    pub const fn chany1(&self) -> &Chany1 {
        &self.chany1
    }
    #[doc = "0x64 - Arbitrary channel configuration register"]
    #[inline(always)]
    pub const fn any_cfg(&self) -> &AnyCfg {
        &self.any_cfg
    }
    #[doc = "0x68 - Arbitrary channel control register"]
    #[inline(always)]
    pub const fn any_cr(&self) -> &AnyCr {
        &self.any_cr
    }
}
#[doc = "ADDATA (r) register accessor: Data register\n\nYou can [`read`](crate::Reg::read) this register and get [`addata::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@addata`] module"]
#[doc(alias = "ADDATA")]
pub type Addata = crate::Reg<addata::AddataSpec>;
#[doc = "Data register"]
pub mod addata;
#[doc = "ADCFGR (rw) register accessor: Configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`adcfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcfgr`] module"]
#[doc(alias = "ADCFGR")]
pub type Adcfgr = crate::Reg<adcfgr::AdcfgrSpec>;
#[doc = "Configure register"]
pub mod adcfgr;
#[doc = "ADCR (rw) register accessor: Control register\n\nYou can [`read`](crate::Reg::read) this register and get [`adcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcr`] module"]
#[doc(alias = "ADCR")]
pub type Adcr = crate::Reg<adcr::AdcrSpec>;
#[doc = "Control register"]
pub mod adcr;
#[doc = "ADCMPR (rw) register accessor: Compare register\n\nYou can [`read`](crate::Reg::read) this register and get [`adcmpr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcmpr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcmpr`] module"]
#[doc(alias = "ADCMPR")]
pub type Adcmpr = crate::Reg<adcmpr::AdcmprSpec>;
#[doc = "Compare register"]
pub mod adcmpr;
#[doc = "ADSTA (r) register accessor: Status register\n\nYou can [`read`](crate::Reg::read) this register and get [`adsta::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adsta`] module"]
#[doc(alias = "ADSTA")]
pub type Adsta = crate::Reg<adsta::AdstaSpec>;
#[doc = "Status register"]
pub mod adsta;
#[doc = "DR0 (r) register accessor: Channel 0 data register\n\nYou can [`read`](crate::Reg::read) this register and get [`dr0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr0`] module"]
#[doc(alias = "DR0")]
pub type Dr0 = crate::Reg<dr0::Dr0Spec>;
#[doc = "Channel 0 data register"]
pub mod dr0;
#[doc = "DR1 (r) register accessor: Channel 1 data register\n\nYou can [`read`](crate::Reg::read) this register and get [`dr1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr1`] module"]
#[doc(alias = "DR1")]
pub type Dr1 = crate::Reg<dr1::Dr1Spec>;
#[doc = "Channel 1 data register"]
pub mod dr1;
#[doc = "DR2 (r) register accessor: Channel 2 data register\n\nYou can [`read`](crate::Reg::read) this register and get [`dr2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr2`] module"]
#[doc(alias = "DR2")]
pub type Dr2 = crate::Reg<dr2::Dr2Spec>;
#[doc = "Channel 2 data register"]
pub mod dr2;
#[doc = "DR3 (r) register accessor: Channel 3 data register\n\nYou can [`read`](crate::Reg::read) this register and get [`dr3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr3`] module"]
#[doc(alias = "DR3")]
pub type Dr3 = crate::Reg<dr3::Dr3Spec>;
#[doc = "Channel 3 data register"]
pub mod dr3;
#[doc = "DR4 (r) register accessor: Channel 4 data register\n\nYou can [`read`](crate::Reg::read) this register and get [`dr4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr4`] module"]
#[doc(alias = "DR4")]
pub type Dr4 = crate::Reg<dr4::Dr4Spec>;
#[doc = "Channel 4 data register"]
pub mod dr4;
#[doc = "DR5 (r) register accessor: Channel 5 data register\n\nYou can [`read`](crate::Reg::read) this register and get [`dr5::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr5`] module"]
#[doc(alias = "DR5")]
pub type Dr5 = crate::Reg<dr5::Dr5Spec>;
#[doc = "Channel 5 data register"]
pub mod dr5;
#[doc = "DR6 (r) register accessor: Channel 6 data register\n\nYou can [`read`](crate::Reg::read) this register and get [`dr6::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr6`] module"]
#[doc(alias = "DR6")]
pub type Dr6 = crate::Reg<dr6::Dr6Spec>;
#[doc = "Channel 6 data register"]
pub mod dr6;
#[doc = "DR7 (r) register accessor: Channel 7 data register\n\nYou can [`read`](crate::Reg::read) this register and get [`dr7::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr7`] module"]
#[doc(alias = "DR7")]
pub type Dr7 = crate::Reg<dr7::Dr7Spec>;
#[doc = "Channel 7 data register"]
pub mod dr7;
#[doc = "DR8 (r) register accessor: Channel 8 data register\n\nYou can [`read`](crate::Reg::read) this register and get [`dr8::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr8`] module"]
#[doc(alias = "DR8")]
pub type Dr8 = crate::Reg<dr8::Dr8Spec>;
#[doc = "Channel 8 data register"]
pub mod dr8;
#[doc = "CHANY0 (rw) register accessor: Arbitrary channel channel selection register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`chany0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chany0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chany0`] module"]
#[doc(alias = "CHANY0")]
pub type Chany0 = crate::Reg<chany0::Chany0Spec>;
#[doc = "Arbitrary channel channel selection register 0"]
pub mod chany0;
#[doc = "CHANY1 (rw) register accessor: Arbitrary channel channel selection register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`chany1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chany1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chany1`] module"]
#[doc(alias = "CHANY1")]
pub type Chany1 = crate::Reg<chany1::Chany1Spec>;
#[doc = "Arbitrary channel channel selection register 1"]
pub mod chany1;
#[doc = "ANY_CFG (rw) register accessor: Arbitrary channel configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`any_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`any_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@any_cfg`] module"]
#[doc(alias = "ANY_CFG")]
pub type AnyCfg = crate::Reg<any_cfg::AnyCfgSpec>;
#[doc = "Arbitrary channel configuration register"]
pub mod any_cfg;
#[doc = "ANY_CR (rw) register accessor: Arbitrary channel control register\n\nYou can [`read`](crate::Reg::read) this register and get [`any_cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`any_cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@any_cr`] module"]
#[doc(alias = "ANY_CR")]
pub type AnyCr = crate::Reg<any_cr::AnyCrSpec>;
#[doc = "Arbitrary channel control register"]
pub mod any_cr;
