#![allow(non_camel_case_types)]
use std::mem;

// minimal SDL
use libc::*;

pub const SDL_INIT_AUDIO: uint32_t = 0x00000010;
pub const SDL_INIT_VIDEO: uint32_t = 0x00000020;

pub const SDL_SWSURFACE: uint32_t = 0x00000000;
pub const SDL_HWSURFACE: uint32_t = 0x00000001;
pub const SDL_DOUBLEBUF: uint32_t = 0x40000000;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub enum SDL_EventType {
    SDL_NOEVENT = 0,
    SDL_ACTIVEEVENT,
    SDL_KEYDOWN,
    SDL_KEYUP,
    SDL_MOUSEMOTION,
    SDL_MOUSEBUTTONDOWN,
    SDL_MOUSEBUTTONUP,
    SDL_JOYAXISMOTION,
    SDL_JOYBALLMOTION,
    SDL_JOYHATMOTION,
    SDL_JOYBUTTONDOWN,
    SDL_JOYBUTTONUP,
    SDL_QUIT,
    SDL_SYSWMEVENT,
    SDL_EVENT_RESERVEDA,
    SDL_EVENT_RESERVEDB,
    SDL_VIDEORESIZE,
    SDL_VIDEOEXPOSE,
    SDL_EVENT_RESERVED2,
    SDL_EVENT_RESERVED3,
    SDL_EVENT_RESERVED4,
    SDL_EVENT_RESERVED5,
    SDL_EVENT_RESERVED6,
    SDL_EVENT_RESERVED7,
    SDL_USEREVENT = 24,
    SDL_NUMEVENTS = 32,
}

#[repr(C)]
pub struct SDL_Rect {
    pub x: i16,
    pub y: i16,
    pub w: u16,
    pub h: u16,
}

#[repr(C)]
pub struct SDL_Surface {
    pub flags: uint32_t,
    pub format: *mut c_void,
    pub w: c_int,
    pub h: c_int,
    pub pitch: uint16_t,
    pub pixels: *mut c_void,
    pub offset: c_int,
    pub hwdata: *mut c_void,
    pub clip_rect: SDL_Rect,
    pub unused1: uint32_t,
    pub locked: uint32_t,
    pub map: *mut c_void,
    pub format_version: c_uint,
    pub refcount: c_int,
}
#[repr(C)]
pub struct SDL_Event {
    pub data: [c_uchar; 24],
}

impl SDL_Event {
    pub fn event_type(&self) -> SDL_EventType {
        unsafe { *self.event_type_raw() }
    }
    fn event_type_raw(&self) -> *const SDL_EventType {
        unsafe { mem::transmute_copy(&self) }
    }
}

pub const MIX_INIT_FLAC: c_int = 0x00000001;
pub const MIX_INIT_MOD: c_int = 0x00000002;
pub const MIX_INIT_MP3: c_int = 0x00000004;
pub const MIX_INIT_OGG: c_int = 0x00000008;
pub const MIX_DEFAULT_FORMAT: uint16_t = 0x8010;

#[repr(C)]
pub struct Mix_Music {
    pub buf: *mut c_void,
}

#[link(name = "SDL_mixer")]
extern "C" {
    pub fn Mix_Init(flags: c_int) -> c_int;
    pub fn Mix_OpenAudio(
        frequency: c_int,
        format: uint16_t,
        channels: c_int,
        chunksize: c_int,
    ) -> c_int;
    pub fn Mix_LoadMUS(file: *const c_char) -> *mut Mix_Music;
    pub fn Mix_PlayMusic(music: *mut Mix_Music, loops: c_int) -> c_int;
    pub fn Mix_HaltMusic() -> c_int;
    pub fn Mix_FreeMusic(music: *mut Mix_Music);
    pub fn Mix_CloseAudio();
}
#[cfg(all(target_arch = "arm", target_os = "linux"))]
#[link(name = "mikmod")]
extern "C" {}

#[link(name = "SDL")]
extern "C" {
    pub fn SDL_Init(flags: uint32_t) -> c_int;
    pub fn SDL_Quit();
    pub fn SDL_SetVideoMode(
        width: c_int,
        height: c_int,
        bpp: c_int,
        flags: uint32_t,
    ) -> *mut SDL_Surface;
    pub fn SDL_Flip(screen: *mut SDL_Surface) -> c_int;
    pub fn SDL_Delay(flags: uint32_t);
    pub fn SDL_PollEvent(event: *mut SDL_Event) -> c_int;
}
