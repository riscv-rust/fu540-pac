#[doc = "Register `coreclksel` reader"]
pub struct R(crate::R<CORECLKSEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CORECLKSEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CORECLKSEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CORECLKSEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `coreclksel` writer"]
pub struct W(crate::W<CORECLKSEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CORECLKSEL_SPEC>;
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
impl From<crate::W<CORECLKSEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CORECLKSEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `coreclksel` reader - CORECLK select. 0 = CORE_PLL output 1 = HFCLK"]
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
#[doc = "Field `coreclksel` writer - CORECLK select. 0 = CORE_PLL output 1 = HFCLK"]
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - CORECLK select. 0 = CORE_PLL output 1 = HFCLK"]
    #[inline(always)]
    pub fn coreclksel(&self) -> CORECLKSEL_R {
        CORECLKSEL_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CORECLK select. 0 = CORE_PLL output 1 = HFCLK"]
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
#[doc = "CORECLK Source Selection Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [coreclksel](index.html) module"]
pub struct CORECLKSEL_SPEC;
impl crate::RegisterSpec for CORECLKSEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [coreclksel::R](R) reader structure"]
impl crate::Readable for CORECLKSEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [coreclksel::W](W) writer structure"]
impl crate::Writable for CORECLKSEL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets coreclksel to value 0x01"]
impl crate::Resettable for CORECLKSEL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
