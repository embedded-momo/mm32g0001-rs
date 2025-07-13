#[doc = "Register `CR` reader"]
pub type R = crate::R<CrSpec>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CrSpec>;
#[doc = "Field `DBG_SLEEP` reader - Can be configured as any channel from ch0 to 9, 14 to 15."]
pub type DbgSleepR = crate::BitReader;
#[doc = "Field `DBG_SLEEP` writer - Can be configured as any channel from ch0 to 9, 14 to 15."]
pub type DbgSleepW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_STOP` reader - Debug Stop mode"]
pub type DbgStopR = crate::BitReader;
#[doc = "Field `DBG_STOP` writer - Debug Stop mode"]
pub type DbgStopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_STOP_FOR_LDO` reader - Debug Stop Ldo"]
pub type DbgStopForLdoR = crate::BitReader;
#[doc = "Field `DBG_STOP_FOR_LDO` writer - Debug Stop Ldo"]
pub type DbgStopForLdoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_IWDG_STOP` reader - Debug independent watchdog stopped when core is stopped"]
pub type DbgIwdgStopR = crate::BitReader;
#[doc = "Field `DBG_IWDG_STOP` writer - Debug independent watchdog stopped when core is stopped"]
pub type DbgIwdgStopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_TIM1_STOP` reader - TIM1 counter stopped when core is halted"]
pub type DbgTim1StopR = crate::BitReader;
#[doc = "Field `DBG_TIM1_STOP` writer - TIM1 counter stopped when core is halted"]
pub type DbgTim1StopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_TIM3_STOP` reader - TIM3 counter stopped when core is halted"]
pub type DbgTim3StopR = crate::BitReader;
#[doc = "Field `DBG_TIM3_STOP` writer - TIM3 counter stopped when core is halted"]
pub type DbgTim3StopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_TIM1_PWM_OFF` reader - TIM1 PWM ouput 0 when core is halted"]
pub type DbgTim1PwmOffR = crate::BitReader;
#[doc = "Field `DBG_TIM1_PWM_OFF` writer - TIM1 PWM ouput 0 when core is halted"]
pub type DbgTim1PwmOffW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_TIM3_PWM_OFF` reader - TIM3 PWM ouput 0 when core is halted"]
pub type DbgTim3PwmOffR = crate::BitReader;
#[doc = "Field `DBG_TIM3_PWM_OFF` writer - TIM3 PWM ouput 0 when core is halted"]
pub type DbgTim3PwmOffW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_TIM14_STOP` reader - TIM14 counter stopped when core is halted"]
pub type DbgTim14StopR = crate::BitReader;
#[doc = "Field `DBG_TIM14_STOP` writer - TIM14 counter stopped when core is halted"]
pub type DbgTim14StopW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Can be configured as any channel from ch0 to 9, 14 to 15."]
    #[inline(always)]
    pub fn dbg_sleep(&self) -> DbgSleepR {
        DbgSleepR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Debug Stop mode"]
    #[inline(always)]
    pub fn dbg_stop(&self) -> DbgStopR {
        DbgStopR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Debug Stop Ldo"]
    #[inline(always)]
    pub fn dbg_stop_for_ldo(&self) -> DbgStopForLdoR {
        DbgStopForLdoR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - Debug independent watchdog stopped when core is stopped"]
    #[inline(always)]
    pub fn dbg_iwdg_stop(&self) -> DbgIwdgStopR {
        DbgIwdgStopR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - TIM1 counter stopped when core is halted"]
    #[inline(always)]
    pub fn dbg_tim1_stop(&self) -> DbgTim1StopR {
        DbgTim1StopR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12 - TIM3 counter stopped when core is halted"]
    #[inline(always)]
    pub fn dbg_tim3_stop(&self) -> DbgTim3StopR {
        DbgTim3StopR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - TIM1 PWM ouput 0 when core is halted"]
    #[inline(always)]
    pub fn dbg_tim1_pwm_off(&self) -> DbgTim1PwmOffR {
        DbgTim1PwmOffR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 15 - TIM3 PWM ouput 0 when core is halted"]
    #[inline(always)]
    pub fn dbg_tim3_pwm_off(&self) -> DbgTim3PwmOffR {
        DbgTim3PwmOffR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 18 - TIM14 counter stopped when core is halted"]
    #[inline(always)]
    pub fn dbg_tim14_stop(&self) -> DbgTim14StopR {
        DbgTim14StopR::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Can be configured as any channel from ch0 to 9, 14 to 15."]
    #[inline(always)]
    pub fn dbg_sleep(&mut self) -> DbgSleepW<CrSpec> {
        DbgSleepW::new(self, 0)
    }
    #[doc = "Bit 1 - Debug Stop mode"]
    #[inline(always)]
    pub fn dbg_stop(&mut self) -> DbgStopW<CrSpec> {
        DbgStopW::new(self, 1)
    }
    #[doc = "Bit 3 - Debug Stop Ldo"]
    #[inline(always)]
    pub fn dbg_stop_for_ldo(&mut self) -> DbgStopForLdoW<CrSpec> {
        DbgStopForLdoW::new(self, 3)
    }
    #[doc = "Bit 8 - Debug independent watchdog stopped when core is stopped"]
    #[inline(always)]
    pub fn dbg_iwdg_stop(&mut self) -> DbgIwdgStopW<CrSpec> {
        DbgIwdgStopW::new(self, 8)
    }
    #[doc = "Bit 10 - TIM1 counter stopped when core is halted"]
    #[inline(always)]
    pub fn dbg_tim1_stop(&mut self) -> DbgTim1StopW<CrSpec> {
        DbgTim1StopW::new(self, 10)
    }
    #[doc = "Bit 12 - TIM3 counter stopped when core is halted"]
    #[inline(always)]
    pub fn dbg_tim3_stop(&mut self) -> DbgTim3StopW<CrSpec> {
        DbgTim3StopW::new(self, 12)
    }
    #[doc = "Bit 13 - TIM1 PWM ouput 0 when core is halted"]
    #[inline(always)]
    pub fn dbg_tim1_pwm_off(&mut self) -> DbgTim1PwmOffW<CrSpec> {
        DbgTim1PwmOffW::new(self, 13)
    }
    #[doc = "Bit 15 - TIM3 PWM ouput 0 when core is halted"]
    #[inline(always)]
    pub fn dbg_tim3_pwm_off(&mut self) -> DbgTim3PwmOffW<CrSpec> {
        DbgTim3PwmOffW::new(self, 15)
    }
    #[doc = "Bit 18 - TIM14 counter stopped when core is halted"]
    #[inline(always)]
    pub fn dbg_tim14_stop(&mut self) -> DbgTim14StopW<CrSpec> {
        DbgTim14StopW::new(self, 18)
    }
}
#[doc = "Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrSpec;
impl crate::RegisterSpec for CrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr::R`](R) reader structure"]
impl crate::Readable for CrSpec {}
#[doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"]
impl crate::Writable for CrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CrSpec {}
