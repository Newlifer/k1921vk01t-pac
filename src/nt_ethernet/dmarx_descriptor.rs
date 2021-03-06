#[doc = "Reader of register DMARxDescriptor"]
pub type R = crate::R<u32, super::DMARXDESCRIPTOR>;
#[doc = "Writer for register DMARxDescriptor"]
pub type W = crate::W<u32, super::DMARXDESCRIPTOR>;
#[doc = "Register DMARxDescriptor `reset()`'s with value 0"]
impl crate::ResetValue for super::DMARXDESCRIPTOR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DADDR`"]
pub type DADDR_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `DADDR`"]
pub struct DADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> DADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3fff_ffff << 2)) | (((value as u32) & 0x3fff_ffff) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bits 2:31"]
    #[inline(always)]
    pub fn daddr(&self) -> DADDR_R {
        DADDR_R::new(((self.bits >> 2) & 0x3fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 2:31"]
    #[inline(always)]
    pub fn daddr(&mut self) -> DADDR_W {
        DADDR_W { w: self }
    }
}
