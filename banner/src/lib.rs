use std::collections::HashMap;
use std::num::ParseFloatError;

pub struct Flag {
    pub short_hand: String,
    pub long_hand: String,
    pub desc: String,
}

impl Flag {
    pub fn opt_flag(l_h: &str, desc: &str) -> Flag {
        Flag {
            short_hand: format!("-{}", l_h[0..1].to_string()),
            long_hand: format!("--{}", l_h),
            desc: desc.to_string(),
        }
    }
}

pub type Callback = fn(&str, &str) -> Result<String, ParseFloatError>;

pub struct FlagsHandler {
    pub flags: HashMap<(String, String), Callback>,
}

impl FlagsHandler {
    pub fn add_flag(&mut self, flag: (String, String), func: Callback) {
        self.flags.insert(flag, func);
    }
    pub fn exec_func(&mut self, flag: (String, String), argv: &[&str]) -> String {
        match self.flags.get(&flag).unwrap()(argv[0], argv[1]) {
            Ok(v) => v,
            Err(e) => e.to_string(),
        }
    }
}

pub fn div(a: &str, b: &str) -> Result<String, ParseFloatError> {
    Ok(
        (a.parse::<f32>()? / b.parse::<f32>()?)
        .to_string()
    )

}
pub fn rem(a: &str, b: &str) -> Result<String, ParseFloatError> {
    Ok (
        (a.parse::<f32>()? % b.parse::<f32>()?)
        .to_string()
    )
}