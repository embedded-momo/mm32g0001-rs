#[doc = "Register `CCER` reader"]
pub type R = crate::R<CcerSpec>;
#[doc = "Register `CCER` writer"]
pub type W = crate::W<CcerSpec>;
#[doc = "Field `CC1E` reader - Compare 1 output enable"]
pub type Cc1eR = crate::BitReader;
#[doc = "Field `CC1E` writer - Compare 1 output enable"]
pub type Cc1eW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC1P` reader - Compare 1 output polarity"]
pub type Cc1pR = crate::BitReader;
#[doc = "Field `CC1P` writer - Compare 1 output polarity"]
pub type Cc1pW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC1NE` reader - Compare 1 complementary output enable"]
pub type Cc1neR = crate::BitReader;
#[doc = "Field `CC1NE` writer - Compare 1 complementary output enable"]
pub type Cc1neW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC1NP` reader - Compare 1 complementary output polarity"]
pub type Cc1npR = crate::BitReader;
#[doc = "Field `CC1NP` writer - Compare 1 complementary output polarity"]
pub type Cc1npW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC2E` reader - Compare 2 output enable"]
pub type Cc2eR = crate::BitReader;
#[doc = "Field `CC2E` writer - Compare 2 output enable"]
pub type Cc2eW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC2P` reader - Compare 2 output polarity"]
pub type Cc2pR = crate::BitReader;
#[doc = "Field `CC2P` writer - Compare 2 output polarity"]
pub type Cc2pW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC2NE` reader - Compare 2 complementary output enable"]
pub type Cc2neR = crate::BitReader;
#[doc = "Field `CC2NE` writer - Compare 2 complementary output enable"]
pub type Cc2neW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC2NP` reader - Compare 2 complementary output polarity"]
pub type Cc2npR = crate::BitReader;
#[doc = "Field `CC2NP` writer - Compare 2 complementary output polarity"]
pub type Cc2npW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC3E` reader - Compare 3 output enable"]
pub type Cc3eR = crate::BitReader;
#[doc = "Field `CC3E` writer - Compare 3 output enable"]
pub type Cc3eW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC3P` reader - Compare 3 output polarity"]
pub type Cc3pR = crate::BitReader;
#[doc = "Field `CC3P` writer - Compare 3 output polarity"]
pub type Cc3pW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC3NE` reader - Compare 3 complementary output enable"]
pub type Cc3neR = crate::BitReader;
#[doc = "Field `CC3NE` writer - Compare 3 complementary output enable"]
pub type Cc3neW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC3NP` reader - Compare 3 complementary output polarity"]
pub type Cc3npR = crate::BitReader;
#[doc = "Field `CC3NP` writer - Compare 3 complementary output polarity"]
pub type Cc3npW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC4E` reader - Compare 4 output enable"]
pub type Cc4eR = crate::BitReader;
#[doc = "Field `CC4E` writer - Compare 4 output enable"]
pub type Cc4eW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC4P` reader - Compare 4 output polarity"]
pub type Cc4pR = crate::BitReader;
#[doc = "Field `CC4P` writer - Compare 4 output polarity"]
pub type Cc4pW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC4NP` reader - Compare 4 complementary output polarity"]
pub type Cc4npR = crate::BitReader;
#[doc = "Field `CC4NP` writer - Compare 4 complementary output polarity"]
pub type Cc4npW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Compare 1 output enable"]
    #[inline(always)]
    pub fn cc1e(&self) -> Cc1eR {
        Cc1eR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Compare 1 output polarity"]
    #[inline(always)]
    pub fn cc1p(&self) -> Cc1pR {
        Cc1pR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Compare 1 complementary output enable"]
    #[inline(always)]
    pub fn cc1ne(&self) -> Cc1neR {
        Cc1neR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Compare 1 complementary output polarity"]
    #[inline(always)]
    pub fn cc1np(&self) -> Cc1npR {
        Cc1npR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Compare 2 output enable"]
    #[inline(always)]
    pub fn cc2e(&self) -> Cc2eR {
        Cc2eR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Compare 2 output polarity"]
    #[inline(always)]
    pub fn cc2p(&self) -> Cc2pR {
        Cc2pR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Compare 2 complementary output enable"]
    #[inline(always)]
    pub fn cc2ne(&self) -> Cc2neR {
        Cc2neR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Compare 2 complementary output polarity"]
    #[inline(always)]
    pub fn cc2np(&self) -> Cc2npR {
        Cc2npR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Compare 3 output enable"]
    #[inline(always)]
    pub fn cc3e(&self) -> Cc3eR {
        Cc3eR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Compare 3 output polarity"]
    #[inline(always)]
    pub fn cc3p(&self) -> Cc3pR {
        Cc3pR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Compare 3 complementary output enable"]
    #[inline(always)]
    pub fn cc3ne(&self) -> Cc3neR {
        Cc3neR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Compare 3 complementary output polarity"]
    #[inline(always)]
    pub fn cc3np(&self) -> Cc3npR {
        Cc3npR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Compare 4 output enable"]
    #[inline(always)]
    pub fn cc4e(&self) -> Cc4eR {
        Cc4eR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Compare 4 output polarity"]
    #[inline(always)]
    pub fn cc4p(&self) -> Cc4pR {
        Cc4pR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 15 - Compare 4 complementary output polarity"]
    #[inline(always)]
    pub fn cc4np(&self) -> Cc4npR {
        Cc4npR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Compare 1 output enable"]
    #[inline(always)]
    pub fn cc1e(&mut self) -> Cc1eW<CcerSpec> {
        Cc1eW::new(self, 0)
    }
    #[doc = "Bit 1 - Compare 1 output polarity"]
    #[inline(always)]
    pub fn cc1p(&mut self) -> Cc1pW<CcerSpec> {
        Cc1pW::new(self, 1)
    }
    #[doc = "Bit 2 - Compare 1 complementary output enable"]
    #[inline(always)]
    pub fn cc1ne(&mut self) -> Cc1neW<CcerSpec> {
        Cc1neW::new(self, 2)
    }
    #[doc = "Bit 3 - Compare 1 complementary output polarity"]
    #[inline(always)]
    pub fn cc1np(&mut self) -> Cc1npW<CcerSpec> {
        Cc1npW::new(self, 3)
    }
    #[doc = "Bit 4 - Compare 2 output enable"]
    #[inline(always)]
    pub fn cc2e(&mut self) -> Cc2eW<CcerSpec> {
        Cc2eW::new(self, 4)
    }
    #[doc = "Bit 5 - Compare 2 output polarity"]
    #[inline(always)]
    pub fn cc2p(&mut self) -> Cc2pW<CcerSpec> {
        Cc2pW::new(self, 5)
    }
    #[doc = "Bit 6 - Compare 2 complementary output enable"]
    #[inline(always)]
    pub fn cc2ne(&mut self) -> Cc2neW<CcerSpec> {
        Cc2neW::new(self, 6)
    }
    #[doc = "Bit 7 - Compare 2 complementary output polarity"]
    #[inline(always)]
    pub fn cc2np(&mut self) -> Cc2npW<CcerSpec> {
        Cc2npW::new(self, 7)
    }
    #[doc = "Bit 8 - Compare 3 output enable"]
    #[inline(always)]
    pub fn cc3e(&mut self) -> Cc3eW<CcerSpec> {
        Cc3eW::new(self, 8)
    }
    #[doc = "Bit 9 - Compare 3 output polarity"]
    #[inline(always)]
    pub fn cc3p(&mut self) -> Cc3pW<CcerSpec> {
        Cc3pW::new(self, 9)
    }
    #[doc = "Bit 10 - Compare 3 complementary output enable"]
    #[inline(always)]
    pub fn cc3ne(&mut self) -> Cc3neW<CcerSpec> {
        Cc3neW::new(self, 10)
    }
    #[doc = "Bit 11 - Compare 3 complementary output polarity"]
    #[inline(always)]
    pub fn cc3np(&mut self) -> Cc3npW<CcerSpec> {
        Cc3npW::new(self, 11)
    }
    #[doc = "Bit 12 - Compare 4 output enable"]
    #[inline(always)]
    pub fn cc4e(&mut self) -> Cc4eW<CcerSpec> {
        Cc4eW::new(self, 12)
    }
    #[doc = "Bit 13 - Compare 4 output polarity"]
    #[inline(always)]
    pub fn cc4p(&mut self) -> Cc4pW<CcerSpec> {
        Cc4pW::new(self, 13)
    }
    #[doc = "Bit 15 - Compare 4 complementary output polarity"]
    #[inline(always)]
    pub fn cc4np(&mut self) -> Cc4npW<CcerSpec> {
        Cc4npW::new(self, 15)
    }
}
#[doc = "compare enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`ccer::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccer::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CcerSpec;
impl crate::RegisterSpec for CcerSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccer::R`](R) reader structure"]
impl crate::Readable for CcerSpec {}
#[doc = "`write(|w| ..)` method takes [`ccer::W`](W) writer structure"]
impl crate::Writable for CcerSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CCER to value 0"]
impl crate::Resettable for CcerSpec {}
