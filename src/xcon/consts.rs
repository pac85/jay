#![allow(dead_code)]

pub const XGE_EVENT: u8 = 35;

pub const INPUT_DEVICE_ALL: u16 = 0;
pub const INPUT_DEVICE_ALL_MASTER: u16 = 1;

pub const WINDOW_CLASS_INPUT_OUTPUT: u16 = 1;

pub const PROP_MODE_REPLACE: u8 = 0;

pub const ATOM_WM_CLASS: u32 = 67;
pub const ATOM_STRING: u32 = 31;

pub const EVENT_MASK_NO_EVENT: u32 = 0;
pub const EVENT_MASK_KEY_PRESS: u32 = 1;
pub const EVENT_MASK_KEY_RELEASE: u32 = 2;
pub const EVENT_MASK_BUTTON_PRESS: u32 = 4;
pub const EVENT_MASK_BUTTON_RELEASE: u32 = 8;
pub const EVENT_MASK_ENTER_WINDOW: u32 = 16;
pub const EVENT_MASK_LEAVE_WINDOW: u32 = 32;
pub const EVENT_MASK_POINTER_MOTION: u32 = 64;
pub const EVENT_MASK_POINTER_MOTION_HINT: u32 = 128;
pub const EVENT_MASK_BUTTON_1_MOTION: u32 = 256;
pub const EVENT_MASK_BUTTON_2_MOTION: u32 = 512;
pub const EVENT_MASK_BUTTON_3_MOTION: u32 = 1024;
pub const EVENT_MASK_BUTTON_4_MOTION: u32 = 2048;
pub const EVENT_MASK_BUTTON_5_MOTION: u32 = 4096;
pub const EVENT_MASK_BUTTON_MOTION: u32 = 8192;
pub const EVENT_MASK_KEYMAP_STATE: u32 = 16384;
pub const EVENT_MASK_EXPOSURE: u32 = 32768;
pub const EVENT_MASK_VISIBILITY_CHANGE: u32 = 65536;
pub const EVENT_MASK_STRUCTURE_NOTIFY: u32 = 131072;
pub const EVENT_MASK_RESIZE_REDIRECT: u32 = 262144;
pub const EVENT_MASK_SUBSTRUCTURE_NOTIFY: u32 = 524288;
pub const EVENT_MASK_SUBSTRUCTURE_REDIRECT: u32 = 1048576;
pub const EVENT_MASK_FOCUS_CHANGE: u32 = 2097152;
pub const EVENT_MASK_PROPERTY_CHANGE: u32 = 4194304;
pub const EVENT_MASK_COLOR_MAP_CHANGE: u32 = 8388608;
pub const EVENT_MASK_OWNER_GRAB_BUTTON: u32 = 16777216;

pub const XI_EVENT_MASK_DEVICE_CHANGED: u32 = 2;
pub const XI_EVENT_MASK_KEY_PRESS: u32 = 4;
pub const XI_EVENT_MASK_KEY_RELEASE: u32 = 8;
pub const XI_EVENT_MASK_BUTTON_PRESS: u32 = 16;
pub const XI_EVENT_MASK_BUTTON_RELEASE: u32 = 32;
pub const XI_EVENT_MASK_MOTION: u32 = 64;
pub const XI_EVENT_MASK_ENTER: u32 = 128;
pub const XI_EVENT_MASK_LEAVE: u32 = 256;
pub const XI_EVENT_MASK_FOCUS_IN: u32 = 512;
pub const XI_EVENT_MASK_FOCUS_OUT: u32 = 1024;
pub const XI_EVENT_MASK_HIERARCHY: u32 = 2048;
pub const XI_EVENT_MASK_PROPERTY: u32 = 4096;
pub const XI_EVENT_MASK_RAW_KEY_PRESS: u32 = 8192;
pub const XI_EVENT_MASK_RAW_KEY_RELEASE: u32 = 16384;
pub const XI_EVENT_MASK_RAW_BUTTON_PRESS: u32 = 32768;
pub const XI_EVENT_MASK_RAW_BUTTON_RELEASE: u32 = 65536;
pub const XI_EVENT_MASK_RAW_MOTION: u32 = 131072;
pub const XI_EVENT_MASK_TOUCH_BEGIN: u32 = 262144;
pub const XI_EVENT_MASK_TOUCH_UPDATE: u32 = 524288;
pub const XI_EVENT_MASK_TOUCH_END: u32 = 1048576;
pub const XI_EVENT_MASK_TOUCH_OWNERSHIP: u32 = 2097152;
pub const XI_EVENT_MASK_RAW_TOUCH_BEGIN: u32 = 4194304;
pub const XI_EVENT_MASK_RAW_TOUCH_UPDATE: u32 = 8388608;
pub const XI_EVENT_MASK_RAW_TOUCH_END: u32 = 16777216;
pub const XI_EVENT_MASK_BARRIER_HIT: u32 = 33554432;
pub const XI_EVENT_MASK_BARRIER_LEAVE: u32 = 67108864;

