use wooting_analog_wrapper::*;
use std::error::Error;
use std::sync::{Arc, Mutex};
// use std::time::Instant;
use std::vec::Vec;
use crate::windows;

type OError = Box<dyn Error>;
type OResult<T> = Result<T, OError>;

pub struct WootingData {
    end: bool,
    data: Vec<WootingEntry>,
    head: i32,
    sample_size: i32,
    k1_code: i32,
    k2_code: i32,
}

#[derive(Clone, Copy)]
pub struct WootingEntry {
    pub k1: f32,
    pub k2: f32,
    pub k1_down: bool,
    pub k2_down: bool,
    // pub time: Instant,
}

impl Default for WootingEntry {
    fn default() -> Self {
        Self {
            k1: 0.0,
            k2: 0.0,
            k1_down: false,
            k2_down: false,
            // time: Instant::now(),
        }
    }
}

impl WootingData {
    pub fn end(&mut self) {
        self.end = true
    }

    pub fn get_head(&self) -> i32 {
        self.head
    }

    fn new() -> Self {
        let settings = config::Config::builder()
            .add_source(config::File::with_name("config"))
            .build()
            .unwrap();

        let sample_size = settings.get_int("sample_size").unwrap() as i32;
        let k1_code = settings.get_int("k1_code").unwrap() as i32;
        let k2_code = settings.get_int("k2_code").unwrap() as i32;

        let mut vec = Vec::new();
        vec.resize_with(sample_size as usize, Default::default);
        Self {
            end: false,
            head: 0,
            data: vec,
            sample_size, k1_code, k2_code
        }
    }

    fn append_entry(&mut self, k1: f32, k2: f32, k1_down: bool, k2_down: bool) {
        self.head += 1;
        if self.head >= self.sample_size as i32 {
            self.head = 0;
        }

        let ent = &mut self.data[self.head as usize];
        ent.k1 = k1;
        ent.k2 = k2;
        ent.k1_down = k1_down;
        ent.k2_down = k2_down;
        // ent.time = Instant::now();
    }

    pub fn copy_data(&self) -> Vec<WootingEntry> {
        self.data.clone()
    }

    pub fn borrow_data(&self) -> &Vec<WootingEntry> {
        &self.data
    }
}

fn run_thread(wdata: Arc<Mutex<WootingData>>) -> OResult<()> {
    initialise().0?;

    set_keycode_mode(KeycodeType::VirtualKey).0?;

    let (k1_code, k2_code) = {
        let lock = wdata.lock().unwrap();
        (lock.k1_code, lock.k2_code)
    };

    loop {
        let val_k1 = read_analog(k1_code as u16).0?;
        let val_k2 = read_analog(k2_code as u16).0?;
        let k1_down = windows::is_key_down(k1_code);
        let k2_down = windows::is_key_down(k2_code);

        {
            let mut lock = wdata.lock().unwrap();
            lock.append_entry(val_k1, val_k2, k1_down, k2_down);

            if lock.end == true { break; }

        }

        let one_frame_ish = std::time::Duration::from_millis(5);
        std::thread::sleep(one_frame_ish);
    }

    uninitialise().0?;
    Ok(())
}

pub fn initialise_wooting() -> OResult<Arc<Mutex<WootingData>>> {
    let dataset = WootingData::new();
    let wdata = Arc::new(Mutex::new(dataset));
    let wdata2 = Arc::clone(&wdata);

    std::thread::spawn(move || {
        run_thread(wdata).unwrap();
    });
    
    Ok(wdata2)
}