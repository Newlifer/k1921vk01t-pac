#[doc = "Reader of register GPIOPUCTLE"]
pub type R = crate::R<u16, super::GPIOPUCTLE>;
#[doc = "Writer for register GPIOPUCTLE"]
pub type W = crate::W<u16, super::GPIOPUCTLE>;
#[doc = "Register GPIOPUCTLE `reset()`'s with value 0"]
impl crate::ResetValue for super::GPIOPUCTLE {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PORTE`"]
pub type PORTE_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `PORTE`"]
pub struct PORTE_W<'a> {
    w: &'a mut W,
}
impl<'a> PORTE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u16) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Port E pull-up enable"]
    #[inline(always)]
    pub fn porte(&self) -> PORTE_R {
        PORTE_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Port E pull-up enable"]
    #[inline(always)]
    pub fn porte(&mut self) -> PORTE_W {
        PORTE_W { w: self }
    }
}
