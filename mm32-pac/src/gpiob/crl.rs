#[doc = "Register `CRL` reader"]
pub type R = crate::R<CrlSpec>;
#[doc = "Register `CRL` writer"]
pub type W = crate::W<CrlSpec>;
#[doc = "Field `MODE0` reader - Port 0 mode bits"]
pub type Mode0R = crate::FieldReader;
#[doc = "Field `MODE0` writer - Port 0 mode bits"]
pub type Mode0W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CNF0` reader - Port 0 configuration bits"]
pub type Cnf0R = crate::FieldReader;
#[doc = "Field `CNF0` writer - Port 0 configuration bits"]
pub type Cnf0W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MODE1` reader - Port 1 mode bits"]
pub type Mode1R = crate::FieldReader;
#[doc = "Field `MODE1` writer - Port 1 mode bits"]
pub type Mode1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CNF1` reader - Port 1 configuration bits"]
pub type Cnf1R = crate::FieldReader;
#[doc = "Field `CNF1` writer - Port 1 configuration bits"]
pub type Cnf1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - Port 0 mode bits"]
    #[inline(always)]
    pub fn mode0(&self) -> Mode0R {
        Mode0R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Port 0 configuration bits"]
    #[inline(always)]
    pub fn cnf0(&self) -> Cnf0R {
        Cnf0R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Port 1 mode bits"]
    #[inline(always)]
    pub fn mode1(&self) -> Mode1R {
        Mode1R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Port 1 configuration bits"]
    #[inline(always)]
    pub fn cnf1(&self) -> Cnf1R {
        Cnf1R::new(((self.bits >> 6) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Port 0 mode bits"]
    #[inline(always)]
    pub fn mode0(&mut self) -> Mode0W<CrlSpec> {
        Mode0W::new(self, 0)
    }
    #[doc = "Bits 2:3 - Port 0 configuration bits"]
    #[inline(always)]
    pub fn cnf0(&mut self) -> Cnf0W<CrlSpec> {
        Cnf0W::new(self, 2)
    }
    #[doc = "Bits 4:5 - Port 1 mode bits"]
    #[inline(always)]
    pub fn mode1(&mut self) -> Mode1W<CrlSpec> {
        Mode1W::new(self, 4)
    }
    #[doc = "Bits 6:7 - Port 1 configuration bits"]
    #[inline(always)]
    pub fn cnf1(&mut self) -> Cnf1W<CrlSpec> {
        Cnf1W::new(self, 6)
    }
}
#[doc = "configuration low register\n\nYou can [`read`](crate::Reg::read) this register and get [`crl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrlSpec;
impl crate::RegisterSpec for CrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`crl::R`](R) reader structure"]
impl crate::Readable for CrlSpec {}
#[doc = "`write(|w| ..)` method takes [`crl::W`](W) writer structure"]
impl crate::Writable for CrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CRL to value 0x4444_4444"]
impl crate::Resettable for CrlSpec {
    const RESET_VALUE: u32 = 0x4444_4444;
}
