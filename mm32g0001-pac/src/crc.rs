#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    dr: Dr,
    idr: Idr,
    ctrl: Ctrl,
}
impl RegisterBlock {
    #[doc = "0x00 - Data register"]
    #[inline(always)]
    pub const fn dr(&self) -> &Dr {
        &self.dr
    }
    #[doc = "0x04 - Independent data register"]
    #[inline(always)]
    pub const fn idr(&self) -> &Idr {
        &self.idr
    }
    #[doc = "0x08 - Control register"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &Ctrl {
        &self.ctrl
    }
}
#[doc = "DR (rw) register accessor: Data register\n\nYou can [`read`](crate::Reg::read) this register and get [`dr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr`] module"]
#[doc(alias = "DR")]
pub type Dr = crate::Reg<dr::DrSpec>;
#[doc = "Data register"]
pub mod dr;
#[doc = "IDR (rw) register accessor: Independent data register\n\nYou can [`read`](crate::Reg::read) this register and get [`idr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`idr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@idr`] module"]
#[doc(alias = "IDR")]
pub type Idr = crate::Reg<idr::IdrSpec>;
#[doc = "Independent data register"]
pub mod idr;
#[doc = "CTRL (w) register accessor: Control register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`] module"]
#[doc(alias = "CTRL")]
pub type Ctrl = crate::Reg<ctrl::CtrlSpec>;
#[doc = "Control register"]
pub mod ctrl;
