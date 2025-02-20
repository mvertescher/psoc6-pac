#[doc = "Reader of register MEM_DATA[%s]"]
pub type R = crate::R<u32, super::MEM_DATA>;
#[doc = "Writer for register MEM_DATA[%s]"]
pub type W = crate::W<u32, super::MEM_DATA>;
#[doc = "Register MEM_DATA[%s] `reset()`'s with value 0"]
impl crate::ResetValue for super::MEM_DATA {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DR`"]
pub type DR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DR`"]
pub struct DR_W<'a> {
    w: &'a mut W,
}
impl<'a> DR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Data Register for EP ; This register is linked to the memory, hence reset value is undefined"]
    #[inline(always)]
    pub fn dr(&self) -> DR_R {
        DR_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Data Register for EP ; This register is linked to the memory, hence reset value is undefined"]
    #[inline(always)]
    pub fn dr(&mut self) -> DR_W {
        DR_W { w: self }
    }
}
