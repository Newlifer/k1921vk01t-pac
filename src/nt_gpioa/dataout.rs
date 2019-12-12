#[doc = "Reader of register DATAOUT"]
pub type R = crate::R<u32, super::DATAOUT>;
#[doc = "Writer for register DATAOUT"]
pub type W = crate::W<u32, super::DATAOUT>;
#[doc = "Register DATAOUT `reset()`'s with value 0"]
impl crate::ResetValue for super::DATAOUT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DATAOUT`"]
pub type DATAOUT_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `DATAOUT`"]
pub struct DATAOUT_W<'a> {
    w: &'a mut W,
}
impl<'a> DATAOUT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn dataout(&self) -> DATAOUT_R {
        DATAOUT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn dataout(&mut self) -> DATAOUT_W {
        DATAOUT_W { w: self }
    }
}
