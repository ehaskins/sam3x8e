#[doc = "Register `ADDR` reader"]
pub struct R(crate::R<ADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADDR` writer"]
pub struct W(crate::W<ADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADDR_SPEC>;
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
impl From<crate::W<ADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADDR_CYCLE0` reader - NAND Flash Array Address cycle 0"]
pub struct ADDR_CYCLE0_R(crate::FieldReader<u8, u8>);
impl ADDR_CYCLE0_R {
    pub(crate) fn new(bits: u8) -> Self {
        ADDR_CYCLE0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADDR_CYCLE0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADDR_CYCLE0` writer - NAND Flash Array Address cycle 0"]
pub struct ADDR_CYCLE0_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDR_CYCLE0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - NAND Flash Array Address cycle 0"]
    #[inline(always)]
    pub fn addr_cycle0(&self) -> ADDR_CYCLE0_R {
        ADDR_CYCLE0_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - NAND Flash Array Address cycle 0"]
    #[inline(always)]
    pub fn addr_cycle0(&mut self) -> ADDR_CYCLE0_W {
        ADDR_CYCLE0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SMC NFC Address Cycle Zero Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [addr](index.html) module"]
pub struct ADDR_SPEC;
impl crate::RegisterSpec for ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [addr::R](R) reader structure"]
impl crate::Readable for ADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [addr::W](W) writer structure"]
impl crate::Writable for ADDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADDR to value 0"]
impl crate::Resettable for ADDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
