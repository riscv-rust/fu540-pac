#[doc = "Register `corepllcfg0` reader"]
pub struct R(crate::R<COREPLLCFG0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<COREPLLCFG0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<COREPLLCFG0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<COREPLLCFG0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `corepllcfg0` writer"]
pub struct W(crate::W<COREPLLCFG0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<COREPLLCFG0_SPEC>;
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
impl From<crate::W<COREPLLCFG0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<COREPLLCFG0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `divr` reader - PLL reference divider value minus one"]
pub struct DIVR_R(crate::FieldReader<u8, u8>);
impl DIVR_R {
    pub(crate) fn new(bits: u8) -> Self {
        DIVR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIVR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `divr` writer - PLL reference divider value minus one"]
pub struct DIVR_W<'a> {
    w: &'a mut W,
}
impl<'a> DIVR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | (value as u32 & 0x3f);
        self.w
    }
}
#[doc = "Field `divf` reader - PLL feedback divider value minus one"]
pub struct DIVF_R(crate::FieldReader<u16, u16>);
impl DIVF_R {
    pub(crate) fn new(bits: u16) -> Self {
        DIVF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIVF_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `divf` writer - PLL feedback divider value minus one"]
pub struct DIVF_W<'a> {
    w: &'a mut W,
}
impl<'a> DIVF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01ff << 6)) | ((value as u32 & 0x01ff) << 6);
        self.w
    }
}
#[doc = "Field `divq` reader - Log2 of PLL output divider. Valid settings are 1, 2, 3, 4, 5, 6"]
pub struct DIVQ_R(crate::FieldReader<u8, u8>);
impl DIVQ_R {
    pub(crate) fn new(bits: u8) -> Self {
        DIVQ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIVQ_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `divq` writer - Log2 of PLL output divider. Valid settings are 1, 2, 3, 4, 5, 6"]
pub struct DIVQ_W<'a> {
    w: &'a mut W,
}
impl<'a> DIVQ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 15)) | ((value as u32 & 0x07) << 15);
        self.w
    }
}
#[doc = "Field `range` reader - PLL filter range. 3'b100 = 33MHz"]
pub struct RANGE_R(crate::FieldReader<u8, u8>);
impl RANGE_R {
    pub(crate) fn new(bits: u8) -> Self {
        RANGE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RANGE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `range` writer - PLL filter range. 3'b100 = 33MHz"]
pub struct RANGE_W<'a> {
    w: &'a mut W,
}
impl<'a> RANGE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 18)) | ((value as u32 & 0x07) << 18);
        self.w
    }
}
#[doc = "Field `bypass` reader - PLL bypass"]
pub struct BYPASS_R(crate::FieldReader<bool, bool>);
impl BYPASS_R {
    pub(crate) fn new(bits: bool) -> Self {
        BYPASS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BYPASS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `bypass` writer - PLL bypass"]
pub struct BYPASS_W<'a> {
    w: &'a mut W,
}
impl<'a> BYPASS_W<'a> {
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
#[doc = "Field `fse` reader - Internal or external input path. Valid setting is 1, internal feedback."]
pub struct FSE_R(crate::FieldReader<bool, bool>);
impl FSE_R {
    pub(crate) fn new(bits: bool) -> Self {
        FSE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FSE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `fse` writer - Internal or external input path. Valid setting is 1, internal feedback."]
pub struct FSE_W<'a> {
    w: &'a mut W,
}
impl<'a> FSE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | ((value as u32 & 0x01) << 25);
        self.w
    }
}
#[doc = "Field `lock` reader - PLL locked"]
pub struct LOCK_R(crate::FieldReader<bool, bool>);
impl LOCK_R {
    pub(crate) fn new(bits: bool) -> Self {
        LOCK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOCK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `lock` writer - PLL locked"]
pub struct LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - PLL reference divider value minus one"]
    #[inline(always)]
    pub fn divr(&self) -> DIVR_R {
        DIVR_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:14 - PLL feedback divider value minus one"]
    #[inline(always)]
    pub fn divf(&self) -> DIVF_R {
        DIVF_R::new(((self.bits >> 6) & 0x01ff) as u16)
    }
    #[doc = "Bits 15:17 - Log2 of PLL output divider. Valid settings are 1, 2, 3, 4, 5, 6"]
    #[inline(always)]
    pub fn divq(&self) -> DIVQ_R {
        DIVQ_R::new(((self.bits >> 15) & 0x07) as u8)
    }
    #[doc = "Bits 18:20 - PLL filter range. 3'b100 = 33MHz"]
    #[inline(always)]
    pub fn range(&self) -> RANGE_R {
        RANGE_R::new(((self.bits >> 18) & 0x07) as u8)
    }
    #[doc = "Bit 24 - PLL bypass"]
    #[inline(always)]
    pub fn bypass(&self) -> BYPASS_R {
        BYPASS_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Internal or external input path. Valid setting is 1, internal feedback."]
    #[inline(always)]
    pub fn fse(&self) -> FSE_R {
        FSE_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 31 - PLL locked"]
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - PLL reference divider value minus one"]
    #[inline(always)]
    pub fn divr(&mut self) -> DIVR_W {
        DIVR_W { w: self }
    }
    #[doc = "Bits 6:14 - PLL feedback divider value minus one"]
    #[inline(always)]
    pub fn divf(&mut self) -> DIVF_W {
        DIVF_W { w: self }
    }
    #[doc = "Bits 15:17 - Log2 of PLL output divider. Valid settings are 1, 2, 3, 4, 5, 6"]
    #[inline(always)]
    pub fn divq(&mut self) -> DIVQ_W {
        DIVQ_W { w: self }
    }
    #[doc = "Bits 18:20 - PLL filter range. 3'b100 = 33MHz"]
    #[inline(always)]
    pub fn range(&mut self) -> RANGE_W {
        RANGE_W { w: self }
    }
    #[doc = "Bit 24 - PLL bypass"]
    #[inline(always)]
    pub fn bypass(&mut self) -> BYPASS_W {
        BYPASS_W { w: self }
    }
    #[doc = "Bit 25 - Internal or external input path. Valid setting is 1, internal feedback."]
    #[inline(always)]
    pub fn fse(&mut self) -> FSE_W {
        FSE_W { w: self }
    }
    #[doc = "Bit 31 - PLL locked"]
    #[inline(always)]
    pub fn lock(&mut self) -> LOCK_W {
        LOCK_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Core PLL Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [corepllcfg0](index.html) module"]
pub struct COREPLLCFG0_SPEC;
impl crate::RegisterSpec for COREPLLCFG0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [corepllcfg0::R](R) reader structure"]
impl crate::Readable for COREPLLCFG0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [corepllcfg0::W](W) writer structure"]
impl crate::Writable for COREPLLCFG0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets corepllcfg0 to value 0x0201_87c1"]
impl crate::Resettable for COREPLLCFG0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0201_87c1
    }
}
