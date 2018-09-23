
#[allow(dead_code)]
#[repr(packed)]
pub struct TrapFrame {
    pub tf_sp:      u64,       //0
    pub tf_lr:      u64,       //8
    pub tf_elr:     u64,       //16
    pub tf_spsr:    u32,       //24
    pub tf_esr:     u32,       //28
    pub tf_x:       [u64; 30], //32
}

// SizeOf = 32 + 30 * 8

//TODO: Static assert?