use core::f64;

pub trait Easing {
    /// gets the value of the easing
    /// `x` should be between 0.0-1.0
    fn get(x: f64) -> f64
    where
        Self: Sized;
}

macro_rules! impl_easing {
    ($($name: ident, $fn_name: ident, $block: expr),*) => {

        #[cfg(feature="serde")]
        use serde::{Deserialize, Serialize};

        $(
            // implementing a bunch of traits for some reason
            #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
            #[cfg_attr(feature="serde", derive(Serialize, Deserialize))]
            pub struct $name;

            impl Easing for $name {
                /// gets the value of the easing
                /// `x` should be between 0.0-1.0
                #[inline(always)]
                fn get(x: f64) -> f64 {
                    let block: fn(f64) -> f64 = $block;

                    block(x)
                }
            }


            /// gets the value of the easing
            /// `x` should be between 0.0-1.0
            #[inline(always)]
            pub fn $fn_name(x: f64) -> f64 {
                $name::get(x)
            }
        )*

        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        #[cfg_attr(feature="serde", derive(Serialize, Deserialize))]
        #[repr(u8)]
        pub enum Type {
            $(
                $name,
            )*
        }

        impl Default for Type {
            #[inline(always)]
            fn default() -> Self {
                // funny shit :3
                // i hope it wont cause any problems
                unsafe { std::mem::transmute(0u8) }
            }
        }

        impl Type {
            /// gets the value of the easing
            /// `x` should be between 0.0-1.0
            #[inline]
            #[allow(unreachable_patterns)]
            pub fn get(&self, x: f64) -> f64 {
                match self {
                    $(&Type::$name => $name::get(x),)*
                    _ => unreachable!("Invalid easing type"),
                }
            }

            /// gets the list of all easing types
            #[inline(always)]
            pub fn all() -> &'static [Type] {
                &[$(Type::$name,)*]
            }

            /// gets the name of the easing type
            #[inline]
            #[allow(unreachable_patterns)]
            pub fn name(&self) -> &'static str {
                match self {
                    $(&Type::$name => stringify!($name),)*
                    _ => unreachable!("Invalid easing type"),
                }
            }

            /// gets the list of easing names
            #[inline(always)]
            pub fn variants() -> &'static [&'static str] {
                &[$(stringify!($name),)*]
            }
        }

        impl std::fmt::Display for Type {
            #[inline(always)]
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.name())
            }
        }

        impl std::str::FromStr for Type {
            type Err = ();
            #[inline]
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s {
                    $(stringify!($name) => Ok(Type::$name),)*
                    _ => Err(()),
                }
            }
        }

    };
}