pub const PRESENT_EVENT_MASK_NO_EVENT: u32 = 0;
pub const PRESENT_EVENT_MASK_CONFIGURE_NOTIFY: u32 = 1;
pub const PRESENT_EVENT_MASK_COMPLETE_NOTIFY: u32 = 2;
pub const PRESENT_EVENT_MASK_IDLE_NOTIFY: u32 = 4;
pub const PRESENT_EVENT_MASK_REDIRECT_NOTIFY: u32 = 8;

pub const INPUT_DEVICE_TYPE_MASTER_POINTER: u16 = 1;
pub const INPUT_DEVICE_TYPE_MASTER_KEYBOARD: u16 = 2;
pub const INPUT_DEVICE_TYPE_SLAVE_POINTER: u16 = 3;
pub const INPUT_DEVICE_TYPE_SLAVE_KEYBOARD: u16 = 4;
pub const INPUT_DEVICE_TYPE_FLOATING_SLAVE: u16 = 5;

pub const XKB_PER_CLIENT_FLAG_DETECTABLE_AUTO_REPEAT: u32 = 1;
pub const XKB_PER_CLIENT_FLAG_GRABS_USE_XKB_STATE: u32 = 2;
pub const XKB_PER_CLIENT_FLAG_AUTO_RESET_CONTROLS: u32 = 4;
pub const XKB_PER_CLIENT_FLAG_LOOKUP_STATE_WHEN_GRABBED: u32 = 8;
pub const XKB_PER_CLIENT_FLAG_SEND_EVENT_USES_XKB_STATE: u32 = 16;

pub const INPUT_HIERARCHY_MASK_MASTER_ADDED: u32 = 1;
pub const INPUT_HIERARCHY_MASK_MASTER_REMOVED: u32 = 2;
pub const INPUT_HIERARCHY_MASK_SLAVE_ADDED: u32 = 4;
pub const INPUT_HIERARCHY_MASK_SLAVE_REMOVED: u32 = 8;
pub const INPUT_HIERARCHY_MASK_SLAVE_ATTACHED: u32 = 16;
pub const INPUT_HIERARCHY_MASK_SLAVE_DETACHED: u32 = 32;
pub const INPUT_HIERARCHY_MASK_DEVICE_ENABLED: u32 = 64;
pub const INPUT_HIERARCHY_MASK_DEVICE_DISABLED: u32 = 128;

pub const GRAB_MODE_SYNC: u8 = 0;
pub const GRAB_MODE_ASYNC: u8 = 1;

pub const GRAB_STATUS_SUCCESS: u8 = 0;
pub const GRAB_STATUS_ALREADY_GRABBED: u8 = 1;
pub const GRAB_STATUS_INVALID_TIME: u8 = 2;
pub const GRAB_STATUS_NOT_VIEWABLE: u8 = 3;
pub const GRAB_STATUS_FROZEN: u8 = 4;

pub const IMAGE_FORMAT_XY_BITMAP: u8 = 0;
pub const IMAGE_FORMAT_XY_PIXMAP: u8 = 1;
pub const IMAGE_FORMAT_Z_PIXMAP: u8 = 2;
