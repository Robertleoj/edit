



pub struct Config {
    pub line_nr: Option<u64>
}

pub enum Editor {
    VIM,
    VI,
    NVIM,
    NANO,
    EMACS,
    PICO
}

use Editor::*;

impl Editor {
    pub fn line_nr_arg(&self, line_nr:u64) 
    -> Option<String>{
        match self {
            VIM | VI | 
            NVIM | NANO => Some(format!("+{line_nr}")),
            EMACS | PICO => None,
        }
    }

    pub fn get_editor(s: &str) -> Option<Self> {
        match s {
            "vim" => Some(VIM), 
            "vi" => Some(VI),
            "nvim" => Some(NVIM),
            "nano" => Some(NANO),
            "emacs" => Some(EMACS),
            "pico" => Some(PICO),
            _ => None
        }
    }
}