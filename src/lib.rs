#![no_std]

pub mod serdes;

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
    pub const SSID_FETCH: ComSpec            = ComSpec{verb: 0x2100, w_words: 0,     r_words: 16*6  ,response: false}; // legacy, not implemented in newer revs
    pub const SSID_FETCH_STR: ComSpec        = ComSpec{verb: 0x2101, w_words: 0,     r_words: 34*8  ,response: false}; // legacy, not implemented in newer revs
    pub const WFX_PDS_LINE_SET: ComSpec      = ComSpec{verb: 0x2200, w_words: 129,   r_words: 0     ,response: false}; // 1 length + 128 buffer. length is in *bytes* not words. Sends one line of a PDS.
    pub const WFX_RXSTAT_GET: ComSpec        = ComSpec{verb: 0x2201, w_words: 0,     r_words: 376/2 ,response: false};
    pub const WFX_FW_REV_GET: ComSpec        = ComSpec{verb: 0x2202, w_words: 0,     r_words: 3     ,response: false};
    pub const WF200_RESET: ComSpec           = ComSpec{verb: 0x2203, w_words: 1,     r_words: 0     ,response: false};
    pub const SSID_SCAN_ON: ComSpec          = ComSpec{verb: 0x2204, w_words: 0,     r_words: 0     ,response: false};
    pub const SSID_SCAN_OFF: ComSpec         = ComSpec{verb: 0x2205, w_words: 0,     r_words: 0     ,response: false};
    // config(2) - control - alloc_fail(2) - alloc_oversize(2) - alloc_count
    pub const WF200_DEBUG: ComSpec           = ComSpec{verb: 0x2206, w_words: 0,     r_words: 8     ,response: false};

    // WLAN_*
    // - SSID & PASS fields are sized to match requirements of the WF200 fullMAC driver API.
    //   See https://docs.silabs.com/wifi/wf200/rtos/latest/group-w-f-m-g-r-o-u-p-c-o-n-c-e-p-t-s
    // - SSID:   2 bytes length + 32 bytes data = 34 bytes --> 17 words
    // - PASS:   2 bytes length + 64 bytes data = 66 bytes --> 33 words
    // - STATUS: 2 bytes length + 64 bytes data = 66 bytes --> 33 words
    // - IPV4_CONF: serialized binary data according to serdes::Ipv4Conf -> 14 words
    pub const WLAN_ON: ComSpec               = ComSpec{verb: 0x2300, w_words: 0,     r_words: 0     ,response: false};
    pub const WLAN_OFF: ComSpec              = ComSpec{verb: 0x2301, w_words: 0,     r_words: 0     ,response: false};
    pub const WLAN_SET_SSID: ComSpec         = ComSpec{verb: 0x2302, w_words: 17,    r_words: 0     ,response: false};
    pub const WLAN_SET_PASS: ComSpec         = ComSpec{verb: 0x2303, w_words: 33,    r_words: 0     ,response: false};
    pub const WLAN_JOIN: ComSpec             = ComSpec{verb: 0x2304, w_words: 0,     r_words: 0     ,response: false};
    pub const WLAN_LEAVE: ComSpec            = ComSpec{verb: 0x2305, w_words: 0,     r_words: 0     ,response: false};
    pub const WLAN_STATUS: ComSpec           = ComSpec{verb: 0x2306, w_words: 0,     r_words: 33    ,response: false};
    pub const WLAN_GET_IPV4_CONF: ComSpec    = ComSpec{verb: 0x2307, w_words: 0,     r_words: 14    ,response: false};
    pub const WLAN_GET_ERRCOUNTS: ComSpec    = ComSpec{verb: 0x2308, w_words: 0,     r_words: 4     ,response: false};
    // binary status reports the following:
    // rssi(1), interface_status(1), ipv4_state(14), ssid(17)
    pub const WLAN_BIN_STATUS: ComSpec       = ComSpec{verb: 0x2309, w_words: 0,     r_words: 2+14+17 ,response: false};
    pub const WLAN_GET_RSSI: ComSpec         = ComSpec{verb: 0x230A, w_words: 0,     r_words: 1     ,response: false};
    // use on resume to sync up the state with the COM. Returns linkstate then dhcpstate
    pub const WLAN_SYNC_STATE: ComSpec       = ComSpec{verb: 0x230B, w_words: 0,     r_words: 2     ,response: false};

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
    pub const UPTIME: ComSpec                = ComSpec{verb: 0x4002, w_words: 0,     r_words: 4     ,response: false};
    pub const TRNG_SEED: ComSpec             = ComSpec{verb: 0x4003, w_words: 8,     r_words: 0     ,response: false};
    pub const EC_SW_TAG: ComSpec             = ComSpec{verb: 0x4004, w_words: 0,     r_words: 16    ,response: false};

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
    pub const POLL_USB_CC: ComSpec           = ComSpec{verb: 0xB000, w_words: 0,     r_words: 5     ,response: false};

    // encoded length WLAN frames
    // LSB mask of 0x7FF encodes number of *bytes* to fetch or send; in the case that an odd number of bytes are
    // required, the last byte is 0-padded. All data is packed in MSB order.
    // note: entries are not comprehensively encoded, just a few examples provided
    // The first word of a "FETCH" frame confirms the number of words to be sent. It should be equal to the LSB of the verb minus 1.
    // "SEND" frames do not encode a confirmation of words to send
    pub const NET_FRAME_FETCH_0: ComSpec     = ComSpec{verb: 0xC800, w_words: 0,     r_words: 0     ,response: false};
    pub const NET_FRAME_FETCH_1: ComSpec     = ComSpec{verb: 0xC801, w_words: 0,     r_words: 1     ,response: false};
    pub const NET_FRAME_FETCH_2: ComSpec     = ComSpec{verb: 0xC802, w_words: 0,     r_words: 2     ,response: false};
    pub const NET_FRAME_FETCH_7FF: ComSpec   = ComSpec{verb: 0xCFFF, w_words: 0,     r_words: 0x7FF ,response: false};
    pub const NET_FRAME_SEND_0: ComSpec      = ComSpec{verb: 0xC000, w_words: 0,     r_words: 0     ,response: false};
    pub const NET_FRAME_SEND_1: ComSpec      = ComSpec{verb: 0xC001, w_words: 1,     r_words: 0     ,response: false};
    pub const NET_FRAME_SEND_7FF: ComSpec    = ComSpec{verb: 0xC7FF, w_words: 0x7FF, r_words: 0     ,response: false};

    // protocol overhead commands
    // - GET_INTERRUPT: 1 word interrupt source, 1 word rx len argument *in bytes* (always returned) -> 2 words
    // - SET_INTMASK: 1 word bitmask for interrupt source. Initially 0.
    // - GET_INTMASK: 1 read word for the current interrupt bitmask
    // - ACK_INTERRUPT: 1 word for acknowledging interrupts. All bits set in the ACK will set the GET_INTERRUPT bit to 0.
    //   note that also calling a verb that handles an interrupt will implicitly acknowledge and clear the interrupt source
    pub const LINK_READ: ComSpec             = ComSpec{verb: 0xF0F0, w_words: 0,     r_words: 0     ,response: false}; // dummy command to "pump" the bus to read data
    pub const LINK_SYNC: ComSpec             = ComSpec{verb: 0xFFFF, w_words: 0,     r_words: 0     ,response: false};
    pub const LINK_GET_INTERRUPT: ComSpec    = ComSpec{verb: 0xF108, w_words: 0,     r_words: 2     ,response: false};
    pub const LINK_SET_INTMASK: ComSpec      = ComSpec{verb: 0xF109, w_words: 1,     r_words: 0     ,response: false};
    pub const LINK_GET_INTMASK: ComSpec      = ComSpec{verb: 0xF10A, w_words: 0,     r_words: 1     ,response: false};
    pub const LINK_ACK_INTERRUPT: ComSpec    = ComSpec{verb: 0xF10B, w_words: 1,     r_words: 0     ,response: false};

    // catch-all error code
    pub const ERROR: ComSpec                 = ComSpec{verb: 0xDEAD, w_words: 0,     r_words: 0     ,response: true};
}


