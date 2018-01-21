fn get_time(base: (i32, i32), target: ((i32, i32), i32)) -> i32 {
    ((base.0 - (target.0).0).pow(2) + (base.1 + (target.0).1).pow(2)) / target.1
}

fn harvester_rescue(data: Data) -> &'static str {
    if get_time(data.harvester, data.worm) > get_time(data.harvester, data.carryall) {
        "The spice must flow! Rescue the harvester!"
    } else { "Damn the spice! I'll rescue the miners!" }
}