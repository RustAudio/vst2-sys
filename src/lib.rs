use std::ffi::c_void;
use std::mem;
use std::os::raw::c_char;

pub const fn cconst(a: u8, b: u8, c: u8, d: u8) -> i32 {
    ((a as i32) << 24) | ((b as i32) << 16) | ((c as i32) << 8) | ((d as i32) << 0)
}

pub mod host_opcodes {
    pub const AUTOMATE: i32 = 0;
    pub const VERSION: i32 = 1;
    pub const CURRENT_ID: i32 = 2;
    pub const IDLE: i32 = 3;
    pub const PIN_CONNECTED: i32 = 4;
    pub const WANT_MIDI: i32 = 6;
    pub const GET_TIME: i32 = 7;
    pub const PROCESS_EVENTS: i32 = 8;
    pub const SET_TIME: i32 = 9;
    pub const TEMPO_AT: i32 = 10;
    pub const GET_NUM_AUTOMATABLE_PARAMETERS: i32 = 11;
    pub const GET_PARAMETER_QUANTIZATION: i32 = 12;
    pub const IO_CHANGED: i32 = 13;
    pub const NEED_IDLE: i32 = 14;
    pub const SIZE_WINDOW: i32 = 15;
    pub const GET_SAMPLE_RATE: i32 = 16;
    pub const GET_BLOCK_SIZE: i32 = 17;
    pub const GET_INPUT_LATENCY: i32 = 18;
    pub const GET_OUTPUT_LATENCY: i32 = 19;
    pub const GET_PREVIOUS_PLUG: i32 = 20;
    pub const GET_NEXT_PLUG: i32 = 21;
    pub const WILL_REPLACE_OR_ACCUMULATE: i32 = 22;
    pub const GET_CURRENT_PROCESS_LEVEL: i32 = 23;
    pub const GET_AUTOMATION_STATE: i32 = 24;
    pub const OFFLINE_START: i32 = 25;
    pub const OFFLINE_READ: i32 = 26;
    pub const OFFLINE_WRITE: i32 = 27;
    pub const OFFLINE_GET_CURRENT_PASS: i32 = 28;
    pub const OFFLINE_GET_CURRENT_META_PASS: i32 = 29;
    pub const SET_OUTPUT_SAMPLE_RATE: i32 = 30;
    pub const GET_SPEAKER_ARRANGEMENT: i32 = 31;
    pub const GET_VENDOR_STRING: i32 = 32;
    pub const GET_PRODUCT_STRING: i32 = 33;
    pub const GET_VENDOR_VERSION: i32 = 34;
    pub const VENDOR_SPECIFIC: i32 = 35;
    pub const SET_ICON: i32 = 36;
    pub const CAN_DO: i32 = 37;
    pub const GET_LANGUAGE: i32 = 38;
    pub const OPEN_WINDOW: i32 = 39;
    pub const CLOSE_WINDOW: i32 = 40;
    pub const GET_DIRECTORY: i32 = 41;
    pub const UPDATE_DISPLAY: i32 = 42;
    pub const BEGIN_EDIT: i32 = 43;
    pub const END_EDIT: i32 = 44;
    pub const OPEN_FILE_SELECTOR: i32 = 45;
    pub const CLOSE_FILE_SELECTOR: i32 = 46;
    pub const EDIT_FILE: i32 = 47;
    pub const GET_CHUNK_FILE: i32 = 48;
    pub const GET_INPUT_SPEAKER_ARRANGEMENT: i32 = 49;
}

pub mod effect_flags {
    pub const HAS_EDITOR: i32 = 1;
    pub const CAN_REPLACING: i32 = 1 << 4;
    pub const PROGRAM_CHUNKS: i32 = 1 << 5;
    pub const IS_SYNTH: i32 = 1 << 8;
}

