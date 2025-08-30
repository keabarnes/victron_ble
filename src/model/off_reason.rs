use bitflags::bitflags;

bitflags! {
    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    #[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
    pub struct OffReason: u32 {
        const NO_REASON = 0x00000000;
        const NO_INPUT_POWER = 0x00000001;
        const SWITCHED_OFF_SWITCH = 0x00000002;
        const SWITCHED_OFF_REGISTER = 0x00000004;
        const REMOTE_INPUT = 0x00000008;
    }
}