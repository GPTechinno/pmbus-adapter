#![no_std]

pub mod commands;
pub mod error;
pub mod formats;
pub mod status;
pub mod vout_mode;

use embedded_hal_async::i2c::I2c;
use heapless::Vec;
use smbus_adapter::SmbusAdaptor;

pub use commands::CommandCode;
pub use error::PmbusError;
pub use formats::{DirectCoefficients, Linear11, ULinear16};
pub use status::*;
pub use vout_mode::{VoutMode, VoutModeType};

// ---------------------------------------------------------------------------
// Macros to generate repetitive PMBus command methods
// ---------------------------------------------------------------------------

/// Generate a send-byte command (no data payload).
macro_rules! pmbus_send_byte {
    ($name:ident, $cmd:ident) => {
        pub async fn $name(&mut self, addr: u8) -> Result<(), BUS::Error> {
            self.send_cmd(addr, CommandCode::$cmd).await
        }
    };
}

/// Generate read-byte and write-byte pair.
macro_rules! pmbus_byte_rw {
    ($set:ident, $get:ident, $cmd:ident) => {
        pub async fn $set(&mut self, addr: u8, data: u8) -> Result<(), BUS::Error> {
            self.write_cmd_byte(addr, CommandCode::$cmd, data).await
        }
        pub async fn $get(&mut self, addr: u8) -> Result<u8, BUS::Error> {
            self.read_cmd_byte(addr, CommandCode::$cmd).await
        }
    };
}

/// Generate write-byte only.
macro_rules! pmbus_write_byte_only {
    ($name:ident, $cmd:ident) => {
        pub async fn $name(&mut self, addr: u8, data: u8) -> Result<(), BUS::Error> {
            self.write_cmd_byte(addr, CommandCode::$cmd, data).await
        }
    };
}

/// Generate read-byte only.
macro_rules! pmbus_read_byte_only {
    ($name:ident, $cmd:ident) => {
        pub async fn $name(&mut self, addr: u8) -> Result<u8, BUS::Error> {
            self.read_cmd_byte(addr, CommandCode::$cmd).await
        }
    };
}

/// Generate read-word and write-word pair.
macro_rules! pmbus_word_rw {
    ($set:ident, $get:ident, $cmd:ident) => {
        pub async fn $set(&mut self, addr: u8, data: u16) -> Result<(), BUS::Error> {
            self.write_cmd_word(addr, CommandCode::$cmd, data).await
        }
        pub async fn $get(&mut self, addr: u8) -> Result<u16, BUS::Error> {
            self.read_cmd_word(addr, CommandCode::$cmd).await
        }
    };
}

/// Generate read-word only.
macro_rules! pmbus_read_word_only {
    ($name:ident, $cmd:ident) => {
        pub async fn $name(&mut self, addr: u8) -> Result<u16, BUS::Error> {
            self.read_cmd_word(addr, CommandCode::$cmd).await
        }
    };
}

/// Generate block read and block write pair.
macro_rules! pmbus_block_rw {
    ($set:ident, $get:ident, $cmd:ident) => {
        pub async fn $set(&mut self, addr: u8, data: &[u8]) -> Result<(), BUS::Error> {
            self.block_write_cmd(addr, CommandCode::$cmd, data).await
        }
        pub async fn $get(&mut self, addr: u8) -> Result<Vec<u8, 32>, BUS::Error> {
            self.block_read_cmd(addr, CommandCode::$cmd).await
        }
    };
}

/// Generate block read only.
macro_rules! pmbus_block_read_only {
    ($name:ident, $cmd:ident) => {
        pub async fn $name(&mut self, addr: u8) -> Result<Vec<u8, 32>, BUS::Error> {
            self.block_read_cmd(addr, CommandCode::$cmd).await
        }
    };
}

// ---------------------------------------------------------------------------
// PmbusAdaptor
// ---------------------------------------------------------------------------

/// A PMBus protocol adapter that wraps an `SmbusAdaptor`.
///
/// Provides typed methods for every standard PMBus 1.4 command. The device
/// address is passed per-call (not stored), matching the smbus-adapter pattern.
pub struct PmbusAdaptor<BUS: I2c> {
    smbus: SmbusAdaptor<BUS>,
}

