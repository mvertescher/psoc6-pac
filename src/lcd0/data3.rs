#[doc = "Reader of register DATA3[%s]"]
pub type R = crate::R<u32, super::DATA3>;
#[doc = "Writer for register DATA3[%s]"]
pub type W = crate::W<u32, super::DATA3>;
#[doc = "Register DATA3[%s] `reset()`'s with value 0"]
impl crate::ResetValue for super::DATA3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DATA`"]
pub type DATA_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `DATA`"]
pub struct DATA_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Bits \\[4i+3:4i\\] represent the pin data for pin \\[i\\] for COMS 13-16 (COM13 is lsb)."]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Bits \\[4i+3:4i\\] represent the pin data for pin \\[i\\] for COMS 13-16 (COM13 is lsb)."]
    #[inline(always)]
    pub fn data(&mut self) -> DATA_W {
        DATA_W { w: self }
    }
}
