#[doc = "Reader of register CTCR"]
pub type R = crate::R<u32, super::CTCR>;
#[doc = "Writer for register CTCR"]
pub type W = crate::W<u32, super::CTCR>;
#[doc = "Register CTCR `reset()`'s with value 0"]
impl crate::ResetValue for super::CTCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Possible values of the field `CTMODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTMODE_A {
    #[doc = "Timer Mode. Incremented every rising APB bus clock edge."]
    TIMER,
    #[doc = "Counter Mode rising edge. TC is incremented on rising edges on the CAP input selected by bits 3:2."]
    COUNTER_RISING_EDGE,
    #[doc = "Counter Mode falling edge. TC is incremented on falling edges on the CAP input selected by bits 3:2."]
    COUNTER_FALLING_EDGE,
    #[doc = "Counter Mode dual edge. TC is incremented on both edges on the CAP input selected by bits 3:2."]
    COUNTER_DUAL_EDGE,
}
impl From<CTMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: CTMODE_A) -> Self {
        match variant {
            CTMODE_A::TIMER => 0,
            CTMODE_A::COUNTER_RISING_EDGE => 1,
            CTMODE_A::COUNTER_FALLING_EDGE => 2,
            CTMODE_A::COUNTER_DUAL_EDGE => 3,
        }
    }
}
#[doc = "Reader of field `CTMODE`"]
pub type CTMODE_R = crate::R<u8, CTMODE_A>;
impl CTMODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTMODE_A {
        match self.bits {
            0 => CTMODE_A::TIMER,
            1 => CTMODE_A::COUNTER_RISING_EDGE,
            2 => CTMODE_A::COUNTER_FALLING_EDGE,
            3 => CTMODE_A::COUNTER_DUAL_EDGE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `TIMER`"]
    #[inline(always)]
    pub fn is_timer(&self) -> bool {
        *self == CTMODE_A::TIMER
    }
    #[doc = "Checks if the value of the field is `COUNTER_RISING_EDGE`"]
    #[inline(always)]
    pub fn is_counter_rising_edge(&self) -> bool {
        *self == CTMODE_A::COUNTER_RISING_EDGE
    }
    #[doc = "Checks if the value of the field is `COUNTER_FALLING_EDGE`"]
    #[inline(always)]
    pub fn is_counter_falling_edge(&self) -> bool {
        *self == CTMODE_A::COUNTER_FALLING_EDGE
    }
    #[doc = "Checks if the value of the field is `COUNTER_DUAL_EDGE`"]
    #[inline(always)]
    pub fn is_counter_dual_edge(&self) -> bool {
        *self == CTMODE_A::COUNTER_DUAL_EDGE
    }
}
#[doc = "Write proxy for field `CTMODE`"]
pub struct CTMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> CTMODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTMODE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Timer Mode. Incremented every rising APB bus clock edge."]
    #[inline(always)]
    pub fn timer(self) -> &'a mut W {
        self.variant(CTMODE_A::TIMER)
    }
    #[doc = "Counter Mode rising edge. TC is incremented on rising edges on the CAP input selected by bits 3:2."]
    #[inline(always)]
    pub fn counter_rising_edge(self) -> &'a mut W {
        self.variant(CTMODE_A::COUNTER_RISING_EDGE)
    }
    #[doc = "Counter Mode falling edge. TC is incremented on falling edges on the CAP input selected by bits 3:2."]
    #[inline(always)]
    pub fn counter_falling_edge(self) -> &'a mut W {
        self.variant(CTMODE_A::COUNTER_FALLING_EDGE)
    }
    #[doc = "Counter Mode dual edge. TC is incremented on both edges on the CAP input selected by bits 3:2."]
    #[inline(always)]
    pub fn counter_dual_edge(self) -> &'a mut W {
        self.variant(CTMODE_A::COUNTER_DUAL_EDGE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Possible values of the field `CINSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CINSEL_A {
    #[doc = "Channel 0. CAPn.0 for CTIMERn"]
    CHANNEL_0,
    #[doc = "Channel 1. CAPn.1 for CTIMERn"]
    CHANNEL_1,
    #[doc = "Channel 2. CAPn.2 for CTIMERn"]
    CHANNEL_2,
    #[doc = "Channel 3. CAPn.3 for CTIMERn"]
    CHANNEL_3,
}
impl From<CINSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: CINSEL_A) -> Self {
        match variant {
            CINSEL_A::CHANNEL_0 => 0,
            CINSEL_A::CHANNEL_1 => 1,
            CINSEL_A::CHANNEL_2 => 2,
            CINSEL_A::CHANNEL_3 => 3,
        }
    }
}
#[doc = "Reader of field `CINSEL`"]
pub type CINSEL_R = crate::R<u8, CINSEL_A>;
impl CINSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CINSEL_A {
        match self.bits {
            0 => CINSEL_A::CHANNEL_0,
            1 => CINSEL_A::CHANNEL_1,
            2 => CINSEL_A::CHANNEL_2,
            3 => CINSEL_A::CHANNEL_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CHANNEL_0`"]
    #[inline(always)]
    pub fn is_channel_0(&self) -> bool {
        *self == CINSEL_A::CHANNEL_0
    }
    #[doc = "Checks if the value of the field is `CHANNEL_1`"]
    #[inline(always)]
    pub fn is_channel_1(&self) -> bool {
        *self == CINSEL_A::CHANNEL_1
    }
    #[doc = "Checks if the value of the field is `CHANNEL_2`"]
    #[inline(always)]
    pub fn is_channel_2(&self) -> bool {
        *self == CINSEL_A::CHANNEL_2
    }
    #[doc = "Checks if the value of the field is `CHANNEL_3`"]
    #[inline(always)]
    pub fn is_channel_3(&self) -> bool {
        *self == CINSEL_A::CHANNEL_3
    }
}
#[doc = "Write proxy for field `CINSEL`"]
pub struct CINSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CINSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CINSEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Channel 0. CAPn.0 for CTIMERn"]
    #[inline(always)]
    pub fn channel_0(self) -> &'a mut W {
        self.variant(CINSEL_A::CHANNEL_0)
    }
    #[doc = "Channel 1. CAPn.1 for CTIMERn"]
    #[inline(always)]
    pub fn channel_1(self) -> &'a mut W {
        self.variant(CINSEL_A::CHANNEL_1)
    }
    #[doc = "Channel 2. CAPn.2 for CTIMERn"]
    #[inline(always)]
    pub fn channel_2(self) -> &'a mut W {
        self.variant(CINSEL_A::CHANNEL_2)
    }
    #[doc = "Channel 3. CAPn.3 for CTIMERn"]
    #[inline(always)]
    pub fn channel_3(self) -> &'a mut W {
        self.variant(CINSEL_A::CHANNEL_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Reader of field `ENCC`"]
pub type ENCC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENCC`"]
pub struct ENCC_W<'a> {
    w: &'a mut W,
}
impl<'a> ENCC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Possible values of the field `SELCC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SELCC_A {
    #[doc = "Channel 0 Rising Edge. Rising edge of the signal on capture channel 0 clears the timer (if bit 4 is set)."]
    CHANNEL_0_RISING,
    #[doc = "Channel 0 Falling Edge. Falling edge of the signal on capture channel 0 clears the timer (if bit 4 is set)."]
    CHANNEL_0_FALLING,
    #[doc = "Channel 1 Rising Edge. Rising edge of the signal on capture channel 1 clears the timer (if bit 4 is set)."]
    CHANNEL_1_RISING,
    #[doc = "Channel 1 Falling Edge. Falling edge of the signal on capture channel 1 clears the timer (if bit 4 is set)."]
    CHANNEL_1_FALLING,
    #[doc = "Channel 2 Rising Edge. Rising edge of the signal on capture channel 2 clears the timer (if bit 4 is set)."]
    CHANNEL_2_RISING,
    #[doc = "Channel 2 Falling Edge. Falling edge of the signal on capture channel 2 clears the timer (if bit 4 is set)."]
    CHANNEL_2_FALLING,
}
impl From<SELCC_A> for u8 {
    #[inline(always)]
    fn from(variant: SELCC_A) -> Self {
        match variant {
            SELCC_A::CHANNEL_0_RISING => 0,
            SELCC_A::CHANNEL_0_FALLING => 1,
            SELCC_A::CHANNEL_1_RISING => 2,
            SELCC_A::CHANNEL_1_FALLING => 3,
            SELCC_A::CHANNEL_2_RISING => 4,
            SELCC_A::CHANNEL_2_FALLING => 5,
        }
    }
}
#[doc = "Reader of field `SELCC`"]
pub type SELCC_R = crate::R<u8, SELCC_A>;
impl SELCC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SELCC_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SELCC_A::CHANNEL_0_RISING),
            1 => Val(SELCC_A::CHANNEL_0_FALLING),
            2 => Val(SELCC_A::CHANNEL_1_RISING),
            3 => Val(SELCC_A::CHANNEL_1_FALLING),
            4 => Val(SELCC_A::CHANNEL_2_RISING),
            5 => Val(SELCC_A::CHANNEL_2_FALLING),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `CHANNEL_0_RISING`"]
    #[inline(always)]
    pub fn is_channel_0_rising(&self) -> bool {
        *self == SELCC_A::CHANNEL_0_RISING
    }
    #[doc = "Checks if the value of the field is `CHANNEL_0_FALLING`"]
    #[inline(always)]
    pub fn is_channel_0_falling(&self) -> bool {
        *self == SELCC_A::CHANNEL_0_FALLING
    }
    #[doc = "Checks if the value of the field is `CHANNEL_1_RISING`"]
    #[inline(always)]
    pub fn is_channel_1_rising(&self) -> bool {
        *self == SELCC_A::CHANNEL_1_RISING
    }
    #[doc = "Checks if the value of the field is `CHANNEL_1_FALLING`"]
    #[inline(always)]
    pub fn is_channel_1_falling(&self) -> bool {
        *self == SELCC_A::CHANNEL_1_FALLING
    }
    #[doc = "Checks if the value of the field is `CHANNEL_2_RISING`"]
    #[inline(always)]
    pub fn is_channel_2_rising(&self) -> bool {
        *self == SELCC_A::CHANNEL_2_RISING
    }
    #[doc = "Checks if the value of the field is `CHANNEL_2_FALLING`"]
    #[inline(always)]
    pub fn is_channel_2_falling(&self) -> bool {
        *self == SELCC_A::CHANNEL_2_FALLING
    }
}
#[doc = "Write proxy for field `SELCC`"]
pub struct SELCC_W<'a> {
    w: &'a mut W,
}
impl<'a> SELCC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SELCC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Channel 0 Rising Edge. Rising edge of the signal on capture channel 0 clears the timer (if bit 4 is set)."]
    #[inline(always)]
    pub fn channel_0_rising(self) -> &'a mut W {
        self.variant(SELCC_A::CHANNEL_0_RISING)
    }
    #[doc = "Channel 0 Falling Edge. Falling edge of the signal on capture channel 0 clears the timer (if bit 4 is set)."]
    #[inline(always)]
    pub fn channel_0_falling(self) -> &'a mut W {
        self.variant(SELCC_A::CHANNEL_0_FALLING)
    }
    #[doc = "Channel 1 Rising Edge. Rising edge of the signal on capture channel 1 clears the timer (if bit 4 is set)."]
    #[inline(always)]
    pub fn channel_1_rising(self) -> &'a mut W {
        self.variant(SELCC_A::CHANNEL_1_RISING)
    }
    #[doc = "Channel 1 Falling Edge. Falling edge of the signal on capture channel 1 clears the timer (if bit 4 is set)."]
    #[inline(always)]
    pub fn channel_1_falling(self) -> &'a mut W {
        self.variant(SELCC_A::CHANNEL_1_FALLING)
    }
    #[doc = "Channel 2 Rising Edge. Rising edge of the signal on capture channel 2 clears the timer (if bit 4 is set)."]
    #[inline(always)]
    pub fn channel_2_rising(self) -> &'a mut W {
        self.variant(SELCC_A::CHANNEL_2_RISING)
    }
    #[doc = "Channel 2 Falling Edge. Falling edge of the signal on capture channel 2 clears the timer (if bit 4 is set)."]
    #[inline(always)]
    pub fn channel_2_falling(self) -> &'a mut W {
        self.variant(SELCC_A::CHANNEL_2_FALLING)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 5)) | (((value as u32) & 0x07) << 5);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Counter/Timer Mode This field selects which rising APB bus clock edges can increment Timer's Prescale Counter (PC), or clear PC and increment Timer Counter (TC). Timer Mode: the TC is incremented when the Prescale Counter matches the Prescale Register."]
    #[inline(always)]
    pub fn ctmode(&self) -> CTMODE_R {
        CTMODE_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - Count Input Select When bits 1:0 in this register are not 00, these bits select which CAP pin is sampled for clocking. Note: If Counter mode is selected for a particular CAPn input in the CTCR, the 3 bits for that input in the Capture Control Register (CCR) must be programmed as 000. However, capture and/or interrupt can be selected for the other 3 CAPn inputs in the same timer."]
    #[inline(always)]
    pub fn cinsel(&self) -> CINSEL_R {
        CINSEL_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bit 4 - Setting this bit to 1 enables clearing of the timer and the prescaler when the capture-edge event specified in bits 7:5 occurs."]
    #[inline(always)]
    pub fn encc(&self) -> ENCC_R {
        ENCC_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 5:7 - Edge select. When bit 4 is 1, these bits select which capture input edge will cause the timer and prescaler to be cleared. These bits have no effect when bit 4 is low. Values 0x2 to 0x3 and 0x6 to 0x7 are reserved."]
    #[inline(always)]
    pub fn selcc(&self) -> SELCC_R {
        SELCC_R::new(((self.bits >> 5) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Counter/Timer Mode This field selects which rising APB bus clock edges can increment Timer's Prescale Counter (PC), or clear PC and increment Timer Counter (TC). Timer Mode: the TC is incremented when the Prescale Counter matches the Prescale Register."]
    #[inline(always)]
    pub fn ctmode(&mut self) -> CTMODE_W {
        CTMODE_W { w: self }
    }
    #[doc = "Bits 2:3 - Count Input Select When bits 1:0 in this register are not 00, these bits select which CAP pin is sampled for clocking. Note: If Counter mode is selected for a particular CAPn input in the CTCR, the 3 bits for that input in the Capture Control Register (CCR) must be programmed as 000. However, capture and/or interrupt can be selected for the other 3 CAPn inputs in the same timer."]
    #[inline(always)]
    pub fn cinsel(&mut self) -> CINSEL_W {
        CINSEL_W { w: self }
    }
    #[doc = "Bit 4 - Setting this bit to 1 enables clearing of the timer and the prescaler when the capture-edge event specified in bits 7:5 occurs."]
    #[inline(always)]
    pub fn encc(&mut self) -> ENCC_W {
        ENCC_W { w: self }
    }
    #[doc = "Bits 5:7 - Edge select. When bit 4 is 1, these bits select which capture input edge will cause the timer and prescaler to be cleared. These bits have no effect when bit 4 is low. Values 0x2 to 0x3 and 0x6 to 0x7 are reserved."]
    #[inline(always)]
    pub fn selcc(&mut self) -> SELCC_W {
        SELCC_W { w: self }
    }
}
