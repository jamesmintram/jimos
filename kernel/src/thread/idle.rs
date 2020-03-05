use scheduler;
//TODO: Incrememnt some counters - to confirm state is OK
//TODO: Also, do we have separate Address Spaces loaded? (I dont think we do!)

pub fn idle1(_param: u64) {
    let mut t = 0;
    loop {
        t += 1;
        println!("Idle1 {}", t);
        if t > 10000 {
            panic!()
        }
        scheduler::switch_to_next();
    }
}

pub fn idle2(_param: u64) {
    loop {
        println!("Idle2");
        scheduler::switch_to_next();
    }
}

pub fn idle3(_param: u64) {
    loop {
        println!("Idle3");
        scheduler::switch_to_next();
    }
}