pub mod effect_opcodes {
    pub const OPEN: i32 = 0;
    pub const CLOSE: i32 = 1;
    pub const SET_PROGRAM: i32 = 2;
    pub const GET_PROGRAM: i32 = 3;
    pub const GET_PROGRAM_NAME: i32 = 5;
    pub const GET_PARAM_LABEL: i32 = 6;
    pub const GET_PARAM_DISPLAY: i32 = 7;
    pub const GET_PARAM_NAME: i32 = 8;
    pub const SET_SAMPLE_RATE: i32 = 10;
    pub const SET_BLOCK_SIZE: i32 = 11;
    pub const MAINS_CHANGED: i32 = 12;
    pub const EDIT_GET_RECT: i32 = 13;
    pub const EDIT_OPEN: i32 = 14;
    pub const EDIT_CLOSE: i32 = 15;
    pub const EDIT_IDLE: i32 = 19;
    pub const EDIT_TOP: i32 = 20;
    pub const GET_CHUNK: i32 = 23;
    pub const SET_CHUNK: i32 = 24;
    pub const PROCESS_EVENTS: i32 = 25;
    pub const CAN_BE_AUTOMATED: i32 = 26;
    pub const STRING_TO_PARAMETER: i32 = 27;
    pub const GET_PLUG_CATEGORY: i32 = 35;
    pub const SET_BYPASS: i32 = 44;
    pub const GET_EFFECT_NAME: i32 = 45;
    pub const GET_VENDOR_STRING: i32 = 47;
    pub const GET_PRODUCT_STRING: i32 = 48;
    pub const GET_VENDOR_VERSION: i32 = 49;
    pub const VENDOR_SPECIFIC: i32 = 50;
    pub const CAN_DO: i32 = 51;
    pub const IDLE: i32 = 53;
    pub const GET_PARAMETER_PROPERTIES: i32 = 56;
    pub const GET_VST_VERSION: i32 = 58;
    pub const SHELL_GET_NEXT_PLUGIN: i32 = 70;
    pub const START_PROCESS: i32 = 71;
    pub const STOP_PROCESS: i32 = 72;
    pub const BEGIN_SET_PROGRAM: i32 = 67;
    pub const END_SET_PROGRAM: i32 = 68;
}

pub const MAGIC: i32 = cconst(b'V', b's', b't', b'P');
pub const LANG_ENGLISH: i32 = 1;
pub const MIDI_TYPE: i32 = 1;
pub const SYSEX_TYPE: i32 = 6;

pub mod transport {
    pub const CHANGED: i32 = 1;
    pub const PLAYING: i32 = 1 << 1;
    pub const CYCLE_ACTIVE: i32 = 1 << 2;
    pub const RECORDING: i32 = 1 << 3;
}

pub mod automation {
    pub const WRITING: i32 = 1 << 6;
    pub const READING: i32 = 1 << 7;
}

pub mod time_info_flags {
    pub const NANOS_VALID: i32 = 1 << 8;
    pub const PPQ_POS_VALID: i32 = 1 << 9;
    pub const TEMPO_VALID: i32 = 1 << 10;
    pub const BARS_VALID: i32 = 1 << 11;
    pub const CYCLE_POS_VALID: i32 = 1 << 12;
    pub const TIME_SIG_VALID: i32 = 1 << 13;
    pub const SMPTE_VALID: i32 = 1 << 14;
    pub const CLOCK_VALID: i32 = 1 << 15;
}

#[repr(C)]
pub struct MidiEvent {
    pub event_type: i32,
    pub byte_size: i32,
    pub delta_frames: i32,
    pub flags: i32,
    pub note_length: i32,
    pub note_offset: i32,
    pub midi_data: [u8; 4],
    pub detune: i8,
    pub note_off_velocity: u8,
    pub reserved_1: u8,
    pub reserved_2: u8,
}

#[repr(C)]
pub struct SysExEvent {
    pub event_type: i32,
    pub byte_size: i32,
    pub delta_frames: i32,
    pub flags: i32,
    pub dump_bytes: i32,
    pub reserved_1: *const c_void,
    pub sysex_dump: *const i8,
    pub reserved_2: *const c_void,
}

#[repr(C)]
pub struct Event {
    dump: [u8; mem::size_of::<MidiEvent>()],
}

#[repr(C)]
pub struct Events {
    num_events: i32,
    reserved: *const c_void,
    events: [*const Event; 2],
}

