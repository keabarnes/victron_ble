use crate::bit_reader::BitReader;
use crate::err::*;

use super::error_state::ErrorState;
use super::mode::Mode;
use super::off_reason::OffReason;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct OrionXSState {
    pub operation_mode: Option<Mode>,
    pub charger_error: Option<ErrorState>,
    pub input_voltage_v: Option<f32>,
    pub input_current_a: Option<f32>,
    pub output_voltage_v: Option<f32>,
    pub output_current_a: Option<f32>,
    pub off_reason: OffReason,
}

impl OrionXSState {
    pub(crate) fn parse(payload: &[u8]) -> Result<Self> {
        let mut reader = BitReader::new(payload);

        let device_state = reader.read_unsigned_int(8)?;
        let charger_error = reader.read_unsigned_int(8)?;
        let output_voltage = reader.read_unsigned_int(16)?;
        let output_current = reader.read_unsigned_int(16)?;
        let input_voltage = reader.read_unsigned_int(16)?;
        let input_current = reader.read_unsigned_int(16)?;
        let off_reason = reader.read_unsigned_int(32)?;

        let operation_mode = if device_state != 0xFF {
            Some(Mode::try_from(device_state)?)
        } else {
            None
        };

        let charger_error = if charger_error != 0xFF {
            Some(ErrorState::try_from(charger_error)?)
        } else {
            None
        };

        let output_voltage_v = if output_voltage != 0xFFFF {
            Some(output_voltage as f32 / 100.0)
        } else {
            None
        };

        let output_current_a = if output_current != 0xFFFF {
            Some(output_current as f32 / 10.0)
        } else {
            None
        };

        let input_voltage_v = if input_voltage != 0xFFFF {
            Some(input_voltage as f32 / 100.0)
        } else {
            None
        };

        let input_current_a = if input_current != 0xFFFF {
            Some(input_current as f32 / 10.0)
        } else {
            None
        };

        Ok(Self {
            operation_mode,
            charger_error,
            input_voltage_v,
            input_current_a,
            output_voltage_v,
            output_current_a,
            off_reason: OffReason::from_bits_truncate(off_reason as u32),
        })
    }
}