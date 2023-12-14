use std::{env, ffi::CString, slice::from_raw_parts_mut};

use libc::c_char;
use rgnano_sample_rs::sdl::*;
fn main() {
    unsafe {
        let mut _ret = SDL_Init(SDL_INIT_VIDEO | SDL_INIT_AUDIO);

        let path = env::current_exe()
            .unwrap()
            .parent()
            .unwrap()
            .join("sample.ogg");

        //let flag = Mix_Init(MIX_INIT_FLAC | MIX_INIT_MOD | MIX_INIT_OGG);
        _ret = Mix_OpenAudio(44100, MIX_DEFAULT_FORMAT, 2, 4096);
        let path_str_c = CString::new(path.as_os_str().as_encoded_bytes()).unwrap();
        let music = Mix_LoadMUS(path_str_c.as_ptr() as *const c_char);
        _ret = Mix_PlayMusic(music, 1);
        SDL_Delay(3000);

        let screen = SDL_SetVideoMode(240, 240, 32, SDL_HWSURFACE | SDL_DOUBLEBUF);

        let buffer = from_raw_parts_mut((*screen).pixels as *mut u32, 240 * 240 * 4);
        let colors = [0xFFFF0000, 0xFF00FF00, 0xFF0000FF];

        let mut event = SDL_Event { data: [0; 24] };
        let mut index = 0;
        'main: loop {
            while SDL_PollEvent(&mut event) > 0 {
                match event.event_type() {
                    SDL_EventType::SDL_QUIT
                    | SDL_EventType::SDL_KEYUP
                    | SDL_EventType::SDL_KEYDOWN => break 'main,
                    _ => {}
                }
            }
            for y in 0..240 {
                for x in 0..240 {
                    buffer[x + y * 240] = colors[index];
                }
            }
            index = (index + 1) % colors.len();
            SDL_Flip(screen);
            SDL_Delay(50);
        }

        Mix_HaltMusic();
        Mix_FreeMusic(music);
        Mix_CloseAudio();

        SDL_Quit();
    };
}
