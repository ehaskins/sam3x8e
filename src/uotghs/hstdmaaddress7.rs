#[doc = "Register `HSTDMAADDRESS7` reader"]
pub struct R(crate::R<HSTDMAADDRESS7_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HSTDMAADDRESS7_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HSTDMAADDRESS7_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HSTDMAADDRESS7_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HSTDMAADDRESS7` writer"]
pub struct W(crate::W<HSTDMAADDRESS7_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HSTDMAADDRESS7_SPEC>;
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
impl From<crate::W<HSTDMAADDRESS7_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HSTDMAADDRESS7_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BUFF_ADD` reader - Buffer Address"]
pub struct BUFF_ADD_R(crate::FieldReader<u32, u32>);
impl BUFF_ADD_R {
    pub(crate) fn new(bits: u32) -> Self {
        BUFF_ADD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BUFF_ADD_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BUFF_ADD` writer - Buffer Address"]
pub struct BUFF_ADD_W<'a> {
    w: &'a mut W,
}
impl<'a> BUFF_ADD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Buffer Address"]
    #[inline(always)]
    pub fn buff_add(&self) -> BUFF_ADD_R {
        BUFF_ADD_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Buffer Address"]
    #[inline(always)]
    pub fn buff_add(&mut self) -> BUFF_ADD_W {
        BUFF_ADD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Host DMA Channel Address Register (n = 7)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hstdmaaddress7](index.html) module"]
pub struct HSTDMAADDRESS7_SPEC;
impl crate::RegisterSpec for HSTDMAADDRESS7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hstdmaaddress7::R](R) reader structure"]
impl crate::Readable for HSTDMAADDRESS7_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hstdmaaddress7::W](W) writer structure"]
impl crate::Writable for HSTDMAADDRESS7_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HSTDMAADDRESS7 to value 0"]
impl crate::Resettable for HSTDMAADDRESS7_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
