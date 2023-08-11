use winapi::um::winuser;

pub fn is_key_down(key: i32) -> bool {
    let state = unsafe { winuser::GetAsyncKeyState(key) };
    (1 << 15) & state != 0
}