impl<BUS: I2c + 'static> PmbusAdaptor<BUS> {
    /// Create a new PMBus adapter wrapping the given SMBus adapter.
    pub fn new(smbus: SmbusAdaptor<BUS>) -> Self {
        Self { smbus }
    }

    /// Consume self and return the inner `SmbusAdaptor`.
    pub fn release(self) -> SmbusAdaptor<BUS> {
        self.smbus
    }

    /// Borrow the inner `SmbusAdaptor` mutably.
    pub fn inner(&mut self) -> &mut SmbusAdaptor<BUS> {
        &mut self.smbus
    }

    // -----------------------------------------------------------------------
    // Private helpers
    // -----------------------------------------------------------------------

    async fn send_cmd(&mut self, addr: u8, cmd: CommandCode) -> Result<(), BUS::Error> {
        self.smbus.send_byte(addr, cmd.code()).await
    }

    async fn write_cmd_byte(
        &mut self,
        addr: u8,
        cmd: CommandCode,
        data: u8,
    ) -> Result<(), BUS::Error> {
        self.smbus.write_byte(addr, cmd.code(), data).await
    }

    async fn read_cmd_byte(&mut self, addr: u8, cmd: CommandCode) -> Result<u8, BUS::Error> {
        self.smbus.read_byte(addr, cmd.code()).await
    }

    async fn write_cmd_word(
        &mut self,
        addr: u8,
        cmd: CommandCode,
        data: u16,
    ) -> Result<(), BUS::Error> {
        self.smbus.write_word(addr, cmd.code(), data).await
    }

    async fn read_cmd_word(&mut self, addr: u8, cmd: CommandCode) -> Result<u16, BUS::Error> {
        self.smbus.read_word(addr, cmd.code()).await
    }

    async fn block_write_cmd(
        &mut self,
        addr: u8,
        cmd: CommandCode,
        data: &[u8],
    ) -> Result<(), BUS::Error> {
        self.smbus.block_write(addr, cmd.code(), data).await
    }

    async fn block_read_cmd(
        &mut self,
        addr: u8,
        cmd: CommandCode,
    ) -> Result<Vec<u8, 32>, BUS::Error> {
        self.smbus.block_read(addr, cmd.code()).await
    }

    async fn block_process_call_cmd(
        &mut self,
        addr: u8,
        cmd: CommandCode,
        data: &[u8],
    ) -> Result<Vec<u8, 32>, BUS::Error> {
        self.smbus
            .block_read_process_call(addr, cmd.code(), data)
            .await
    }

    // =======================================================================
    // Send-byte commands (no data)
    // =======================================================================

    pmbus_send_byte!(clear_faults, ClearFaults);
    pmbus_send_byte!(store_default_all, StoreDefaultAll);
    pmbus_send_byte!(restore_default_all, RestoreDefaultAll);
    pmbus_send_byte!(store_user_all, StoreUserAll);
    pmbus_send_byte!(restore_user_all, RestoreUserAll);

    // =======================================================================
    // Byte read/write commands
    // =======================================================================

    pmbus_byte_rw!(set_page, get_page, Page);
    pmbus_byte_rw!(set_operation, get_operation, Operation);
    pmbus_byte_rw!(set_on_off_config, get_on_off_config, OnOffConfig);
    pmbus_byte_rw!(set_phase, get_phase, Phase);
    pmbus_byte_rw!(set_write_protect, get_write_protect, WriteProtect);
    pmbus_byte_rw!(set_power_mode, get_power_mode, PowerMode);
    pmbus_byte_rw!(set_fan_config_12, get_fan_config_12, FanConfig12);
    pmbus_byte_rw!(set_fan_config_34, get_fan_config_34, FanConfig34);

    // Fault responses (byte r/w)
    pmbus_byte_rw!(
        set_vout_ov_fault_response,
        get_vout_ov_fault_response,
        VoutOvFaultResponse
    );
    pmbus_byte_rw!(
        set_vout_uv_fault_response,
        get_vout_uv_fault_response,
        VoutUvFaultResponse
    );
    pmbus_byte_rw!(
        set_iout_oc_fault_response,
        get_iout_oc_fault_response,
        IoutOcFaultResponse
    );
    pmbus_byte_rw!(
        set_iout_oc_lv_fault_response,
        get_iout_oc_lv_fault_response,
        IoutOcLvFaultResponse
    );
    pmbus_byte_rw!(
        set_iout_uc_fault_response,
        get_iout_uc_fault_response,
        IoutUcFaultResponse
    );
    pmbus_byte_rw!(
        set_ot_fault_response,
        get_ot_fault_response,
        OtFaultResponse
    );
    pmbus_byte_rw!(
        set_ut_fault_response,
        get_ut_fault_response,
        UtFaultResponse
    );
    pmbus_byte_rw!(
        set_vin_ov_fault_response,
        get_vin_ov_fault_response,
        VinOvFaultResponse
    );
    pmbus_byte_rw!(
        set_vin_uv_fault_response,
        get_vin_uv_fault_response,
        VinUvFaultResponse
    );
    pmbus_byte_rw!(
        set_iin_oc_fault_response,
        get_iin_oc_fault_response,
        IinOcFaultResponse
    );
    pmbus_byte_rw!(
        set_ton_max_fault_response,
        get_ton_max_fault_response,
        TonMaxFaultResponse
    );
    pmbus_byte_rw!(
        set_pout_op_fault_response,
        get_pout_op_fault_response,
        PoutOpFaultResponse
    );

    // Write-byte only
    pmbus_write_byte_only!(store_default_code, StoreDefaultCode);
    pmbus_write_byte_only!(restore_default_code, RestoreDefaultCode);
    pmbus_write_byte_only!(store_user_code, StoreUserCode);
    pmbus_write_byte_only!(restore_user_code, RestoreUserCode);

    // Read-byte only
    pmbus_read_byte_only!(get_capability, Capability);
    pmbus_read_byte_only!(get_pmbus_revision, PmbusRevision);
    pmbus_read_byte_only!(get_mfr_pin_accuracy, MfrPinAccuracy);

    // =======================================================================
    // Word read/write commands
    // =======================================================================

    // Output voltage
    pmbus_word_rw!(set_vout_command, get_vout_command, VoutCommand);
    pmbus_word_rw!(set_vout_trim, get_vout_trim, VoutTrim);
    pmbus_word_rw!(set_vout_cal_offset, get_vout_cal_offset, VoutCalOffset);
    pmbus_word_rw!(set_vout_max, get_vout_max, VoutMax);
    pmbus_word_rw!(set_vout_margin_high, get_vout_margin_high, VoutMarginHigh);
    pmbus_word_rw!(set_vout_margin_low, get_vout_margin_low, VoutMarginLow);
    pmbus_word_rw!(
        set_vout_transition_rate,
        get_vout_transition_rate,
        VoutTransitionRate
    );
    pmbus_word_rw!(set_vout_droop, get_vout_droop, VoutDroop);
    pmbus_word_rw!(set_vout_scale_loop, get_vout_scale_loop, VoutScaleLoop);
    pmbus_word_rw!(
        set_vout_scale_monitor,
        get_vout_scale_monitor,
        VoutScaleMonitor
    );
    pmbus_word_rw!(set_vout_min, get_vout_min, VoutMin);

    // Power / switching
    pmbus_word_rw!(set_pout_max, get_pout_max, PoutMax);
    pmbus_word_rw!(set_max_duty, get_max_duty, MaxDuty);
    pmbus_word_rw!(set_frequency_switch, get_frequency_switch, FrequencySwitch);
    pmbus_word_rw!(set_vin_on, get_vin_on, VinOn);
    pmbus_word_rw!(set_vin_off, get_vin_off, VinOff);
    pmbus_word_rw!(set_interleave, get_interleave, Interleave);
    pmbus_word_rw!(set_iout_cal_gain, get_iout_cal_gain, IoutCalGain);
    pmbus_word_rw!(set_iout_cal_offset, get_iout_cal_offset, IoutCalOffset);

    // Fan commands
    pmbus_word_rw!(set_fan_command_1, get_fan_command_1, FanCommand1);
    pmbus_word_rw!(set_fan_command_2, get_fan_command_2, FanCommand2);
    pmbus_word_rw!(set_fan_command_3, get_fan_command_3, FanCommand3);
    pmbus_word_rw!(set_fan_command_4, get_fan_command_4, FanCommand4);

    // Fault/warn limits (word r/w)
    pmbus_word_rw!(
        set_vout_ov_fault_limit,
        get_vout_ov_fault_limit,
        VoutOvFaultLimit
    );
    pmbus_word_rw!(
        set_vout_ov_warn_limit,
        get_vout_ov_warn_limit,
        VoutOvWarnLimit
    );
    pmbus_word_rw!(
        set_vout_uv_warn_limit,
        get_vout_uv_warn_limit,
        VoutUvWarnLimit
    );
    pmbus_word_rw!(
        set_vout_uv_fault_limit,
        get_vout_uv_fault_limit,
        VoutUvFaultLimit
    );
    pmbus_word_rw!(
        set_iout_oc_fault_limit,
        get_iout_oc_fault_limit,
        IoutOcFaultLimit
    );
    pmbus_word_rw!(
        set_iout_oc_lv_fault_limit,
        get_iout_oc_lv_fault_limit,
        IoutOcLvFaultLimit
    );
    pmbus_word_rw!(
        set_iout_oc_warn_limit,
        get_iout_oc_warn_limit,
        IoutOcWarnLimit
    );
    pmbus_word_rw!(
        set_iout_uc_fault_limit,
        get_iout_uc_fault_limit,
        IoutUcFaultLimit
    );
    pmbus_word_rw!(set_ot_fault_limit, get_ot_fault_limit, OtFaultLimit);
    pmbus_word_rw!(set_ot_warn_limit, get_ot_warn_limit, OtWarnLimit);
    pmbus_word_rw!(set_ut_warn_limit, get_ut_warn_limit, UtWarnLimit);
    pmbus_word_rw!(set_ut_fault_limit, get_ut_fault_limit, UtFaultLimit);
    pmbus_word_rw!(
        set_vin_ov_fault_limit,
        get_vin_ov_fault_limit,
        VinOvFaultLimit
    );
    pmbus_word_rw!(set_vin_ov_warn_limit, get_vin_ov_warn_limit, VinOvWarnLimit);
    pmbus_word_rw!(set_vin_uv_warn_limit, get_vin_uv_warn_limit, VinUvWarnLimit);
    pmbus_word_rw!(
        set_vin_uv_fault_limit,
        get_vin_uv_fault_limit,
        VinUvFaultLimit
    );
    pmbus_word_rw!(
        set_iin_oc_fault_limit,
        get_iin_oc_fault_limit,
        IinOcFaultLimit
    );
    pmbus_word_rw!(set_iin_oc_warn_limit, get_iin_oc_warn_limit, IinOcWarnLimit);
    pmbus_word_rw!(set_power_good_on, get_power_good_on, PowerGoodOn);
    pmbus_word_rw!(set_power_good_off, get_power_good_off, PowerGoodOff);
    pmbus_word_rw!(set_ton_delay, get_ton_delay, TonDelay);
    pmbus_word_rw!(set_ton_rise, get_ton_rise, TonRise);
    pmbus_word_rw!(
        set_ton_max_fault_limit,
        get_ton_max_fault_limit,
        TonMaxFaultLimit
    );
    pmbus_word_rw!(set_toff_delay, get_toff_delay, ToffDelay);
    pmbus_word_rw!(set_toff_fall, get_toff_fall, ToffFall);
    pmbus_word_rw!(
        set_toff_max_warn_limit,
        get_toff_max_warn_limit,
        ToffMaxWarnLimit
    );
    pmbus_word_rw!(
        set_pout_op_fault_limit,
        get_pout_op_fault_limit,
        PoutOpFaultLimit
    );
    pmbus_word_rw!(
        set_pout_op_warn_limit,
        get_pout_op_warn_limit,
        PoutOpWarnLimit
    );
    pmbus_word_rw!(set_pin_op_warn_limit, get_pin_op_warn_limit, PinOpWarnLimit);

    // Zone / KWH config
    pmbus_word_rw!(set_zone_config, get_zone_config, ZoneConfig);
    pmbus_word_rw!(set_zone_active, get_zone_active, ZoneActive);
    pmbus_word_rw!(set_read_kwh_config, get_read_kwh_config, ReadKwhConfig);

    // MFR telemetry limits (word r/w)
    pmbus_word_rw!(set_mfr_vin_min, get_mfr_vin_min, MfrVinMin);
    pmbus_word_rw!(set_mfr_vin_max, get_mfr_vin_max, MfrVinMax);
    pmbus_word_rw!(set_mfr_iin_max, get_mfr_iin_max, MfrIinMax);
    pmbus_word_rw!(set_mfr_pin_max, get_mfr_pin_max, MfrPinMax);
    pmbus_word_rw!(set_mfr_vout_min, get_mfr_vout_min, MfrVoutMin);
    pmbus_word_rw!(set_mfr_vout_max, get_mfr_vout_max, MfrVoutMax);
    pmbus_word_rw!(set_mfr_iout_max, get_mfr_iout_max, MfrIoutMax);
    pmbus_word_rw!(set_mfr_pout_max, get_mfr_pout_max, MfrPoutMax);
    pmbus_word_rw!(set_mfr_tambient_max, get_mfr_tambient_max, MfrTambientMax);
    pmbus_word_rw!(set_mfr_tambient_min, get_mfr_tambient_min, MfrTambientMin);
    pmbus_word_rw!(set_mfr_max_temp_1, get_mfr_max_temp_1, MfrMaxTemp1);
    pmbus_word_rw!(set_mfr_max_temp_2, get_mfr_max_temp_2, MfrMaxTemp2);
    pmbus_word_rw!(set_mfr_max_temp_3, get_mfr_max_temp_3, MfrMaxTemp3);

    // =======================================================================
    // Read-word only (sensor telemetry)
    // =======================================================================

    pmbus_read_word_only!(read_vin, ReadVin);
    pmbus_read_word_only!(read_iin, ReadIin);
    pmbus_read_word_only!(read_vcap, ReadVcap);
    pmbus_read_word_only!(read_vout, ReadVout);
    pmbus_read_word_only!(read_iout, ReadIout);
    pmbus_read_word_only!(read_temperature_1, ReadTemperature1);
    pmbus_read_word_only!(read_temperature_2, ReadTemperature2);
    pmbus_read_word_only!(read_temperature_3, ReadTemperature3);
    pmbus_read_word_only!(read_fan_speed_1, ReadFanSpeed1);
    pmbus_read_word_only!(read_fan_speed_2, ReadFanSpeed2);
    pmbus_read_word_only!(read_fan_speed_3, ReadFanSpeed3);
    pmbus_read_word_only!(read_fan_speed_4, ReadFanSpeed4);
    pmbus_read_word_only!(read_duty_cycle, ReadDutyCycle);
    pmbus_read_word_only!(read_frequency, ReadFrequency);
    pmbus_read_word_only!(read_pout, ReadPout);
    pmbus_read_word_only!(read_pin, ReadPin);

    // =======================================================================
    // Block read/write commands
    // =======================================================================

    pmbus_block_rw!(set_mfr_id, get_mfr_id, MfrId);
    pmbus_block_rw!(set_mfr_model, get_mfr_model, MfrModel);
    pmbus_block_rw!(set_mfr_revision, get_mfr_revision, MfrRevision);
    pmbus_block_rw!(set_mfr_location, get_mfr_location, MfrLocation);
    pmbus_block_rw!(set_mfr_date, get_mfr_date, MfrDate);
    pmbus_block_rw!(set_mfr_serial, get_mfr_serial, MfrSerial);
    pmbus_block_read_only!(get_app_profile_support, AppProfileSupport);
    pmbus_block_read_only!(get_ic_device_id, IcDeviceId);
    pmbus_block_read_only!(get_ic_device_rev, IcDeviceRev);
    pmbus_block_read_only!(get_mfr_efficiency_ll, MfrEfficiencyLl);
    pmbus_block_read_only!(get_mfr_efficiency_hl, MfrEfficiencyHl);
    pmbus_block_read_only!(read_ein, ReadEin);
    pmbus_block_read_only!(read_eout, ReadEout);

    // =======================================================================
    // User data — indexed block read/write
    // =======================================================================

    /// Write user data block at the given index (0-15).
    pub async fn set_user_data(
        &mut self,
        addr: u8,
        index: u8,
        data: &[u8],
    ) -> Result<(), BUS::Error> {
        let code = CommandCode::UserData00.code() + (index & 0x0F);
        self.smbus.block_write(addr, code, data).await
    }

    /// Read user data block at the given index (0-15).
    pub async fn get_user_data(&mut self, addr: u8, index: u8) -> Result<Vec<u8, 32>, BUS::Error> {
        let code = CommandCode::UserData00.code() + (index & 0x0F);
        self.smbus.block_read(addr, code).await
    }

    // =======================================================================
    // Status registers — typed accessors
    // =======================================================================

    /// Read STATUS_BYTE (0x78).
    pub async fn get_status_byte(&mut self, addr: u8) -> Result<StatusByte, BUS::Error> {
        let raw = self.read_cmd_byte(addr, CommandCode::StatusByte).await?;
        Ok(StatusByte::from_raw(raw))
    }

    /// Write STATUS_BYTE to clear bits (0x78).
    pub async fn set_status_byte(
        &mut self,
        addr: u8,
        status: StatusByte,
    ) -> Result<(), BUS::Error> {
        self.write_cmd_byte(addr, CommandCode::StatusByte, status.bits())
            .await
    }

    /// Read STATUS_WORD (0x79).
    pub async fn get_status_word(&mut self, addr: u8) -> Result<StatusWord, BUS::Error> {
        let raw = self.read_cmd_word(addr, CommandCode::StatusWord).await?;
        Ok(StatusWord::from_raw(raw))
    }

    /// Write STATUS_WORD to clear bits (0x79).
    pub async fn set_status_word(
        &mut self,
        addr: u8,
        status: StatusWord,
    ) -> Result<(), BUS::Error> {
        self.write_cmd_word(addr, CommandCode::StatusWord, status.bits())
            .await
    }

    /// Read STATUS_VOUT (0x7A).
    pub async fn get_status_vout(&mut self, addr: u8) -> Result<StatusVout, BUS::Error> {
        let raw = self.read_cmd_byte(addr, CommandCode::StatusVout).await?;
        Ok(StatusVout::from_raw(raw))
    }

    /// Write STATUS_VOUT to clear bits (0x7A).
    pub async fn set_status_vout(
        &mut self,
        addr: u8,
        status: StatusVout,
    ) -> Result<(), BUS::Error> {
        self.write_cmd_byte(addr, CommandCode::StatusVout, status.bits())
            .await
    }

    /// Read STATUS_IOUT (0x7B).
    pub async fn get_status_iout(&mut self, addr: u8) -> Result<StatusIout, BUS::Error> {
        let raw = self.read_cmd_byte(addr, CommandCode::StatusIout).await?;
        Ok(StatusIout::from_raw(raw))
    }

    /// Write STATUS_IOUT to clear bits (0x7B).
    pub async fn set_status_iout(
        &mut self,
        addr: u8,
        status: StatusIout,
    ) -> Result<(), BUS::Error> {
        self.write_cmd_byte(addr, CommandCode::StatusIout, status.bits())
            .await
    }

    /// Read STATUS_INPUT (0x7C).
    pub async fn get_status_input(&mut self, addr: u8) -> Result<StatusInput, BUS::Error> {
        let raw = self.read_cmd_byte(addr, CommandCode::StatusInput).await?;
        Ok(StatusInput::from_raw(raw))
    }

    /// Write STATUS_INPUT to clear bits (0x7C).
    pub async fn set_status_input(
        &mut self,
        addr: u8,
        status: StatusInput,
    ) -> Result<(), BUS::Error> {
        self.write_cmd_byte(addr, CommandCode::StatusInput, status.bits())
            .await
    }

    /// Read STATUS_TEMPERATURE (0x7D).
    pub async fn get_status_temperature(
        &mut self,
        addr: u8,
    ) -> Result<StatusTemperature, BUS::Error> {
        let raw = self
            .read_cmd_byte(addr, CommandCode::StatusTemperature)
            .await?;
        Ok(StatusTemperature::from_raw(raw))
    }

    /// Write STATUS_TEMPERATURE to clear bits (0x7D).
    pub async fn set_status_temperature(
        &mut self,
        addr: u8,
        status: StatusTemperature,
    ) -> Result<(), BUS::Error> {
        self.write_cmd_byte(addr, CommandCode::StatusTemperature, status.bits())
            .await
    }

    /// Read STATUS_CML (0x7E).
    pub async fn get_status_cml(&mut self, addr: u8) -> Result<StatusCml, BUS::Error> {
        let raw = self.read_cmd_byte(addr, CommandCode::StatusCml).await?;
        Ok(StatusCml::from_raw(raw))
    }

    /// Write STATUS_CML to clear bits (0x7E).
    pub async fn set_status_cml(&mut self, addr: u8, status: StatusCml) -> Result<(), BUS::Error> {
        self.write_cmd_byte(addr, CommandCode::StatusCml, status.bits())
            .await
    }

    /// Read STATUS_OTHER (0x7F).
    pub async fn get_status_other(&mut self, addr: u8) -> Result<StatusOther, BUS::Error> {
        let raw = self.read_cmd_byte(addr, CommandCode::StatusOther).await?;
        Ok(StatusOther::from_raw(raw))
    }

    /// Write STATUS_OTHER to clear bits (0x7F).
    pub async fn set_status_other(
        &mut self,
        addr: u8,
        status: StatusOther,
    ) -> Result<(), BUS::Error> {
        self.write_cmd_byte(addr, CommandCode::StatusOther, status.bits())
            .await
    }

    /// Read STATUS_MFR_SPECIFIC (0x80).
    pub async fn get_status_mfr_specific(&mut self, addr: u8) -> Result<u8, BUS::Error> {
        self.read_cmd_byte(addr, CommandCode::StatusMfrSpecific)
            .await
    }

    /// Write STATUS_MFR_SPECIFIC to clear bits (0x80).
    pub async fn set_status_mfr_specific(&mut self, addr: u8, data: u8) -> Result<(), BUS::Error> {
        self.write_cmd_byte(addr, CommandCode::StatusMfrSpecific, data)
            .await
    }

    /// Read STATUS_FANS_1_2 (0x81).
    pub async fn get_status_fans_12(&mut self, addr: u8) -> Result<StatusFans12, BUS::Error> {
        let raw = self.read_cmd_byte(addr, CommandCode::StatusFans12).await?;
        Ok(StatusFans12::from_raw(raw))
    }

    /// Write STATUS_FANS_1_2 to clear bits (0x81).
    pub async fn set_status_fans_12(
        &mut self,
        addr: u8,
        status: StatusFans12,
    ) -> Result<(), BUS::Error> {
        self.write_cmd_byte(addr, CommandCode::StatusFans12, status.bits())
            .await
    }

    /// Read STATUS_FANS_3_4 (0x82).
    pub async fn get_status_fans_34(&mut self, addr: u8) -> Result<StatusFans34, BUS::Error> {
        let raw = self.read_cmd_byte(addr, CommandCode::StatusFans34).await?;
        Ok(StatusFans34::from_raw(raw))
    }

    /// Write STATUS_FANS_3_4 to clear bits (0x82).
    pub async fn set_status_fans_34(
        &mut self,
        addr: u8,
        status: StatusFans34,
    ) -> Result<(), BUS::Error> {
        self.write_cmd_byte(addr, CommandCode::StatusFans34, status.bits())
            .await
    }

    // =======================================================================
    // Special commands — manual implementations
    // =======================================================================

    /// Read VOUT_MODE (0x20) and parse into `VoutMode`.
    pub async fn get_vout_mode(&mut self, addr: u8) -> Result<VoutMode, BUS::Error> {
        let raw = self.read_cmd_byte(addr, CommandCode::VoutMode).await?;
        Ok(VoutMode::from_raw(raw))
    }

    /// Write VOUT_MODE (0x20) from a `VoutMode` value.
    pub async fn set_vout_mode(&mut self, addr: u8, mode: VoutMode) -> Result<(), BUS::Error> {
        self.write_cmd_byte(addr, CommandCode::VoutMode, mode.to_raw())
            .await
    }

    /// Read COEFFICIENTS (0x30) using block read/write process call.
    ///
    /// `query` is the 1-byte code identifying which coefficient set to read.
    pub async fn get_coefficients(
        &mut self,
        addr: u8,
        query: u8,
    ) -> Result<DirectCoefficients, PmbusError<BUS::Error>> {
        let resp = self
            .block_process_call_cmd(addr, CommandCode::Coefficients, &[query])
            .await?;
        // Response: [byte_count, m_low, m_high, b_low, b_high, r]
        if resp.len() < 6 {
            return Err(PmbusError::InvalidResponseLength);
        }
        DirectCoefficients::from_coefficients_response(&resp[1..6])
            .ok_or(PmbusError::InvalidResponseLength)
    }

    /// Execute QUERY command (0x1A) — asks the device about a command's support.
    pub async fn query(&mut self, addr: u8, command: u8) -> Result<u8, BUS::Error> {
        self.smbus
            .process_call(addr, CommandCode::Query.code(), command as u16)
            .await
            .map(|w| w as u8)
    }

    /// Read SMBALERT_MASK (0x1B) using process call.
    pub async fn get_smbalert_mask(
        &mut self,
        addr: u8,
        status_register: u8,
    ) -> Result<u8, BUS::Error> {
        self.smbus
            .process_call(
                addr,
                CommandCode::SmbalertMask.code(),
                status_register as u16,
            )
            .await
            .map(|w| w as u8)
    }

    /// Write SMBALERT_MASK (0x1B).
    pub async fn set_smbalert_mask(&mut self, addr: u8, data: u16) -> Result<(), BUS::Error> {
        self.write_cmd_word(addr, CommandCode::SmbalertMask, data)
            .await
    }

    /// Read PAGE_PLUS_READ (0x06) — reads a byte from a specific page in one transaction.
    pub async fn page_plus_read(
        &mut self,
        addr: u8,
        page: u8,
        command: u8,
    ) -> Result<Vec<u8, 32>, BUS::Error> {
        self.block_process_call_cmd(addr, CommandCode::PagePlusRead, &[page, command])
            .await
    }

    /// Write PAGE_PLUS_WRITE (0x05) — writes data to a specific page in one transaction.
    pub async fn page_plus_write(&mut self, addr: u8, data: &[u8]) -> Result<(), BUS::Error> {
        self.block_write_cmd(addr, CommandCode::PagePlusWrite, data)
            .await
    }

    /// Read KWH_IN (0x83) — 4-byte (32-bit) read via I2C write_read.
    pub async fn read_kwh_in(&mut self, addr: u8) -> Result<u32, BUS::Error> {
        let mut buf = [0u8; 4];
        self.smbus
            .write_read(addr, &[CommandCode::ReadKwhIn.code()], &mut buf)
            .await?;
        Ok(u32::from_le_bytes(buf))
    }

    /// Read KWH_OUT (0x84) — 4-byte (32-bit) read via I2C write_read.
    pub async fn read_kwh_out(&mut self, addr: u8) -> Result<u32, BUS::Error> {
        let mut buf = [0u8; 4];
        self.smbus
            .write_read(addr, &[CommandCode::ReadKwhOut.code()], &mut buf)
            .await?;
        Ok(u32::from_le_bytes(buf))
    }

    // =======================================================================
    // Raw methods for manufacturer-specific codes
    // =======================================================================

    /// Read a byte from any command code.
    pub async fn raw_read_byte(&mut self, addr: u8, code: u8) -> Result<u8, BUS::Error> {
        self.smbus.read_byte(addr, code).await
    }

    /// Write a byte to any command code.
    pub async fn raw_write_byte(&mut self, addr: u8, code: u8, data: u8) -> Result<(), BUS::Error> {
        self.smbus.write_byte(addr, code, data).await
    }

    /// Read a word from any command code.
    pub async fn raw_read_word(&mut self, addr: u8, code: u8) -> Result<u16, BUS::Error> {
        self.smbus.read_word(addr, code).await
    }

    /// Write a word to any command code.
    pub async fn raw_write_word(
        &mut self,
        addr: u8,
        code: u8,
        data: u16,
    ) -> Result<(), BUS::Error> {
        self.smbus.write_word(addr, code, data).await
    }

    /// Block read from any command code.
    pub async fn raw_block_read(&mut self, addr: u8, code: u8) -> Result<Vec<u8, 32>, BUS::Error> {
        self.smbus.block_read(addr, code).await
    }

    /// Block write to any command code.
    pub async fn raw_block_write(
        &mut self,
        addr: u8,
        code: u8,
        data: &[u8],
    ) -> Result<(), BUS::Error> {
        self.smbus.block_write(addr, code, data).await
    }

    // =======================================================================
    // Extended command protocol
    // =======================================================================

    /// Extended read byte — sends [prefix, ext_cmd] and reads 1 byte.
    pub async fn extended_read_byte(
        &mut self,
        addr: u8,
        prefix: u8,
        ext_cmd: u8,
    ) -> Result<u8, BUS::Error> {
        let mut buf = [0u8; 1];
        self.smbus
            .write_read(addr, &[prefix, ext_cmd], &mut buf)
            .await?;
        Ok(buf[0])
    }

    /// Extended write byte — sends [prefix, ext_cmd, data].
    pub async fn extended_write_byte(
        &mut self,
        addr: u8,
        prefix: u8,
        ext_cmd: u8,
        data: u8,
    ) -> Result<(), BUS::Error> {
        self.smbus.write(addr, &[prefix, ext_cmd, data]).await
    }

    /// Extended read word — sends [prefix, ext_cmd] and reads 2 bytes (LE).
    pub async fn extended_read_word(
        &mut self,
        addr: u8,
        prefix: u8,
        ext_cmd: u8,
    ) -> Result<u16, BUS::Error> {
        let mut buf = [0u8; 2];
        self.smbus
            .write_read(addr, &[prefix, ext_cmd], &mut buf)
            .await?;
        Ok(u16::from_le_bytes(buf))
    }

    /// Extended write word — sends [prefix, ext_cmd, lo, hi].
    pub async fn extended_write_word(
        &mut self,
        addr: u8,
        prefix: u8,
        ext_cmd: u8,
        data: u16,
    ) -> Result<(), BUS::Error> {
        let bytes = data.to_le_bytes();
        self.smbus
            .write(addr, &[prefix, ext_cmd, bytes[0], bytes[1]])
            .await
    }
}
