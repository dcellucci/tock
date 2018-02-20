#[allow(unused_imports)]
use kernel::common::VolatileCell;

#[allow(dead_code)]
const HIB_BASE: usize = 0x400fc000;

#[allow(dead_code)]
struct HIBRegisters {
    rtcc: VolatileCell<u32>,
    rtcm0: VolatileCell<u32>,
    _reserved0: [u32; 1],
    rtcld: VolatileCell<u32>,
    ctl: VolatileCell<u32>,
    im: VolatileCell<u32>,
    ris: VolatileCell<u32>,
    mis: VolatileCell<u32>,
    ic: VolatileCell<u32>,
    rtct: VolatileCell<u32>,
    rtcss: VolatileCell<u32>,
    io: VolatileCell<u32>,
    data: VolatileCell<u32>,
    _reserved1: [u32; 179],
    calctl: VolatileCell<u32>,
    _reserved2: [u32; 3],
    cal0: VolatileCell<u32>,
    cal1: VolatileCell<u32>,
    _reserved3: [u32; 2],
    calld0: VolatileCell<u32>,
    calld1: VolatileCell<u32>,
    _reserved4: [u32; 2],
    calm0: VolatileCell<u32>,
    calm1: VolatileCell<u32>,
    _reserved5: [u32; 10],
    lock: VolatileCell<u32>,
    _reserved6: [u32; 39],
    tpctl: VolatileCell<u32>,
    tpstat: VolatileCell<u32>,
    _reserved7: [u32; 2],
    tpio: VolatileCell<u32>,
    _reserved8: [u32; 51],
    tplog0: VolatileCell<u32>,
    tplog1: VolatileCell<u32>,
    tplog2: VolatileCell<u32>,
    tplog3: VolatileCell<u32>,
    tplog4: VolatileCell<u32>,
    tplog5: VolatileCell<u32>,
    tplog6: VolatileCell<u32>,
    tplog7: VolatileCell<u32>,
    _reserved9: [u32; 688],
    pp: VolatileCell<u32>,
    _reserved10: [u32; 1],
    cc: VolatileCell<u32>,
}
