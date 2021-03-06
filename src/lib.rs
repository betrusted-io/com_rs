#![no_std]

/// COM link states. These constants encode the commands sent from the SoC to the EC.

#[derive(Copy, Clone, Debug)]
pub struct ComSpec {
    /// the "verb" specifying the command
    pub verb: u16,
    /// number of payload words expected -- not counting the verb or dummies for read
    pub w_words: u16,
    /// number of words to be returned to host; host must generate dummy exchanges equal to this to empty the FIFO
    pub r_words: u16,
    /// specifies if this "verb" is a response code, or a verb
    pub response: bool,
}

#[non_exhaustive]
pub struct ComState;

impl ComState {
    // wifi-related
    pub const SSID_CHECK: ComSpec            = ComSpec{verb: 0x2000, w_words: 0,     r_words: 1     ,response: false};
    pub const SSID_FETCH: ComSpec            = ComSpec{verb: 0x2100, w_words: 0,     r_words: 16*6  ,response: false};
    pub const WFX_PDS_LINE_SET: ComSpec      = ComSpec{verb: 0x2200, w_words: 129,   r_words: 0     ,response: false}; // 1 length + 128 buffer. length is in *bytes* not words. Sends one line of a PDS.
    pub const WFX_RXSTAT_GET: ComSpec        = ComSpec{verb: 0x2201, w_words: 0,     r_words: 376/2 ,response: false};
    pub const WFX_FW_REV_GET: ComSpec        = ComSpec{verb: 0x2202, w_words: 0,     r_words: 3     ,response: false};
    pub const WF200_RESET: ComSpec           = ComSpec{verb: 0x2203, w_words: 1,     r_words: 0     ,response: false};
    pub const SSID_SCAN_ON: ComSpec          = ComSpec{verb: 0x2204, w_words: 0,     r_words: 0     ,response: false};
    pub const SSID_SCAN_OFF: ComSpec         = ComSpec{verb: 0x2205, w_words: 0,     r_words: 0     ,response: false};

    // flash commands
    pub const FLASH_WAITACK: ComSpec         = ComSpec{verb: 0x3000, w_words: 0,     r_words: 1     ,response: false};
    pub const FLASH_ACK: ComSpec             = ComSpec{verb: 0x3CC3, w_words: 0,     r_words: 0     ,response: true};
    pub const FLASH_ERASE: ComSpec           = ComSpec{verb: 0x3200, w_words: 4,     r_words: 0     ,response: false};
    pub const FLASH_PP: ComSpec              = ComSpec{verb: 0x3300, w_words: 130,   r_words: 0     ,response: false};
    pub const FLASH_LOCK: ComSpec            = ComSpec{verb: 0x3400, w_words: 0,     r_words: 0     ,response: false}; // lock activity for updates
    pub const FLASH_UNLOCK: ComSpec          = ComSpec{verb: 0x3434, w_words: 0,     r_words: 0     ,response: false}; // unlock activity for updates

    // system meta commands
    pub const LOOP_TEST: ComSpec             = ComSpec{verb: 0x4000, w_words: 0,     r_words: 1     ,response: false};
    pub const EC_GIT_REV: ComSpec            = ComSpec{verb: 0x4001, w_words: 0,     r_words: 3     ,response: false};

    // charger "dangerous" commands
    pub const CHG_START: ComSpec             = ComSpec{verb: 0x5A00, w_words: 0,     r_words: 0     ,response: false};
    pub const CHG_BOOST_ON: ComSpec          = ComSpec{verb: 0x5ABB, w_words: 0,     r_words: 0     ,response: false};
    pub const CHG_BOOST_OFF: ComSpec         = ComSpec{verb: 0x5AFE, w_words: 0,     r_words: 0     ,response: false};

    // backlight: this is an odd bird: back light is set by directly using the lower 10 bits to code the backlight level
    pub const BL_START: ComSpec              = ComSpec{verb: 0x6800, w_words: 0,     r_words: 0     ,response: false};
    pub const BL_END: ComSpec                = ComSpec{verb: 0x6BFF, w_words: 0,     r_words: 0     ,response: false};

    // gas gauge commands
    pub const GAS_GAUGE: ComSpec             = ComSpec{verb: 0x7000, w_words: 0,     r_words: 4     ,response: false};
    pub const GG_FACTORY_CAPACITY: ComSpec   = ComSpec{verb: 0x7676, w_words: 1,     r_words: 1     ,response: false};
    pub const GG_GET_CAPACITY: ComSpec       = ComSpec{verb: 0x7600, w_words: 0,     r_words: 1     ,response: false};
    pub const GG_DEBUG: ComSpec              = ComSpec{verb: 0x7200, w_words: 0,     r_words: 1     ,response: false};
    pub const GG_SOC: ComSpec                = ComSpec{verb: 0x7300, w_words: 0,     r_words: 1     ,response: false};
    pub const GG_REMAINING: ComSpec          = ComSpec{verb: 0x7400, w_words: 0,     r_words: 1     ,response: false};
    pub const GG_FULL_CAPACITY: ComSpec      = ComSpec{verb: 0x7402, w_words: 0,     r_words: 1,     response: false};

    // charger status - non-dangerous charger commands
    pub const STAT: ComSpec                  = ComSpec{verb: 0x8000, w_words: 0,     r_words: 16    ,response: false};
    pub const STAT_RETURN: ComSpec           = ComSpec{verb: 0x8001, w_words: 0,     r_words: 0     ,response: true};

    // power state commands
    pub const POWER_OFF: ComSpec             = ComSpec{verb: 0x9000, w_words: 0,     r_words: 1     ,response: false};
    pub const POWER_CHARGER_STATE: ComSpec   = ComSpec{verb: 0x9100, w_words: 0,     r_words: 1     ,response: false};
    pub const POWER_SHIPMODE: ComSpec        = ComSpec{verb: 0x9200, w_words: 0,     r_words: 0     ,response: false};

    // gyro commands
    pub const GYRO_UPDATE: ComSpec           = ComSpec{verb: 0xA000, w_words: 0,     r_words: 0     ,response: false};
    pub const GYRO_READ: ComSpec             = ComSpec{verb: 0xA100, w_words: 0,     r_words: 4     ,response: false};

    // USB CC commands
    pub const POLL_USB_CC: ComSpec           = ComSpec{verb: 0xB000, w_words: 0,     r_words: 4     ,response: false};

    // protocol overhead commands
    pub const LINK_READ: ComSpec             = ComSpec{verb: 0xF0F0, w_words: 0,     r_words: 0     ,response: false}; // dummy command to "pump" the bus to read data
    pub const LINK_SYNC: ComSpec             = ComSpec{verb: 0xFFFF, w_words: 0,     r_words: 0     ,response: false};

    // catch-all error code
    pub const ERROR: ComSpec                 = ComSpec{verb: 0xDEAD, w_words: 0,     r_words: 0     ,response: true};
}
