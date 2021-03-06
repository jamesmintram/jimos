
#[allow(dead_code)]
#[repr(packed)]
#[derive(Clone, Copy)]
pub struct TrapFrame {
    pub tf_sp:      u64,       //0
    pub tf_lr:      u64,       //8
    pub tf_elr:     u64,       //16
    pub tf_spsr:    u32,       //24
    pub tf_esr:     u32,       //28
    pub tf_x:       [u64; 30], //32    
}

pub const DEFAULT_TRAP_FRAME: TrapFrame = TrapFrame {
    tf_sp: 0,
    tf_lr: 0,
    tf_elr: 0, 
    tf_spsr: 0,
    tf_esr: 0,
    tf_x: [0;30]
};



#[allow(dead_code)]
#[repr(packed)]
#[derive(Clone, Copy, Default, Debug)]
pub struct ArchThreadBlock {
    pub id: usize,
    pub sp: usize,
}

pub const DEFAULT_ARCH_THREAD_BLOCK: ArchThreadBlock = ArchThreadBlock {
    id: 0,
    sp: 0,
};

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