#[doc = "Reader of register PLL0CLKSEL"]
pub type R = crate::R<u32, super::PLL0CLKSEL>;
#[doc = "Writer for register PLL0CLKSEL"]
pub type W = crate::W<u32, super::PLL0CLKSEL>;
#[doc = "Register PLL0CLKSEL `reset()`'s with value 0x07"]
impl crate::ResetValue for super::PLL0CLKSEL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x07
    }
}
#[doc = "Possible values of the field `SEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SEL_A {
    #[doc = "FRO 12 MHz clock."]
    ENUM_0X0,
    #[doc = "CLKIN clock."]
    ENUM_0X1,
    #[doc = "FRO 1MHz clock."]
    ENUM_0X2,
    #[doc = "Oscillator 32kHz clock."]
    ENUM_0X3,
    #[doc = "No clock."]
    ENUM_0X4,
    #[doc = "No clock."]
    ENUM_0X5,
    #[doc = "No clock."]
    ENUM_0X6,
    #[doc = "No clock."]
    ENUM_0X7,
}
impl From<SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: SEL_A) -> Self {
        match variant {
            SEL_A::ENUM_0X0 => 0,
            SEL_A::ENUM_0X1 => 1,
            SEL_A::ENUM_0X2 => 2,
            SEL_A::ENUM_0X3 => 3,
            SEL_A::ENUM_0X4 => 4,
            SEL_A::ENUM_0X5 => 5,
            SEL_A::ENUM_0X6 => 6,
            SEL_A::ENUM_0X7 => 7,
        }
    }
}
#[doc = "Reader of field `SEL`"]
pub type SEL_R = crate::R<u8, SEL_A>;
impl SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SEL_A {
        match self.bits {
            0 => SEL_A::ENUM_0X0,
            1 => SEL_A::ENUM_0X1,
            2 => SEL_A::ENUM_0X2,
            3 => SEL_A::ENUM_0X3,
            4 => SEL_A::ENUM_0X4,
            5 => SEL_A::ENUM_0X5,
            6 => SEL_A::ENUM_0X6,
            7 => SEL_A::ENUM_0X7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ENUM_0X0`"]
    #[inline(always)]
    pub fn is_enum_0x0(&self) -> bool {
        *self == SEL_A::ENUM_0X0
    }
    #[doc = "Checks if the value of the field is `ENUM_0X1`"]
    #[inline(always)]
    pub fn is_enum_0x1(&self) -> bool {
        *self == SEL_A::ENUM_0X1
    }
    #[doc = "Checks if the value of the field is `ENUM_0X2`"]
    #[inline(always)]
    pub fn is_enum_0x2(&self) -> bool {
        *self == SEL_A::ENUM_0X2
    }
    #[doc = "Checks if the value of the field is `ENUM_0X3`"]
    #[inline(always)]
    pub fn is_enum_0x3(&self) -> bool {
        *self == SEL_A::ENUM_0X3
    }
    #[doc = "Checks if the value of the field is `ENUM_0X4`"]
    #[inline(always)]
    pub fn is_enum_0x4(&self) -> bool {
        *self == SEL_A::ENUM_0X4
    }
    #[doc = "Checks if the value of the field is `ENUM_0X5`"]
    #[inline(always)]
    pub fn is_enum_0x5(&self) -> bool {
        *self == SEL_A::ENUM_0X5
    }
    #[doc = "Checks if the value of the field is `ENUM_0X6`"]
    #[inline(always)]
    pub fn is_enum_0x6(&self) -> bool {
        *self == SEL_A::ENUM_0X6
    }
    #[doc = "Checks if the value of the field is `ENUM_0X7`"]
    #[inline(always)]
    pub fn is_enum_0x7(&self) -> bool {
        *self == SEL_A::ENUM_0X7
    }
}
#[doc = "Write proxy for field `SEL`"]
pub struct SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "FRO 12 MHz clock."]
    #[inline(always)]
    pub fn enum_0x0(self) -> &'a mut W {
        self.variant(SEL_A::ENUM_0X0)
    }
    #[doc = "CLKIN clock."]
    #[inline(always)]
    pub fn enum_0x1(self) -> &'a mut W {
        self.variant(SEL_A::ENUM_0X1)
    }
    #[doc = "FRO 1MHz clock."]
    #[inline(always)]
    pub fn enum_0x2(self) -> &'a mut W {
        self.variant(SEL_A::ENUM_0X2)
    }
    #[doc = "Oscillator 32kHz clock."]
    #[inline(always)]
    pub fn enum_0x3(self) -> &'a mut W {
        self.variant(SEL_A::ENUM_0X3)
    }
    #[doc = "No clock."]
    #[inline(always)]
    pub fn enum_0x4(self) -> &'a mut W {
        self.variant(SEL_A::ENUM_0X4)
    }
    #[doc = "No clock."]
    #[inline(always)]
    pub fn enum_0x5(self) -> &'a mut W {
        self.variant(SEL_A::ENUM_0X5)
    }
    #[doc = "No clock."]
    #[inline(always)]
    pub fn enum_0x6(self) -> &'a mut W {
        self.variant(SEL_A::ENUM_0X6)
    }
    #[doc = "No clock."]
    #[inline(always)]
    pub fn enum_0x7(self) -> &'a mut W {
        self.variant(SEL_A::ENUM_0X7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - PLL0 clock source select."]
    #[inline(always)]
    pub fn sel(&self) -> SEL_R {
        SEL_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - PLL0 clock source select."]
    #[inline(always)]
    pub fn sel(&mut self) -> SEL_W {
        SEL_W { w: self }
    }
}
