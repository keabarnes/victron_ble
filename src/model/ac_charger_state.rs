use crate::bit_reader::BitReader;
use crate::err::*;

use super::error_state::ErrorState;
use super::mode::Mode;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct AcChargerState {
    pub charge_state: Option<Mode>,
    pub charger_error: Option<ErrorState>,
    pub output_voltage1: Option<f32>,
    pub output_current1: Option<f32>,
    pub output_voltage2: Option<f32>,
    pub output_current2: Option<f32>,
    pub output_voltage3: Option<f32>,
    pub output_current3: Option<f32>,
    pub temperature: Option<f32>,
    pub ac_current: Option<f32>,
}

impl AcChargerState {
    pub(crate) fn parse(payload: &[u8]) -> Result<Self> {
        let mut reader = BitReader::new(payload);

        let charge_state = reader.read_unsigned_int(8)?;
        let charger_error = reader.read_unsigned_int(8)?;
        let output_voltage1 = reader.read_unsigned_int(13)?;
        let output_current1 = reader.read_unsigned_int(11)?;
        let output_voltage2 = reader.read_unsigned_int(13)?;
        let output_current2 = reader.read_unsigned_int(11)?;
        let output_voltage3 = reader.read_unsigned_int(13)?;
        let output_current3 = reader.read_unsigned_int(11)?;
        let temperature = reader.read_unsigned_int(7)?;
        let ac_current = reader.read_unsigned_int(9)?;

        let charge_state = if charge_state != 0xFF {
            Some(Mode::try_from(charge_state)?)
        } else {
            None
        };

        let charger_error = if charger_error != 0xFF {
            Some(ErrorState::try_from(charger_error)?)
        } else {
            None
        };

        let output_voltage1 = if output_voltage1 != 0x1FFF {
            Some(output_voltage1 as f32 / 100.0)
        } else {
            None
        };

        let output_current1 = if output_current1 != 0x7FF {
            Some(output_current1 as f32 / 10.0)
        } else {
            None
        };

        let output_voltage2 = if output_voltage2 != 0x1FFF {
            Some(output_voltage2 as f32 / 100.0)
        } else {
            None
        };

        let output_current2 = if output_current2 != 0x7FF {
            Some(output_current2 as f32 / 10.0)
        } else {
            None
        };

        let output_voltage3 = if output_voltage3 != 0x1FFF {
            Some(output_voltage3 as f32 / 100.0)
        } else {
            None
        };

        let output_current3 = if output_current3 != 0x7FF {
            Some(output_current3 as f32 / 10.0)
        } else {
            None
        };

        let temperature = if temperature != 0x7F {
            Some((temperature as f32) - 40.0)
        } else {
            None
        };

        let ac_current = if ac_current != 0x1FF {
            Some(ac_current as f32 / 10.0)
        } else {
            None
        };

        Ok(Self {
            charge_state,
            charger_error,
            output_voltage1,
            output_current1,
            output_voltage2,
            output_current2,
            output_voltage3,
            output_current3,
            temperature,
            ac_current,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::model::Mode;

    #[test]
    fn test_parse_ac_charger() {
        // Test data from https://github.com/keshavdv/victron-ble: "060046a500ffffffffffffbdffeb3d1f"
        // Expected values: charge_state=STORAGE, charger_error=NO_ERROR, output_voltage1=13.5, output_current1=0.5, temperature=21
        let payload = hex::decode("060046a500ffffffffffffbdffeb3d1f").unwrap();
        let result = AcChargerState::parse(&payload).unwrap();

        assert_eq!(result.charge_state, Some(Mode::Storage));
        assert_eq!(result.charger_error, Some(ErrorState::NoError));
        assert_eq!(result.output_voltage1, Some(13.5));
        assert_eq!(result.output_current1, Some(0.5));
        assert_eq!(result.output_voltage2, None);
        assert_eq!(result.output_current2, None);
        assert_eq!(result.output_voltage3, None);
        assert_eq!(result.output_current3, None);
        assert_eq!(result.temperature, Some(21.0));
        assert_eq!(result.ac_current, None);
    }
}