// COM interrupt mask bits
// set when an Rx packet is ready. Argument is length of packet.
pub const INT_WLAN_RX_READY: u16      = 0b0000_0000_0000_0001;
// set when IP configuration has been updated and should be read out. Generated by the DHCP engine.
pub const INT_WLAN_IPCONF_UPDATE: u16 = 0b0000_0000_0000_0010;
// set when SSID scan has new data.
pub const INT_WLAN_SSID_UPDATE: u16   = 0b0000_0000_0000_0100;
// set when battery is critical and system is about to shut down
pub const INT_BATTERY_CRITICAL: u16   = 0b0000_0000_0000_1000;
// set if there's an error transmitting a packet
pub const INT_WLAN_TX_ERROR: u16      = 0b0000_0000_0001_0000;
// set if there's an error receiving a packet
pub const INT_WLAN_RX_ERROR: u16      = 0b0000_0000_0010_0000;
// set if there's a disconnect event happened
pub const INT_WLAN_DISCONNECT: u16    = 0b0000_0000_0100_0000;
// set when a connection attempt finishes. must read the status code for the exact result.
pub const INT_WLAN_CONNECT_EVENT: u16 = 0b0000_0000_1000_0000;
// set when SSID scan has new data.
pub const INT_WLAN_SSID_FINISHED: u16 = 0b0000_0001_0000_0000;
// reserve one code for internal error handling
pub const INT_INVALID: u16            = 0b1000_0000_0000_0000;

