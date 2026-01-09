use libc::c_int;

#[repr(C)]
pub struct EventArgs<T> {
    pub size: u8,
    pub list: *const T,
}

#[repr(C)]
pub struct OnPlayerEnterArgs {
    pub player: *const c_int,
}

//#[no_mangle]
//pub unsafe extern "C" fn OMPRS_OnPlayerEnter(args: *const EventArgs<OnPlayerEnterArgs>) {
//    let playerid = *(*(*args).list).player;
//    println!("🦀 OMPRS_OnPlayerEnter called: playerid = {}", playerid);
//}

#[no_mangle]
pub extern "C" fn OMPRS_OnPlayerEnter(playerid: c_int) {
    println!("🦀 OMPRS_OnPlayerEnter BEGIN playerid={}", playerid);
}
