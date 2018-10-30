
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


impl Default for TrapFrame {
    fn default() -> TrapFrame {
        TrapFrame {
            tf_sp:      0,       
            tf_lr:      0,       
            tf_elr:     0,       
            tf_spsr:    0,       
            tf_esr:     0,       
            tf_x:       [0; 30],    
        }
    }
}

// SizeOf = 32 + 30 * 8

//TODO: Static assert?