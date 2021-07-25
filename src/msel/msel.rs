#[doc = "Register `MSEL` reader"]
pub struct R(crate::R<MSEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MSEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MSEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MSEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `MSEL` reader - "]
pub struct MSEL_R(crate::FieldReader<u8, u8>);
impl MSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        MSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MSEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn msel(&self) -> MSEL_R {
        MSEL_R::new((self.bits & 0x0f) as u8)
    }
}
#[doc = "The MSEL pin state\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [msel](index.html) module"]
pub struct MSEL_SPEC;
impl crate::RegisterSpec for MSEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [msel::R](R) reader structure"]
impl crate::Readable for MSEL_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MSEL to value 0"]
impl crate::Resettable for MSEL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
