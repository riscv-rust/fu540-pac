#[doc = "Register `ddrpllcfg1` reader"]
pub struct R(crate::R<DDRPLLCFG1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DDRPLLCFG1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DDRPLLCFG1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DDRPLLCFG1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ddrpllcfg1` writer"]
pub struct W(crate::W<DDRPLLCFG1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DDRPLLCFG1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<DDRPLLCFG1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DDRPLLCFG1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `cke` reader - PLL clock output enable. Glitch free clock gate after PLL output. 1 enables clock, 0 disables clock"]
pub struct CKE_R(crate::FieldReader<bool, bool>);
impl CKE_R {
    pub(crate) fn new(bits: bool) -> Self {
        CKE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CKE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cke` writer - PLL clock output enable. Glitch free clock gate after PLL output. 1 enables clock, 0 disables clock"]
pub struct CKE_W<'a> {
    w: &'a mut W,
}
impl<'a> CKE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bit 24 - PLL clock output enable. Glitch free clock gate after PLL output. 1 enables clock, 0 disables clock"]
    #[inline(always)]
    pub fn cke(&self) -> CKE_R {
        CKE_R::new(((self.bits >> 24) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 24 - PLL clock output enable. Glitch free clock gate after PLL output. 1 enables clock, 0 disables clock"]
    #[inline(always)]
    pub fn cke(&mut self) -> CKE_W {
        CKE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DDR PLL Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrpllcfg1](index.html) module"]
pub struct DDRPLLCFG1_SPEC;
impl crate::RegisterSpec for DDRPLLCFG1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ddrpllcfg1::R](R) reader structure"]
impl crate::Readable for DDRPLLCFG1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ddrpllcfg1::W](W) writer structure"]
impl crate::Writable for DDRPLLCFG1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ddrpllcfg1 to value 0"]
impl crate::Resettable for DDRPLLCFG1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
