#[doc = "Reader of register MISC_CTRL_REG"]
pub type R = crate::R<u32, super::MISC_CTRL_REG>;
#[doc = "Writer for register MISC_CTRL_REG"]
pub type W = crate::W<u32, super::MISC_CTRL_REG>;
#[doc = "Register MISC_CTRL_REG `reset()`'s with value 0xaaaa"]
impl crate::ResetValue for super::MISC_CTRL_REG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xaaaa
    }
}
#[doc = "Possible values of the field `WRITE_LOCK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WRITE_LOCK_A {
    #[doc = "Restricted mode."]
    RESTRICTED,
    #[doc = "Secure control registers can be written."]
    ACCESSIBLE,
}
impl From<WRITE_LOCK_A> for u8 {
    #[inline(always)]
    fn from(variant: WRITE_LOCK_A) -> Self {
        match variant {
            WRITE_LOCK_A::RESTRICTED => 1,
            WRITE_LOCK_A::ACCESSIBLE => 2,
        }
    }
}
#[doc = "Reader of field `WRITE_LOCK`"]
pub type WRITE_LOCK_R = crate::R<u8, WRITE_LOCK_A>;
impl WRITE_LOCK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, WRITE_LOCK_A> {
        use crate::Variant::*;
        match self.bits {
            1 => Val(WRITE_LOCK_A::RESTRICTED),
            2 => Val(WRITE_LOCK_A::ACCESSIBLE),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `RESTRICTED`"]
    #[inline(always)]
    pub fn is_restricted(&self) -> bool {
        *self == WRITE_LOCK_A::RESTRICTED
    }
    #[doc = "Checks if the value of the field is `ACCESSIBLE`"]
    #[inline(always)]
    pub fn is_accessible(&self) -> bool {
        *self == WRITE_LOCK_A::ACCESSIBLE
    }
}
#[doc = "Write proxy for field `WRITE_LOCK`"]
pub struct WRITE_LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> WRITE_LOCK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WRITE_LOCK_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Restricted mode."]
    #[inline(always)]
    pub fn restricted(self) -> &'a mut W {
        self.variant(WRITE_LOCK_A::RESTRICTED)
    }
    #[doc = "Secure control registers can be written."]
    #[inline(always)]
    pub fn accessible(self) -> &'a mut W {
        self.variant(WRITE_LOCK_A::ACCESSIBLE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Possible values of the field `ENABLE_SECURE_CHECKING`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENABLE_SECURE_CHECKING_A {
    #[doc = "Restricted mode."]
    ENABLE,
    #[doc = "Disable check."]
    DISABLE,
}
impl From<ENABLE_SECURE_CHECKING_A> for u8 {
    #[inline(always)]
    fn from(variant: ENABLE_SECURE_CHECKING_A) -> Self {
        match variant {
            ENABLE_SECURE_CHECKING_A::ENABLE => 1,
            ENABLE_SECURE_CHECKING_A::DISABLE => 2,
        }
    }
}
#[doc = "Reader of field `ENABLE_SECURE_CHECKING`"]
pub type ENABLE_SECURE_CHECKING_R = crate::R<u8, ENABLE_SECURE_CHECKING_A>;
impl ENABLE_SECURE_CHECKING_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, ENABLE_SECURE_CHECKING_A> {
        use crate::Variant::*;
        match self.bits {
            1 => Val(ENABLE_SECURE_CHECKING_A::ENABLE),
            2 => Val(ENABLE_SECURE_CHECKING_A::DISABLE),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ENABLE_SECURE_CHECKING_A::ENABLE
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ENABLE_SECURE_CHECKING_A::DISABLE
    }
}
#[doc = "Write proxy for field `ENABLE_SECURE_CHECKING`"]
pub struct ENABLE_SECURE_CHECKING_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE_SECURE_CHECKING_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENABLE_SECURE_CHECKING_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Restricted mode."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ENABLE_SECURE_CHECKING_A::ENABLE)
    }
    #[doc = "Disable check."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ENABLE_SECURE_CHECKING_A::DISABLE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Possible values of the field `ENABLE_S_PRIV_CHECK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENABLE_S_PRIV_CHECK_A {
    #[doc = "Restricted mode."]
    ENABLE,
    #[doc = "Disable check."]
    DISABLE,
}
impl From<ENABLE_S_PRIV_CHECK_A> for u8 {
    #[inline(always)]
    fn from(variant: ENABLE_S_PRIV_CHECK_A) -> Self {
        match variant {
            ENABLE_S_PRIV_CHECK_A::ENABLE => 1,
            ENABLE_S_PRIV_CHECK_A::DISABLE => 2,
        }
    }
}
#[doc = "Reader of field `ENABLE_S_PRIV_CHECK`"]
pub type ENABLE_S_PRIV_CHECK_R = crate::R<u8, ENABLE_S_PRIV_CHECK_A>;
impl ENABLE_S_PRIV_CHECK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, ENABLE_S_PRIV_CHECK_A> {
        use crate::Variant::*;
        match self.bits {
            1 => Val(ENABLE_S_PRIV_CHECK_A::ENABLE),
            2 => Val(ENABLE_S_PRIV_CHECK_A::DISABLE),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ENABLE_S_PRIV_CHECK_A::ENABLE
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ENABLE_S_PRIV_CHECK_A::DISABLE
    }
}
#[doc = "Write proxy for field `ENABLE_S_PRIV_CHECK`"]
pub struct ENABLE_S_PRIV_CHECK_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE_S_PRIV_CHECK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENABLE_S_PRIV_CHECK_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Restricted mode."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ENABLE_S_PRIV_CHECK_A::ENABLE)
    }
    #[doc = "Disable check."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ENABLE_S_PRIV_CHECK_A::DISABLE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Possible values of the field `ENABLE_NS_PRIV_CHECK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENABLE_NS_PRIV_CHECK_A {
    #[doc = "Restricted mode."]
    ENABLE,
    #[doc = "Disable check."]
    DISABLE,
}
impl From<ENABLE_NS_PRIV_CHECK_A> for u8 {
    #[inline(always)]
    fn from(variant: ENABLE_NS_PRIV_CHECK_A) -> Self {
        match variant {
            ENABLE_NS_PRIV_CHECK_A::ENABLE => 1,
            ENABLE_NS_PRIV_CHECK_A::DISABLE => 2,
        }
    }
}
#[doc = "Reader of field `ENABLE_NS_PRIV_CHECK`"]
pub type ENABLE_NS_PRIV_CHECK_R = crate::R<u8, ENABLE_NS_PRIV_CHECK_A>;
impl ENABLE_NS_PRIV_CHECK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, ENABLE_NS_PRIV_CHECK_A> {
        use crate::Variant::*;
        match self.bits {
            1 => Val(ENABLE_NS_PRIV_CHECK_A::ENABLE),
            2 => Val(ENABLE_NS_PRIV_CHECK_A::DISABLE),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ENABLE_NS_PRIV_CHECK_A::ENABLE
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ENABLE_NS_PRIV_CHECK_A::DISABLE
    }
}
#[doc = "Write proxy for field `ENABLE_NS_PRIV_CHECK`"]
pub struct ENABLE_NS_PRIV_CHECK_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE_NS_PRIV_CHECK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENABLE_NS_PRIV_CHECK_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Restricted mode."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ENABLE_NS_PRIV_CHECK_A::ENABLE)
    }
    #[doc = "Disable check."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ENABLE_NS_PRIV_CHECK_A::DISABLE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "Possible values of the field `DISABLE_VIOLATION_ABORT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DISABLE_VIOLATION_ABORT_A {
    #[doc = "Disable abort fort secure checker."]
    DISABLE,
    #[doc = "Enable abort fort secure checker."]
    ENABLE,
}
impl From<DISABLE_VIOLATION_ABORT_A> for u8 {
    #[inline(always)]
    fn from(variant: DISABLE_VIOLATION_ABORT_A) -> Self {
        match variant {
            DISABLE_VIOLATION_ABORT_A::DISABLE => 1,
            DISABLE_VIOLATION_ABORT_A::ENABLE => 2,
        }
    }
}
#[doc = "Reader of field `DISABLE_VIOLATION_ABORT`"]
pub type DISABLE_VIOLATION_ABORT_R = crate::R<u8, DISABLE_VIOLATION_ABORT_A>;
impl DISABLE_VIOLATION_ABORT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, DISABLE_VIOLATION_ABORT_A> {
        use crate::Variant::*;
        match self.bits {
            1 => Val(DISABLE_VIOLATION_ABORT_A::DISABLE),
            2 => Val(DISABLE_VIOLATION_ABORT_A::ENABLE),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == DISABLE_VIOLATION_ABORT_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == DISABLE_VIOLATION_ABORT_A::ENABLE
    }
}
#[doc = "Write proxy for field `DISABLE_VIOLATION_ABORT`"]
pub struct DISABLE_VIOLATION_ABORT_W<'a> {
    w: &'a mut W,
}
impl<'a> DISABLE_VIOLATION_ABORT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DISABLE_VIOLATION_ABORT_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Disable abort fort secure checker."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(DISABLE_VIOLATION_ABORT_A::DISABLE)
    }
    #[doc = "Enable abort fort secure checker."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(DISABLE_VIOLATION_ABORT_A::ENABLE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Possible values of the field `DISABLE_SIMPLE_MASTER_STRICT_MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DISABLE_SIMPLE_MASTER_STRICT_MODE_A {
    #[doc = "Simple master in tier mode."]
    TIER_MODE,
    #[doc = "Simple master in strict mode."]
    STRICT_MODE,
}
impl From<DISABLE_SIMPLE_MASTER_STRICT_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: DISABLE_SIMPLE_MASTER_STRICT_MODE_A) -> Self {
        match variant {
            DISABLE_SIMPLE_MASTER_STRICT_MODE_A::TIER_MODE => 1,
            DISABLE_SIMPLE_MASTER_STRICT_MODE_A::STRICT_MODE => 2,
        }
    }
}
#[doc = "Reader of field `DISABLE_SIMPLE_MASTER_STRICT_MODE`"]
pub type DISABLE_SIMPLE_MASTER_STRICT_MODE_R = crate::R<u8, DISABLE_SIMPLE_MASTER_STRICT_MODE_A>;
impl DISABLE_SIMPLE_MASTER_STRICT_MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, DISABLE_SIMPLE_MASTER_STRICT_MODE_A> {
        use crate::Variant::*;
        match self.bits {
            1 => Val(DISABLE_SIMPLE_MASTER_STRICT_MODE_A::TIER_MODE),
            2 => Val(DISABLE_SIMPLE_MASTER_STRICT_MODE_A::STRICT_MODE),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `TIER_MODE`"]
    #[inline(always)]
    pub fn is_tier_mode(&self) -> bool {
        *self == DISABLE_SIMPLE_MASTER_STRICT_MODE_A::TIER_MODE
    }
    #[doc = "Checks if the value of the field is `STRICT_MODE`"]
    #[inline(always)]
    pub fn is_strict_mode(&self) -> bool {
        *self == DISABLE_SIMPLE_MASTER_STRICT_MODE_A::STRICT_MODE
    }
}
#[doc = "Write proxy for field `DISABLE_SIMPLE_MASTER_STRICT_MODE`"]
pub struct DISABLE_SIMPLE_MASTER_STRICT_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> DISABLE_SIMPLE_MASTER_STRICT_MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DISABLE_SIMPLE_MASTER_STRICT_MODE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Simple master in tier mode."]
    #[inline(always)]
    pub fn tier_mode(self) -> &'a mut W {
        self.variant(DISABLE_SIMPLE_MASTER_STRICT_MODE_A::TIER_MODE)
    }
    #[doc = "Simple master in strict mode."]
    #[inline(always)]
    pub fn strict_mode(self) -> &'a mut W {
        self.variant(DISABLE_SIMPLE_MASTER_STRICT_MODE_A::STRICT_MODE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
        self.w
    }
}
#[doc = "Possible values of the field `DISABLE_SMART_MASTER_STRICT_MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DISABLE_SMART_MASTER_STRICT_MODE_A {
    #[doc = "Smart master in tier mode."]
    TIER_MODE,
    #[doc = "Smart master in strict mode."]
    STRICT_MODE,
}
impl From<DISABLE_SMART_MASTER_STRICT_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: DISABLE_SMART_MASTER_STRICT_MODE_A) -> Self {
        match variant {
            DISABLE_SMART_MASTER_STRICT_MODE_A::TIER_MODE => 1,
            DISABLE_SMART_MASTER_STRICT_MODE_A::STRICT_MODE => 2,
        }
    }
}
#[doc = "Reader of field `DISABLE_SMART_MASTER_STRICT_MODE`"]
pub type DISABLE_SMART_MASTER_STRICT_MODE_R = crate::R<u8, DISABLE_SMART_MASTER_STRICT_MODE_A>;
impl DISABLE_SMART_MASTER_STRICT_MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, DISABLE_SMART_MASTER_STRICT_MODE_A> {
        use crate::Variant::*;
        match self.bits {
            1 => Val(DISABLE_SMART_MASTER_STRICT_MODE_A::TIER_MODE),
            2 => Val(DISABLE_SMART_MASTER_STRICT_MODE_A::STRICT_MODE),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `TIER_MODE`"]
    #[inline(always)]
    pub fn is_tier_mode(&self) -> bool {
        *self == DISABLE_SMART_MASTER_STRICT_MODE_A::TIER_MODE
    }
    #[doc = "Checks if the value of the field is `STRICT_MODE`"]
    #[inline(always)]
    pub fn is_strict_mode(&self) -> bool {
        *self == DISABLE_SMART_MASTER_STRICT_MODE_A::STRICT_MODE
    }
}
#[doc = "Write proxy for field `DISABLE_SMART_MASTER_STRICT_MODE`"]
pub struct DISABLE_SMART_MASTER_STRICT_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> DISABLE_SMART_MASTER_STRICT_MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DISABLE_SMART_MASTER_STRICT_MODE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Smart master in tier mode."]
    #[inline(always)]
    pub fn tier_mode(self) -> &'a mut W {
        self.variant(DISABLE_SMART_MASTER_STRICT_MODE_A::TIER_MODE)
    }
    #[doc = "Smart master in strict mode."]
    #[inline(always)]
    pub fn strict_mode(self) -> &'a mut W {
        self.variant(DISABLE_SMART_MASTER_STRICT_MODE_A::STRICT_MODE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
#[doc = "Possible values of the field `IDAU_ALL_NS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IDAU_ALL_NS_A {
    #[doc = "IDAU is disable."]
    DISABLE,
    #[doc = "IDAU is enabled."]
    ENABLE,
}
impl From<IDAU_ALL_NS_A> for u8 {
    #[inline(always)]
    fn from(variant: IDAU_ALL_NS_A) -> Self {
        match variant {
            IDAU_ALL_NS_A::DISABLE => 1,
            IDAU_ALL_NS_A::ENABLE => 2,
        }
    }
}
#[doc = "Reader of field `IDAU_ALL_NS`"]
pub type IDAU_ALL_NS_R = crate::R<u8, IDAU_ALL_NS_A>;
impl IDAU_ALL_NS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, IDAU_ALL_NS_A> {
        use crate::Variant::*;
        match self.bits {
            1 => Val(IDAU_ALL_NS_A::DISABLE),
            2 => Val(IDAU_ALL_NS_A::ENABLE),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == IDAU_ALL_NS_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == IDAU_ALL_NS_A::ENABLE
    }
}
#[doc = "Write proxy for field `IDAU_ALL_NS`"]
pub struct IDAU_ALL_NS_W<'a> {
    w: &'a mut W,
}
impl<'a> IDAU_ALL_NS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IDAU_ALL_NS_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "IDAU is disable."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(IDAU_ALL_NS_A::DISABLE)
    }
    #[doc = "IDAU is enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(IDAU_ALL_NS_A::ENABLE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Write lock."]
    #[inline(always)]
    pub fn write_lock(&self) -> WRITE_LOCK_R {
        WRITE_LOCK_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - Enable secure check for AHB matrix."]
    #[inline(always)]
    pub fn enable_secure_checking(&self) -> ENABLE_SECURE_CHECKING_R {
        ENABLE_SECURE_CHECKING_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - Enable secure privilege check for AHB matrix."]
    #[inline(always)]
    pub fn enable_s_priv_check(&self) -> ENABLE_S_PRIV_CHECK_R {
        ENABLE_S_PRIV_CHECK_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - Enable non-secure privilege check for AHB matrix."]
    #[inline(always)]
    pub fn enable_ns_priv_check(&self) -> ENABLE_NS_PRIV_CHECK_R {
        ENABLE_NS_PRIV_CHECK_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - Disable secure violation abort."]
    #[inline(always)]
    pub fn disable_violation_abort(&self) -> DISABLE_VIOLATION_ABORT_R {
        DISABLE_VIOLATION_ABORT_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:11 - Disable simple master strict mode."]
    #[inline(always)]
    pub fn disable_simple_master_strict_mode(&self) -> DISABLE_SIMPLE_MASTER_STRICT_MODE_R {
        DISABLE_SIMPLE_MASTER_STRICT_MODE_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 12:13 - Disable smart master strict mode."]
    #[inline(always)]
    pub fn disable_smart_master_strict_mode(&self) -> DISABLE_SMART_MASTER_STRICT_MODE_R {
        DISABLE_SMART_MASTER_STRICT_MODE_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 14:15 - Disable IDAU."]
    #[inline(always)]
    pub fn idau_all_ns(&self) -> IDAU_ALL_NS_R {
        IDAU_ALL_NS_R::new(((self.bits >> 14) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Write lock."]
    #[inline(always)]
    pub fn write_lock(&mut self) -> WRITE_LOCK_W {
        WRITE_LOCK_W { w: self }
    }
    #[doc = "Bits 2:3 - Enable secure check for AHB matrix."]
    #[inline(always)]
    pub fn enable_secure_checking(&mut self) -> ENABLE_SECURE_CHECKING_W {
        ENABLE_SECURE_CHECKING_W { w: self }
    }
    #[doc = "Bits 4:5 - Enable secure privilege check for AHB matrix."]
    #[inline(always)]
    pub fn enable_s_priv_check(&mut self) -> ENABLE_S_PRIV_CHECK_W {
        ENABLE_S_PRIV_CHECK_W { w: self }
    }
    #[doc = "Bits 6:7 - Enable non-secure privilege check for AHB matrix."]
    #[inline(always)]
    pub fn enable_ns_priv_check(&mut self) -> ENABLE_NS_PRIV_CHECK_W {
        ENABLE_NS_PRIV_CHECK_W { w: self }
    }
    #[doc = "Bits 8:9 - Disable secure violation abort."]
    #[inline(always)]
    pub fn disable_violation_abort(&mut self) -> DISABLE_VIOLATION_ABORT_W {
        DISABLE_VIOLATION_ABORT_W { w: self }
    }
    #[doc = "Bits 10:11 - Disable simple master strict mode."]
    #[inline(always)]
    pub fn disable_simple_master_strict_mode(&mut self) -> DISABLE_SIMPLE_MASTER_STRICT_MODE_W {
        DISABLE_SIMPLE_MASTER_STRICT_MODE_W { w: self }
    }
    #[doc = "Bits 12:13 - Disable smart master strict mode."]
    #[inline(always)]
    pub fn disable_smart_master_strict_mode(&mut self) -> DISABLE_SMART_MASTER_STRICT_MODE_W {
        DISABLE_SMART_MASTER_STRICT_MODE_W { w: self }
    }
    #[doc = "Bits 14:15 - Disable IDAU."]
    #[inline(always)]
    pub fn idau_all_ns(&mut self) -> IDAU_ALL_NS_W {
        IDAU_ALL_NS_W { w: self }
    }
}
