#[doc = "Register `procmoncfg` reader"]
pub struct R(crate::R<PROCMONCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PROCMONCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PROCMONCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PROCMONCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `procmoncfg` writer"]
pub struct W(crate::W<PROCMONCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PROCMONCFG_SPEC>;
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
impl From<crate::W<PROCMONCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PROCMONCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `coreclksel` reader - Select CORECLK"]
pub struct CORECLKSEL_R(crate::FieldReader<bool, bool>);
impl CORECLKSEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        CORECLKSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORECLKSEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `coreclksel` writer - Select CORECLK"]
pub struct CORECLKSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CORECLKSEL_W<'a> {
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
    #[doc = "Bit 24 - Select CORECLK"]
    #[inline(always)]
    pub fn coreclksel(&self) -> CORECLKSEL_R {
        CORECLKSEL_R::new(((self.bits >> 24) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 24 - Select CORECLK"]
    #[inline(always)]
    pub fn coreclksel(&mut self) -> CORECLKSEL_W {
        CORECLKSEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PROCMON Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [procmoncfg](index.html) module"]
pub struct PROCMONCFG_SPEC;
impl crate::RegisterSpec for PROCMONCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [procmoncfg::R](R) reader structure"]
impl crate::Readable for PROCMONCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [procmoncfg::W](W) writer structure"]
impl crate::Writable for PROCMONCFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets procmoncfg to value 0"]
impl crate::Resettable for PROCMONCFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