/// Possible link layer connection states
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(u16)]
pub enum LinkState {
    Unknown = 0,
    ResetHold = 1,
    Uninitialized = 2,
    Initializing = 3,
    Disconnected = 4,
    Connecting = 5,
    Connected = 6,
    WFXError = 7,
}
impl LinkState {
    pub fn decode_u16(state: u16) -> Self {
        match state {
            0 => LinkState::Unknown,
            1 => LinkState::ResetHold,
            2 => LinkState::Uninitialized,
            3 => LinkState::Initializing,
            4 => LinkState::Disconnected,
            5 => LinkState::Connecting,
            6 => LinkState::Connected,
            7 => LinkState::WFXError,
            _ => LinkState::Unknown,
        }
    }
}

/// DHCP Client States
///
/// Note that InitReboot and Rebooting were intentionally omitted. Also, Halted is for
/// power-up or receiving a DHCPNAK while in Renewing or Rebinding.
///
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(u16)]
pub enum DhcpState {
    Halted = 0,
    Init = 1,
    Selecting = 2,
    Requesting = 3,
    Bound = 4,
    Renewing = 5,
    Rebinding = 6,
    Invalid = 7,
}

/// Possible connection results
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(u16)]
pub enum ConnectResult {
    Success = 0,
    NoMatchingAp = 1,
    Aborted = 7,
    Timeout = 2,
    Reject = 3,
    AuthFail = 4,
    Error = 5,
    Pending = 6,
}
impl ConnectResult {
    pub fn decode_u16(state: u16) -> Self {
        match state {
            0 => ConnectResult::Success,
            1 => ConnectResult::NoMatchingAp,
            2 => ConnectResult::Timeout,
            3 => ConnectResult::Reject,
            4 => ConnectResult::AuthFail,
            5 => ConnectResult::Error,
            7 => ConnectResult::Aborted,
            _ => ConnectResult::Pending,
        }
    }
}