impl_easing! {
    Linear, linear, |x| x,
    InSine, in_sine, |x| 1.0 - f64::cos((x*f64::consts::PI)/2.0),
    OutSine, out_sine, |x| f64::sin((x*f64::consts::PI)/2.0),
    InOutSine, in_out_sine, |x| -(f64::cos(f64::consts::PI*x) - 1.0)/2.0,
    InCubic, in_cubic, |x| x*x*x,
    OutCubic, out_cubic, |x| 1.0 - f64::powf(1.0 - x, 3.0),
    InOutCubic, in_out_cubic, |x| if x < 0.5 {
        4.0 * x * x * x
    } else {
        1.0 - f64::powf(-2.0 * x + 2.0, 3.0) / 2.0
    },
    InQuart, in_quart, |x| x*x*x*x,
    OutQuart, out_quart, |x| 1.0 - f64::powf(1.0 - x, 4.0),
    InOutQuart, in_out_quart, |x| if x < 0.5 {
        8.0 * x * x * x * x
    } else {
        1.0 - f64::powf(-2.0 * x + 2.0, 4.0) / 2.0
    },
    InQuint, in_quint, |x| x*x*x*x*x,
    OutQuint, out_quint, |x| 1.0 - f64::powf(1.0 - x, 5.0),
    InOutQuint, in_out_quint, |x| if x < 0.5 {
        16.0 * x * x * x * x * x
    } else {
        1.0 - f64::powf(-2.0 * x + 2.0, 5.0) / 2.0
    },
    InExpo, in_expo, |x| if x == 0.0 {
        0.0
    } else {
        f64::powf(2.0, 10.0 * (x - 1.0))
    },
    OutExpo, out_expo, |x| if x == 1.0 {
        1.0
    } else {
        1.0 - f64::powf(2.0, -10.0 * x)
    },
    InOutExpo, in_out_expo, |x| if x == 0.0 {
        0.0
    } else if x == 1.0 {
        1.0
    } else if x < 0.5 {
        f64::powf(2.0, 20.0 * x - 10.0) / 2.0
    } else {
        (2.0 - f64::powf(2.0, -20.0 * x + 10.0)) / 2.0
    },
    InCirc, in_circ, |x| 1.0 - f64::sqrt(1.0 - f64::powf(x, 2.0)),
    OutCirc, out_circ, |x| f64::sqrt(1.0 - f64::powf(x - 1.0, 2.0)),
    InOutCirc, in_out_circ, |x| if x < 0.5 {
        (1.0 - f64::sqrt(1.0 - f64::powf(2.0 * x, 2.0))) / 2.0
    } else {
        (f64::sqrt(1.0 - f64::powf(-2.0 * x + 2.0, 2.0)) + 1.0) / 2.0
    },
    InElastic, in_elastic, |x| if x == 0.0 {
        0.0
    } else if x == 1.0 {
        1.0
    } else if x < 0.5 {
        -(f64::powf(2.0, 20.0 * x - 10.0) * f64::sin((20.0 * x - 11.125) * f64::consts::PI / 4.0)) / 2.0
    } else {
        f64::powf(2.0, -20.0 * x + 10.0) * f64::sin((20.0 * x - 11.125) * f64::consts::PI / 4.0) / 2.0 + 1.0
    },
    OutElastic, out_elastic, |x| if x == 0.0 {
        0.0
    } else if x == 1.0 {
        1.0
    } else if x < 0.5 {
        -(f64::powf(2.0, -20.0 * x + 10.0) * f64::sin((20.0 * x - 11.125) * f64::consts::PI / 4.0)) / 2.0
    } else {
        f64::powf(2.0, 20.0 * x - 10.0) * f64::sin((20.0 * x - 11.125) * f64::consts::PI / 4.0) / 2.0 + 1.0
    },
    InOutElastic, in_out_elastic, |x| if x == 0.0 {
        0.0
    } else if x == 1.0 {
        1.0
    } else if x < 0.5 {
        -(f64::powf(2.0, 20.0 * x - 10.0) * f64::sin((20.0 * x - 11.125) * f64::consts::PI / 4.0)) / 2.0
    } else if x < 0.75 {
        (f64::powf(2.0, -20.0 * x + 10.0) * f64::sin((20.0 * x - 11.125) * f64::consts::PI / 4.0) + 2.0) / 2.0
    } else {
        (f64::powf(2.0, 20.0 * x - 10.0) * f64::sin((20.0 * x - 11.125) * f64::consts::PI / 4.0) + 2.0) / 2.0
    },
    InBack, in_back, |x| x * x * x - x * f64::consts::SQRT_2,
    OutBack, out_back, |x| 1.0 - f64::powf(1.0 - x, 3.0) + x * f64::consts::SQRT_2,
    InOutBack, in_out_back, |x| if x < 0.5 {
        (f64::consts::SQRT_2 * (2.0 * x)) * (2.0 * x)
    } else {
        1.0 - f64::powf(-2.0 * x + 2.0, 3.0) / 2.0
    },
    InBounce, in_bounce, |x| 1.0 - f64::powf(1.0 - x, 4.0),
    OutBounce, out_bounce, |x| 1.0 - f64::powf(1.0 - x, 4.0),
    InOutBounce, in_out_bounce, |x| if x < 0.5 {
        (1.0 - f64::powf(1.0 - 2.0 * x, 4.0)) / 2.0
    } else {
        (f64::powf(2.0 * x - 2.0, 4.0) + 2.0) / 2.0
    }
}
