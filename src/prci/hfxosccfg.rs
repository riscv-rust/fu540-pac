#[doc = "Register `hfxosccfg` reader"]
pub struct R(crate::R<HFXOSCCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HFXOSCCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HFXOSCCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HFXOSCCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `hfxosccfg` writer"]
pub struct W(crate::W<HFXOSCCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HFXOSCCFG_SPEC>;
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
impl From<crate::W<HFXOSCCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HFXOSCCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `xosc_rdy` reader - Crystal input ready"]
pub struct XOSC_RDY_R(crate::FieldReader<bool, bool>);
impl XOSC_RDY_R {
    pub(crate) fn new(bits: bool) -> Self {
        XOSC_RDY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for XOSC_RDY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `xosc_rdy` writer - Crystal input ready"]
pub struct XOSC_RDY_W<'a> {
    w: &'a mut W,
}
impl<'a> XOSC_RDY_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | ((value as u32 & 0x01) << 29);
        self.w
    }
}
#[doc = "Field `xosccfg_en` reader - Crystal input enable"]
pub struct XOSCCFG_EN_R(crate::FieldReader<bool, bool>);
impl XOSCCFG_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        XOSCCFG_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for XOSCCFG_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `xosccfg_en` writer - Crystal input enable"]
pub struct XOSCCFG_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> XOSCCFG_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
        self.w
    }
}
impl R {
    #[doc = "Bit 29 - Crystal input ready"]
    #[inline(always)]
    pub fn xosc_rdy(&self) -> XOSC_RDY_R {
        XOSC_RDY_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Crystal input enable"]
    #[inline(always)]
    pub fn xosccfg_en(&self) -> XOSCCFG_EN_R {
        XOSCCFG_EN_R::new(((self.bits >> 30) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 29 - Crystal input ready"]
    #[inline(always)]
    pub fn xosc_rdy(&mut self) -> XOSC_RDY_W {
        XOSC_RDY_W { w: self }
    }
    #[doc = "Bit 30 - Crystal input enable"]
    #[inline(always)]
    pub fn xosccfg_en(&mut self) -> XOSCCFG_EN_W {
        XOSCCFG_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Crystal Input Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hfxosccfg](index.html) module"]
pub struct HFXOSCCFG_SPEC;
impl crate::RegisterSpec for HFXOSCCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hfxosccfg::R](R) reader structure"]
impl crate::Readable for HFXOSCCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hfxosccfg::W](W) writer structure"]
impl crate::Writable for HFXOSCCFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets hfxosccfg to value 0x4000_0000"]
impl crate::Resettable for HFXOSCCFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x4000_0000
    }
}
