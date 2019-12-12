#[doc = "Reader of register GPIODENH"]
pub type R = crate::R<u16, super::GPIODENH>;
#[doc = "Writer for register GPIODENH"]
pub type W = crate::W<u16, super::GPIODENH>;
#[doc = "Register GPIODENH `reset()`'s with value 0"]
impl crate::ResetValue for super::GPIODENH {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PORTH`"]
pub type PORTH_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `PORTH`"]
pub struct PORTH_W<'a> {
    w: &'a mut W,
}
impl<'a> PORTH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u16) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Port H digital output (GPIO or ALTFUNC) enable"]
    #[inline(always)]
    pub fn porth(&self) -> PORTH_R {
        PORTH_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Port H digital output (GPIO or ALTFUNC) enable"]
    #[inline(always)]
    pub fn porth(&mut self) -> PORTH_W {
        PORTH_W { w: self }
    }
}
