use scheduler;
//TODO: Incrememnt some counters - to confirm state is OK
//TODO: Also, do we have separate Address Spaces loaded? (I dont think we do!)

pub fn idle1(param: u64) {
    loop {
        println!("Idle1");
        scheduler::switch_to_next();
    }
}

pub fn idle2(param: u64) {
    loop {
        println!("Idle2");
        scheduler::switch_to_next();
    }
}

pub fn idle3(param: u64) {
    loop {
        println!("Idle3");
        scheduler::switch_to_next();
    }
}