pub mod string_constants {
    pub const MAX_NAME_LEN: usize = 64;
    pub const MAX_LABEL_LEN: usize = 64;
    pub const MAX_SHORT_LABEL_LEN: usize = 8;
    pub const MAX_CATEG_LABEL_LEN: usize = 24;
    pub const MAX_FILE_NAME_LEN: usize = 100;
}

pub mod plug_category {
    pub const UNKNOWN: i32 = 0;
    pub const EFFECT: i32 = 1;
    pub const SYNTH: i32 = 2;
    pub const ANALYSIS: i32 = 3;
    pub const MASTERING: i32 = 4;
    pub const SPACIALIZER: i32 = 5;
    pub const ROOM_FX: i32 = 6;
    pub const SURROUND_FX: i32 = 7;
    pub const RESTORATION: i32 = 8;
    pub const OFFLINE_PROCESS: i32 = 9;
    pub const SHELL: i32 = 10;
    pub const GENERATOR: i32 = 11;
    pub const MAX_COUNT: i32 = 12;
}

#[repr(C)]
pub struct ParameterProperties {
    pub step_float: f32,
    pub small_step_float: f32,
    pub large_step_float: f32,
    pub label: [c_char; string_constants::MAX_LABEL_LEN],
    pub flags: i32,
    pub min_integer: i32,
    pub max_integer: i32,
    pub step_integer: i32,
    pub large_step_integer: i32,
    pub short_label: [c_char; string_constants::MAX_SHORT_LABEL_LEN],
    pub display_index: i16,
    pub category: i16,
    pub num_parameters_in_category: i16,
    pub reserved: i16,
    pub category_label: [c_char; string_constants::MAX_CATEG_LABEL_LEN],
    pub future: [u8; 16],
}

pub mod parameter_flags {
    pub const IS_SWITCH: i32 = 1 << 0;
    pub const USES_INTEGER_MIN_MAX: i32 = 1 << 1;
    pub const USES_FLOAT_STEP: i32 = 1 << 2;
    pub const USES_INT_STEP: i32 = 1 << 3;
    pub const SUPPORTS_DISPLAY_INDEX: i32 = 1 << 4;
    pub const SUPPORTS_DISPLAY_CATEGORY: i32 = 1 << 5;
    pub const CAN_RAMP: i32 = 1 << 6;
}

#[repr(C)]
pub struct AEffect {
    pub magic: i32,
    pub dispatcher: extern "C" fn(*mut AEffect, i32, i32, isize, *mut c_void, f32) -> isize,
    pub process: extern "C" fn(*mut AEffect, *const *const f32, *mut *mut f32, i32),
    pub set_parameter: extern "C" fn(*mut AEffect, i32, f32),
    pub get_parameter: extern "C" fn(*mut AEffect, i32) -> f32,
    pub num_programs: i32,
    pub num_params: i32,
    pub num_inputs: i32,
    pub num_outputs: i32,
    pub flags: i32,
    pub ptr_1: *mut c_void,
    pub ptr_2: *mut c_void,
    pub initial_delay: i32,
    pub empty_2: [u8; 4 + 4],
    pub unknown_float: f32,
    pub object: *mut c_void,
    pub user: *mut c_void,
    pub unique_id: i32,
    pub version: i32,
    pub process_replacing: extern "C" fn(*mut AEffect, *const *const f32, *mut *mut f32, i32),
}

#[repr(C)]
pub struct TimeInfo {
    pub sample_pos: f64,
    pub sample_rate: f64,
    pub nano_seconds: f64,
    pub ppq_pos: f64,
    pub tempo: f64,
    pub bar_start_pos: f64,
    pub cycle_start_pos: f64,
    pub cycle_end_pos: f64,
    pub time_sig_numerator: i32,
    pub time_sig_denominator: i32,
    pub smpte_offset: i32,
    pub smpte_frame_rate: i32,
    pub samples_to_next_clock: i32,
    pub flags: i32,
}

pub type HostCallbackProc = extern "C" fn(*mut AEffect, i32, i32, isize, *mut c_void, f32) -> isize;
