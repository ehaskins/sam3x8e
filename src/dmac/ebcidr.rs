#[doc = "Register `EBCIDR` writer"]
pub struct W(crate::W<EBCIDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EBCIDR_SPEC>;
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
impl From<crate::W<EBCIDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EBCIDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BTC0` writer - Buffer Transfer Completed \\[5:0\\]"]
pub struct BTC0_W<'a> {
    w: &'a mut W,
}
impl<'a> BTC0_W<'a> {
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
#[doc = "Field `BTC1` writer - Buffer Transfer Completed \\[5:0\\]"]
pub struct BTC1_W<'a> {
    w: &'a mut W,
}
impl<'a> BTC1_W<'a> {
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
#[doc = "Field `BTC2` writer - Buffer Transfer Completed \\[5:0\\]"]
pub struct BTC2_W<'a> {
    w: &'a mut W,
}
impl<'a> BTC2_W<'a> {
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
#[doc = "Field `BTC3` writer - Buffer Transfer Completed \\[5:0\\]"]
pub struct BTC3_W<'a> {
    w: &'a mut W,
}
impl<'a> BTC3_W<'a> {
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
#[doc = "Field `BTC4` writer - Buffer Transfer Completed \\[5:0\\]"]
pub struct BTC4_W<'a> {
    w: &'a mut W,
}
impl<'a> BTC4_W<'a> {
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
#[doc = "Field `BTC5` writer - Buffer Transfer Completed \\[5:0\\]"]
pub struct BTC5_W<'a> {
    w: &'a mut W,
}
impl<'a> BTC5_W<'a> {
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
#[doc = "Field `CBTC0` writer - Chained Buffer Transfer Completed \\[5:0\\]"]
pub struct CBTC0_W<'a> {
    w: &'a mut W,
}
impl<'a> CBTC0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `CBTC1` writer - Chained Buffer Transfer Completed \\[5:0\\]"]
pub struct CBTC1_W<'a> {
    w: &'a mut W,
}
impl<'a> CBTC1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Field `CBTC2` writer - Chained Buffer Transfer Completed \\[5:0\\]"]
pub struct CBTC2_W<'a> {
    w: &'a mut W,
}
impl<'a> CBTC2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Field `CBTC3` writer - Chained Buffer Transfer Completed \\[5:0\\]"]
pub struct CBTC3_W<'a> {
    w: &'a mut W,
}
impl<'a> CBTC3_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Field `CBTC4` writer - Chained Buffer Transfer Completed \\[5:0\\]"]
pub struct CBTC4_W<'a> {
    w: &'a mut W,
}
impl<'a> CBTC4_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Field `CBTC5` writer - Chained Buffer Transfer Completed \\[5:0\\]"]
pub struct CBTC5_W<'a> {
    w: &'a mut W,
}
impl<'a> CBTC5_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "Field `ERR0` writer - Access Error \\[5:0\\]"]
pub struct ERR0_W<'a> {
    w: &'a mut W,
}
impl<'a> ERR0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Field `ERR1` writer - Access Error \\[5:0\\]"]
pub struct ERR1_W<'a> {
    w: &'a mut W,
}
impl<'a> ERR1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "Field `ERR2` writer - Access Error \\[5:0\\]"]
pub struct ERR2_W<'a> {
    w: &'a mut W,
}
impl<'a> ERR2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
#[doc = "Field `ERR3` writer - Access Error \\[5:0\\]"]
pub struct ERR3_W<'a> {
    w: &'a mut W,
}
impl<'a> ERR3_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
#[doc = "Field `ERR4` writer - Access Error \\[5:0\\]"]
pub struct ERR4_W<'a> {
    w: &'a mut W,
}
impl<'a> ERR4_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
#[doc = "Field `ERR5` writer - Access Error \\[5:0\\]"]
pub struct ERR5_W<'a> {
    w: &'a mut W,
}
impl<'a> ERR5_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | ((value as u32 & 0x01) << 21);
        self.w
    }
}
impl W {
    #[doc = "Bit 0 - Buffer Transfer Completed \\[5:0\\]"]
    #[inline(always)]
    pub fn btc0(&mut self) -> BTC0_W {
        BTC0_W { w: self }
    }
    #[doc = "Bit 1 - Buffer Transfer Completed \\[5:0\\]"]
    #[inline(always)]
    pub fn btc1(&mut self) -> BTC1_W {
        BTC1_W { w: self }
    }
    #[doc = "Bit 2 - Buffer Transfer Completed \\[5:0\\]"]
    #[inline(always)]
    pub fn btc2(&mut self) -> BTC2_W {
        BTC2_W { w: self }
    }
    #[doc = "Bit 3 - Buffer Transfer Completed \\[5:0\\]"]
    #[inline(always)]
    pub fn btc3(&mut self) -> BTC3_W {
        BTC3_W { w: self }
    }
    #[doc = "Bit 4 - Buffer Transfer Completed \\[5:0\\]"]
    #[inline(always)]
    pub fn btc4(&mut self) -> BTC4_W {
        BTC4_W { w: self }
    }
    #[doc = "Bit 5 - Buffer Transfer Completed \\[5:0\\]"]
    #[inline(always)]
    pub fn btc5(&mut self) -> BTC5_W {
        BTC5_W { w: self }
    }
    #[doc = "Bit 8 - Chained Buffer Transfer Completed \\[5:0\\]"]
    #[inline(always)]
    pub fn cbtc0(&mut self) -> CBTC0_W {
        CBTC0_W { w: self }
    }
    #[doc = "Bit 9 - Chained Buffer Transfer Completed \\[5:0\\]"]
    #[inline(always)]
    pub fn cbtc1(&mut self) -> CBTC1_W {
        CBTC1_W { w: self }
    }
    #[doc = "Bit 10 - Chained Buffer Transfer Completed \\[5:0\\]"]
    #[inline(always)]
    pub fn cbtc2(&mut self) -> CBTC2_W {
        CBTC2_W { w: self }
    }
    #[doc = "Bit 11 - Chained Buffer Transfer Completed \\[5:0\\]"]
    #[inline(always)]
    pub fn cbtc3(&mut self) -> CBTC3_W {
        CBTC3_W { w: self }
    }
    #[doc = "Bit 12 - Chained Buffer Transfer Completed \\[5:0\\]"]
    #[inline(always)]
    pub fn cbtc4(&mut self) -> CBTC4_W {
        CBTC4_W { w: self }
    }
    #[doc = "Bit 13 - Chained Buffer Transfer Completed \\[5:0\\]"]
    #[inline(always)]
    pub fn cbtc5(&mut self) -> CBTC5_W {
        CBTC5_W { w: self }
    }
    #[doc = "Bit 16 - Access Error \\[5:0\\]"]
    #[inline(always)]
    pub fn err0(&mut self) -> ERR0_W {
        ERR0_W { w: self }
    }
    #[doc = "Bit 17 - Access Error \\[5:0\\]"]
    #[inline(always)]
    pub fn err1(&mut self) -> ERR1_W {
        ERR1_W { w: self }
    }
    #[doc = "Bit 18 - Access Error \\[5:0\\]"]
    #[inline(always)]
    pub fn err2(&mut self) -> ERR2_W {
        ERR2_W { w: self }
    }
    #[doc = "Bit 19 - Access Error \\[5:0\\]"]
    #[inline(always)]
    pub fn err3(&mut self) -> ERR3_W {
        ERR3_W { w: self }
    }
    #[doc = "Bit 20 - Access Error \\[5:0\\]"]
    #[inline(always)]
    pub fn err4(&mut self) -> ERR4_W {
        ERR4_W { w: self }
    }
    #[doc = "Bit 21 - Access Error \\[5:0\\]"]
    #[inline(always)]
    pub fn err5(&mut self) -> ERR5_W {
        ERR5_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMAC Error, Chained Buffer Transfer Completed Interrupt and Buffer Transfer Completed Interrupt Disable register.\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ebcidr](index.html) module"]
pub struct EBCIDR_SPEC;
impl crate::RegisterSpec for EBCIDR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [ebcidr::W](W) writer structure"]
impl crate::Writable for EBCIDR_SPEC {
    type Writer = W;
}
