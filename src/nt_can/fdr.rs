#[doc = "Reader of register FDR"]
pub type R = crate::R<u32, super::FDR>;
#[doc = "Writer for register FDR"]
pub type W = crate::W<u32, super::FDR>;
#[doc = "Register FDR `reset()`'s with value 0"]
impl crate::ResetValue for super::FDR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `STEP`"]
pub type STEP_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `STEP`"]
pub struct STEP_W<'a> {
    w: &'a mut W,
}
impl<'a> STEP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | ((value as u32) & 0x03ff);
        self.w
    }
}
#[doc = "Reader of field `SM`"]
pub type SM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SM`"]
pub struct SM_W<'a> {
    w: &'a mut W,
}
impl<'a> SM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `SC`"]
pub type SC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SC`"]
pub struct SC_W<'a> {
    w: &'a mut W,
}
impl<'a> SC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DM_A {
    #[doc = "0: counter disabled"]
    DISABLE,
    #[doc = "1: normal operation mode"]
    NORMALMODE,
    #[doc = "2: divider operation mode"]
    DIVIDERMODE,
}
impl From<DM_A> for u8 {
    #[inline(always)]
    fn from(variant: DM_A) -> Self {
        match variant {
            DM_A::DISABLE => 0,
            DM_A::NORMALMODE => 1,
            DM_A::DIVIDERMODE => 2,
        }
    }
}
#[doc = "Reader of field `DM`"]
pub type DM_R = crate::R<u8, DM_A>;
impl DM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, DM_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(DM_A::DISABLE),
            1 => Val(DM_A::NORMALMODE),
            2 => Val(DM_A::DIVIDERMODE),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == DM_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `NORMALMODE`"]
    #[inline(always)]
    pub fn is_normal_mode(&self) -> bool {
        *self == DM_A::NORMALMODE
    }
    #[doc = "Checks if the value of the field is `DIVIDERMODE`"]
    #[inline(always)]
    pub fn is_divider_mode(&self) -> bool {
        *self == DM_A::DIVIDERMODE
    }
}
#[doc = "Write proxy for field `DM`"]
pub struct DM_W<'a> {
    w: &'a mut W,
}
impl<'a> DM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DM_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "counter disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(DM_A::DISABLE)
    }
    #[doc = "normal operation mode"]
    #[inline(always)]
    pub fn normal_mode(self) -> &'a mut W {
        self.variant(DM_A::NORMALMODE)
    }
    #[doc = "divider operation mode"]
    #[inline(always)]
    pub fn divider_mode(self) -> &'a mut W {
        self.variant(DM_A::DIVIDERMODE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
        self.w
    }
}
#[doc = "Reader of field `RESULT`"]
pub type RESULT_R = crate::R<u16, u16>;
#[doc = "Reader of field `SUSACK`"]
pub type SUSACK_R = crate::R<bool, bool>;
#[doc = "Reader of field `SUSREQ`"]
pub type SUSREQ_R = crate::R<bool, bool>;
#[doc = "Reader of field `ENHW`"]
pub type ENHW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENHW`"]
pub struct ENHW_W<'a> {
    w: &'a mut W,
}
impl<'a> ENHW_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Reader of field `DISCLK`"]
pub type DISCLK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DISCLK`"]
pub struct DISCLK_W<'a> {
    w: &'a mut W,
}
impl<'a> DISCLK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn step(&self) -> STEP_R {
        STEP_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 11 - Select the transition mode Suspend"]
    #[inline(always)]
    pub fn sm(&self) -> SM_R {
        SM_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn sc(&self) -> SC_R {
        SC_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn dm(&self) -> DM_R {
        DM_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 16:25"]
    #[inline(always)]
    pub fn result(&self) -> RESULT_R {
        RESULT_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
    #[doc = "Bit 28 - Indicate mode Suspend"]
    #[inline(always)]
    pub fn susack(&self) -> SUSACK_R {
        SUSACK_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Activate Suspend mode"]
    #[inline(always)]
    pub fn susreq(&self) -> SUSREQ_R {
        SUSREQ_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Control bit synchronization"]
    #[inline(always)]
    pub fn enhw(&self) -> ENHW_R {
        ENHW_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Disable bit internal clock"]
    #[inline(always)]
    pub fn disclk(&self) -> DISCLK_R {
        DISCLK_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn step(&mut self) -> STEP_W {
        STEP_W { w: self }
    }
    #[doc = "Bit 11 - Select the transition mode Suspend"]
    #[inline(always)]
    pub fn sm(&mut self) -> SM_W {
        SM_W { w: self }
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn sc(&mut self) -> SC_W {
        SC_W { w: self }
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn dm(&mut self) -> DM_W {
        DM_W { w: self }
    }
    #[doc = "Bit 30 - Control bit synchronization"]
    #[inline(always)]
    pub fn enhw(&mut self) -> ENHW_W {
        ENHW_W { w: self }
    }
    #[doc = "Bit 31 - Disable bit internal clock"]
    #[inline(always)]
    pub fn disclk(&mut self) -> DISCLK_W {
        DISCLK_W { w: self }
    }
}
