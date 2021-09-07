#[doc = "Register `STE` reader"]
pub struct R(crate::R<STE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STE` writer"]
pub struct W(crate::W<STE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STE_SPEC>;
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
impl From<crate::W<STE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SQER` reader - SQE test errors"]
pub struct SQER_R(crate::FieldReader<u8, u8>);
impl SQER_R {
    pub(crate) fn new(bits: u8) -> Self {
        SQER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SQER_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SQER` writer - SQE test errors"]
pub struct SQER_W<'a> {
    w: &'a mut W,
}
impl<'a> SQER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - SQE test errors"]
    #[inline(always)]
    pub fn sqer(&self) -> SQER_R {
        SQER_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - SQE test errors"]
    #[inline(always)]
    pub fn sqer(&mut self) -> SQER_W {
        SQER_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SQE Test Errors Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ste](index.html) module"]
pub struct STE_SPEC;
impl crate::RegisterSpec for STE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ste::R](R) reader structure"]
impl crate::Readable for STE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ste::W](W) writer structure"]
impl crate::Writable for STE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets STE to value 0"]
impl crate::Resettable for STE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}