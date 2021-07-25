#[doc = "Register `clkmuxstatus` reader"]
pub struct R(crate::R<CLKMUXSTATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLKMUXSTATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLKMUXSTATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLKMUXSTATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `clkmuxstatus` writer"]
pub struct W(crate::W<CLKMUXSTATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLKMUXSTATUS_SPEC>;
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
impl From<crate::W<CLKMUXSTATUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLKMUXSTATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `coreclkpllsel` reader - "]
pub struct CORECLKPLLSEL_R(crate::FieldReader<bool, bool>);
impl CORECLKPLLSEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        CORECLKPLLSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORECLKPLLSEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `coreclkpllsel` writer - "]
pub struct CORECLKPLLSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CORECLKPLLSEL_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Field `tlclksel` reader - "]
pub struct TLCLKSEL_R(crate::FieldReader<bool, bool>);
impl TLCLKSEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        TLCLKSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TLCLKSEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tlclksel` writer - "]
pub struct TLCLKSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> TLCLKSEL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `rtcxsel` reader - "]
pub struct RTCXSEL_R(crate::FieldReader<bool, bool>);
impl RTCXSEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        RTCXSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTCXSEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rtcxsel` writer - "]
pub struct RTCXSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCXSEL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `ddrctrlclksel` reader - "]
pub struct DDRCTRLCLKSEL_R(crate::FieldReader<bool, bool>);
impl DDRCTRLCLKSEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        DDRCTRLCLKSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DDRCTRLCLKSEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ddrctrlclksel` writer - "]
pub struct DDRCTRLCLKSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> DDRCTRLCLKSEL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `ddrphyclksel` reader - "]
pub struct DDRPHYCLKSEL_R(crate::FieldReader<bool, bool>);
impl DDRPHYCLKSEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        DDRPHYCLKSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DDRPHYCLKSEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ddrphyclksel` writer - "]
pub struct DDRPHYCLKSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> DDRPHYCLKSEL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `gemgxlclksel` reader - "]
pub struct GEMGXLCLKSEL_R(crate::FieldReader<bool, bool>);
impl GEMGXLCLKSEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        GEMGXLCLKSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GEMGXLCLKSEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `gemgxlclksel` writer - "]
pub struct GEMGXLCLKSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> GEMGXLCLKSEL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn coreclkpllsel(&self) -> CORECLKPLLSEL_R {
        CORECLKPLLSEL_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn tlclksel(&self) -> TLCLKSEL_R {
        TLCLKSEL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn rtcxsel(&self) -> RTCXSEL_R {
        RTCXSEL_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn ddrctrlclksel(&self) -> DDRCTRLCLKSEL_R {
        DDRCTRLCLKSEL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn ddrphyclksel(&self) -> DDRPHYCLKSEL_R {
        DDRPHYCLKSEL_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn gemgxlclksel(&self) -> GEMGXLCLKSEL_R {
        GEMGXLCLKSEL_R::new(((self.bits >> 6) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn coreclkpllsel(&mut self) -> CORECLKPLLSEL_W {
        CORECLKPLLSEL_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn tlclksel(&mut self) -> TLCLKSEL_W {
        TLCLKSEL_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn rtcxsel(&mut self) -> RTCXSEL_W {
        RTCXSEL_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn ddrctrlclksel(&mut self) -> DDRCTRLCLKSEL_W {
        DDRCTRLCLKSEL_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn ddrphyclksel(&mut self) -> DDRPHYCLKSEL_W {
        DDRPHYCLKSEL_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn gemgxlclksel(&mut self) -> GEMGXLCLKSEL_W {
        GEMGXLCLKSEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CLKMUX Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clkmuxstatus](index.html) module"]
pub struct CLKMUXSTATUS_SPEC;
impl crate::RegisterSpec for CLKMUXSTATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clkmuxstatus::R](R) reader structure"]
impl crate::Readable for CLKMUXSTATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clkmuxstatus::W](W) writer structure"]
impl crate::Writable for CLKMUXSTATUS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets clkmuxstatus to value 0"]
impl crate::Resettable for CLKMUXSTATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
