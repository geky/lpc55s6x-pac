#[doc = "Reader of register SEC_CTRL_RAM3_SLAVE_RULE"]
pub type R = crate::R<u32, super::SEC_CTRL_RAM3_SLAVE_RULE>;
#[doc = "Writer for register SEC_CTRL_RAM3_SLAVE_RULE"]
pub type W = crate::W<u32, super::SEC_CTRL_RAM3_SLAVE_RULE>;
#[doc = "Register SEC_CTRL_RAM3_SLAVE_RULE `reset()`'s with value 0"]
impl crate::ResetValue for super::SEC_CTRL_RAM3_SLAVE_RULE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Possible values of the field `RAM3_RULE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RAM3_RULE_A {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP,
    #[doc = "Non-secure and Privilege access allowed."]
    ENUM_NS_P,
    #[doc = "Secure and Non-priviledge user access allowed."]
    ENUM_S_NP,
    #[doc = "Secure and Priviledge user access allowed."]
    ENUM_S_P,
}
impl From<RAM3_RULE_A> for u8 {
    #[inline(always)]
    fn from(variant: RAM3_RULE_A) -> Self {
        match variant {
            RAM3_RULE_A::ENUM_NS_NP => 0,
            RAM3_RULE_A::ENUM_NS_P => 1,
            RAM3_RULE_A::ENUM_S_NP => 2,
            RAM3_RULE_A::ENUM_S_P => 3,
        }
    }
}
#[doc = "Reader of field `RAM3_RULE`"]
pub type RAM3_RULE_R = crate::R<u8, RAM3_RULE_A>;
impl RAM3_RULE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RAM3_RULE_A {
        match self.bits {
            0 => RAM3_RULE_A::ENUM_NS_NP,
            1 => RAM3_RULE_A::ENUM_NS_P,
            2 => RAM3_RULE_A::ENUM_S_NP,
            3 => RAM3_RULE_A::ENUM_S_P,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_NP`"]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == RAM3_RULE_A::ENUM_NS_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_P`"]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == RAM3_RULE_A::ENUM_NS_P
    }
    #[doc = "Checks if the value of the field is `ENUM_S_NP`"]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        *self == RAM3_RULE_A::ENUM_S_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_S_P`"]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        *self == RAM3_RULE_A::ENUM_S_P
    }
}
#[doc = "Write proxy for field `RAM3_RULE`"]
pub struct RAM3_RULE_W<'a> {
    w: &'a mut W,
}
impl<'a> RAM3_RULE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RAM3_RULE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut W {
        self.variant(RAM3_RULE_A::ENUM_NS_NP)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut W {
        self.variant(RAM3_RULE_A::ENUM_NS_P)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut W {
        self.variant(RAM3_RULE_A::ENUM_S_NP)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut W {
        self.variant(RAM3_RULE_A::ENUM_S_P)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Security access rules for the whole RAM3: 0x2003_0000 - 0x2003_FFFF"]
    #[inline(always)]
    pub fn ram3_rule(&self) -> RAM3_RULE_R {
        RAM3_RULE_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Security access rules for the whole RAM3: 0x2003_0000 - 0x2003_FFFF"]
    #[inline(always)]
    pub fn ram3_rule(&mut self) -> RAM3_RULE_W {
        RAM3_RULE_W { w: self }
    }
}
