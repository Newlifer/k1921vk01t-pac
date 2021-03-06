#[doc = "Reader of register GPIOODCTLG"]
pub type R = crate::R<u16, super::GPIOODCTLG>;
#[doc = "Writer for register GPIOODCTLG"]
pub type W = crate::W<u16, super::GPIOODCTLG>;
#[doc = "Register GPIOODCTLG `reset()`'s with value 0"]
impl crate::ResetValue for super::GPIOODCTLG {
    type Type = u16;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PORTG`"]
pub type PORTG_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `PORTG`"]
pub struct PORTG_W<'a> {
    w: &'a mut W,
}
impl<'a> PORTG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u16) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Port G open-drain enable"]
    #[inline(always)]
    pub fn portg(&self) -> PORTG_R {
        PORTG_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Port G open-drain enable"]
    #[inline(always)]
    pub fn portg(&mut self) -> PORTG_W {
        PORTG_W { w: self }
    }
}
