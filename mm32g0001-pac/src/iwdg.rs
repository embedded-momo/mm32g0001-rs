#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    kr: Kr,
    pr: Pr,
    rlr: Rlr,
    sr: Sr,
    cr: Cr,
    igen: Igen,
    cnt: Cnt,
}
impl RegisterBlock {
    #[doc = "0x00 - Key register"]
    #[inline(always)]
    pub const fn kr(&self) -> &Kr {
        &self.kr
    }
    #[doc = "0x04 - Prescaler register"]
    #[inline(always)]
    pub const fn pr(&self) -> &Pr {
        &self.pr
    }
    #[doc = "0x08 - Reload register"]
    #[inline(always)]
    pub const fn rlr(&self) -> &Rlr {
        &self.rlr
    }
    #[doc = "0x0c - Status register"]
    #[inline(always)]
    pub const fn sr(&self) -> &Sr {
        &self.sr
    }
    #[doc = "0x10 - Control register"]
    #[inline(always)]
    pub const fn cr(&self) -> &Cr {
        &self.cr
    }
    #[doc = "0x14 - Interruput generate value register"]
    #[inline(always)]
    pub const fn igen(&self) -> &Igen {
        &self.igen
    }
    #[doc = "0x18 - Counter"]
    #[inline(always)]
    pub const fn cnt(&self) -> &Cnt {
        &self.cnt
    }
}
#[doc = "KR (w) register accessor: Key register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`kr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@kr`] module"]
#[doc(alias = "KR")]
pub type Kr = crate::Reg<kr::KrSpec>;
#[doc = "Key register"]
pub mod kr;
#[doc = "PR (rw) register accessor: Prescaler register\n\nYou can [`read`](crate::Reg::read) this register and get [`pr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr`] module"]
#[doc(alias = "PR")]
pub type Pr = crate::Reg<pr::PrSpec>;
#[doc = "Prescaler register"]
pub mod pr;
#[doc = "RLR (rw) register accessor: Reload register\n\nYou can [`read`](crate::Reg::read) this register and get [`rlr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rlr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rlr`] module"]
#[doc(alias = "RLR")]
pub type Rlr = crate::Reg<rlr::RlrSpec>;
#[doc = "Reload register"]
pub mod rlr;
#[doc = "SR (r) register accessor: Status register\n\nYou can [`read`](crate::Reg::read) this register and get [`sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr`] module"]
#[doc(alias = "SR")]
pub type Sr = crate::Reg<sr::SrSpec>;
#[doc = "Status register"]
pub mod sr;
#[doc = "CR (rw) register accessor: Control register\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`] module"]
#[doc(alias = "CR")]
pub type Cr = crate::Reg<cr::CrSpec>;
#[doc = "Control register"]
pub mod cr;
#[doc = "IGEN (rw) register accessor: Interruput generate value register\n\nYou can [`read`](crate::Reg::read) this register and get [`igen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`igen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@igen`] module"]
#[doc(alias = "IGEN")]
pub type Igen = crate::Reg<igen::IgenSpec>;
#[doc = "Interruput generate value register"]
pub mod igen;
#[doc = "CNT (rw) register accessor: Counter\n\nYou can [`read`](crate::Reg::read) this register and get [`cnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cnt`] module"]
#[doc(alias = "CNT")]
pub type Cnt = crate::Reg<cnt::CntSpec>;
#[doc = "Counter"]
pub mod cnt;
