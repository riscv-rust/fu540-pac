#[doc = "Register `devicesresetreg` reader"]
pub struct R(crate::R<DEVICESRESETREG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DEVICESRESETREG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DEVICESRESETREG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DEVICESRESETREG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `devicesresetreg` writer"]
pub struct W(crate::W<DEVICESRESETREG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DEVICESRESETREG_SPEC>;
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
impl From<crate::W<DEVICESRESETREG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DEVICESRESETREG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DDR_CTRL_RST_N` reader - DDR Controller reset (active low)"]
pub struct DDR_CTRL_RST_N_R(crate::FieldReader<bool, bool>);
impl DDR_CTRL_RST_N_R {
    pub(crate) fn new(bits: bool) -> Self {
        DDR_CTRL_RST_N_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DDR_CTRL_RST_N_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DDR_CTRL_RST_N` writer - DDR Controller reset (active low)"]
pub struct DDR_CTRL_RST_N_W<'a> {
    w: &'a mut W,
}
impl<'a> DDR_CTRL_RST_N_W<'a> {
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
#[doc = "Field `DDR_AXI_RST_N` reader - DDR Controller AXI interface reset (active low)"]
pub struct DDR_AXI_RST_N_R(crate::FieldReader<bool, bool>);
impl DDR_AXI_RST_N_R {
    pub(crate) fn new(bits: bool) -> Self {
        DDR_AXI_RST_N_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DDR_AXI_RST_N_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DDR_AXI_RST_N` writer - DDR Controller AXI interface reset (active low)"]
pub struct DDR_AXI_RST_N_W<'a> {
    w: &'a mut W,
}
impl<'a> DDR_AXI_RST_N_W<'a> {
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
#[doc = "Field `DDR_AHB_RST_N` reader - DDR Controller AHB interface reset (active low)"]
pub struct DDR_AHB_RST_N_R(crate::FieldReader<bool, bool>);
impl DDR_AHB_RST_N_R {
    pub(crate) fn new(bits: bool) -> Self {
        DDR_AHB_RST_N_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DDR_AHB_RST_N_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DDR_AHB_RST_N` writer - DDR Controller AHB interface reset (active low)"]
pub struct DDR_AHB_RST_N_W<'a> {
    w: &'a mut W,
}
impl<'a> DDR_AHB_RST_N_W<'a> {
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
#[doc = "Field `DDR_PHY_RST_N` reader - DDR PHY reset (active low)"]
pub struct DDR_PHY_RST_N_R(crate::FieldReader<bool, bool>);
impl DDR_PHY_RST_N_R {
    pub(crate) fn new(bits: bool) -> Self {
        DDR_PHY_RST_N_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DDR_PHY_RST_N_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DDR_PHY_RST_N` writer - DDR PHY reset (active low)"]
pub struct DDR_PHY_RST_N_W<'a> {
    w: &'a mut W,
}
impl<'a> DDR_PHY_RST_N_W<'a> {
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
#[doc = "Field `GEMGXL_RST_N` reader - Gigabit Ethernet Subsystem reset (active low)"]
pub struct GEMGXL_RST_N_R(crate::FieldReader<bool, bool>);
impl GEMGXL_RST_N_R {
    pub(crate) fn new(bits: bool) -> Self {
        GEMGXL_RST_N_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GEMGXL_RST_N_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GEMGXL_RST_N` writer - Gigabit Ethernet Subsystem reset (active low)"]
pub struct GEMGXL_RST_N_W<'a> {
    w: &'a mut W,
}
impl<'a> GEMGXL_RST_N_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - DDR Controller reset (active low)"]
    #[inline(always)]
    pub fn ddr_ctrl_rst_n(&self) -> DDR_CTRL_RST_N_R {
        DDR_CTRL_RST_N_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - DDR Controller AXI interface reset (active low)"]
    #[inline(always)]
    pub fn ddr_axi_rst_n(&self) -> DDR_AXI_RST_N_R {
        DDR_AXI_RST_N_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - DDR Controller AHB interface reset (active low)"]
    #[inline(always)]
    pub fn ddr_ahb_rst_n(&self) -> DDR_AHB_RST_N_R {
        DDR_AHB_RST_N_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - DDR PHY reset (active low)"]
    #[inline(always)]
    pub fn ddr_phy_rst_n(&self) -> DDR_PHY_RST_N_R {
        DDR_PHY_RST_N_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Gigabit Ethernet Subsystem reset (active low)"]
    #[inline(always)]
    pub fn gemgxl_rst_n(&self) -> GEMGXL_RST_N_R {
        GEMGXL_RST_N_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DDR Controller reset (active low)"]
    #[inline(always)]
    pub fn ddr_ctrl_rst_n(&mut self) -> DDR_CTRL_RST_N_W {
        DDR_CTRL_RST_N_W { w: self }
    }
    #[doc = "Bit 1 - DDR Controller AXI interface reset (active low)"]
    #[inline(always)]
    pub fn ddr_axi_rst_n(&mut self) -> DDR_AXI_RST_N_W {
        DDR_AXI_RST_N_W { w: self }
    }
    #[doc = "Bit 2 - DDR Controller AHB interface reset (active low)"]
    #[inline(always)]
    pub fn ddr_ahb_rst_n(&mut self) -> DDR_AHB_RST_N_W {
        DDR_AHB_RST_N_W { w: self }
    }
    #[doc = "Bit 3 - DDR PHY reset (active low)"]
    #[inline(always)]
    pub fn ddr_phy_rst_n(&mut self) -> DDR_PHY_RST_N_W {
        DDR_PHY_RST_N_W { w: self }
    }
    #[doc = "Bit 5 - Gigabit Ethernet Subsystem reset (active low)"]
    #[inline(always)]
    pub fn gemgxl_rst_n(&mut self) -> GEMGXL_RST_N_W {
        GEMGXL_RST_N_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Peripheral Devices Reset Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [devicesresetreg](index.html) module"]
pub struct DEVICESRESETREG_SPEC;
impl crate::RegisterSpec for DEVICESRESETREG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [devicesresetreg::R](R) reader structure"]
impl crate::Readable for DEVICESRESETREG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [devicesresetreg::W](W) writer structure"]
impl crate::Writable for DEVICESRESETREG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets devicesresetreg to value 0"]
impl crate::Resettable for DEVICESRESETREG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
