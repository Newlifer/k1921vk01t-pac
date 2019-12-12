#[doc = "Reader of register GPIODSCTLA"]
pub type R = crate::R<u16, super::GPIODSCTLA>;
#[doc = "Writer for register GPIODSCTLA"]
pub type W = crate::W<u16, super::GPIODSCTLA>;
#[doc = "Register GPIODSCTLA `reset()`'s with value 0"]
impl crate::ResetValue for super::GPIODSCTLA {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PORTA`"]
pub type PORTA_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `PORTA`"]
pub struct PORTA_W<'a> {
    w: &'a mut W,
}
impl<'a> PORTA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u16) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Port A strength control"]
    #[inline(always)]
    pub fn porta(&self) -> PORTA_R {
        PORTA_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Port A strength control"]
    #[inline(always)]
    pub fn porta(&mut self) -> PORTA_W {
        PORTA_W { w: self }
    }
}
