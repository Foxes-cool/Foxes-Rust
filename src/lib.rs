use lazy_static::lazy_static;
use rand::Rng;
use std::collections::HashMap;
use std::fmt;
use std::ops::Deref;
use std::sync::RwLock;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Eq, Hash, PartialEq)]
enum Tags {
    #[allow(non_camel_case_types)]
    fox,
    #[allow(non_camel_case_types)]
    curious,
    #[allow(non_camel_case_types)]
    happy,
    #[allow(non_camel_case_types)]
    scary,
    #[allow(non_camel_case_types)]
    sleeping,
}

struct Tag {
    time: u64,
    count: u64,
}

pub struct Foxes {
    string: String,
}

lazy_static! {
    static ref TIMES: RwLock<HashMap<Tags, Tag>> = RwLock::new(HashMap::new());
}

macro_rules! function {
    ( $x:ident, $name:expr ) => {
        /// https://foxes.cool/docs is going to have more details about how to use this and what it
        /// does
        pub fn $x() -> Option<Foxes> {
            let start = SystemTime::now();
            let days = start.duration_since(UNIX_EPOCH).ok()?.as_secs() / 86400;
            if !TIMES.read().ok()?.get(&Tags::$x).is_some()
                || TIMES.read().ok()?.get(&Tags::$x)?.time != days
            {
                let count = ureq::get(&format!("https://foxes.cool/counts/{}", $name))
                    .call()
                    .ok()?
                    .into_string()
                    .ok()?
                    .parse::<u64>()
                    .ok()?;
                TIMES.write().ok()?.insert(
                    Tags::$x,
                    Tag {
                        time: days,
                        count,
                    },
                );
            }
            Some(Foxes {
                string: format!(
                    "https://img.foxes.cool/{}/{}.jpg",
                    $name,
                    rand::thread_rng().gen_range(0..TIMES.read().ok()?.get(&Tags::$x)?.count)
                ),
            })
        }
    };
}

impl Foxes {
    fn add_argument(&self, argument: String) -> Foxes {
        if self.string.ends_with(".jpg") {
            Foxes {
                string: format!("{}?{argument}", &self.string),
            }
        } else {
            Foxes {
                string: format!("{}&{argument}", &self.string),
            }
        }
    }

    /// https://foxes.cool/docs is going to have more details about how to use this and what it
    /// does
    pub fn width(&self, width: u64) -> Foxes {
        self.add_argument(format!("width={width}"))
    }

    /// https://foxes.cool/docs is going to have more details about how to use this and what it
    /// does
    pub fn height(&self, height: u64) -> Foxes {
        self.add_argument(format!("height={height}"))
    }

    /// https://foxes.cool/docs is going to have more details about how to use this and what it
    /// does
    pub fn aspect_ratio(&self, x: u64, y: u64) -> Foxes {
        self.add_argument(format!("aspect_ratio={x}:{y}"))
    }
}

impl Deref for Foxes {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.string
    }
}

impl From<Foxes> for String {
    fn from(item: Foxes) -> Self {
        item.string
    }
}

impl fmt::Display for Foxes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.string)
    }
}

function!(fox, "fox");
function!(curious, "curious");
function!(happy, "happy");
function!(scary, "scary");
function!(sleeping, "sleeping");
