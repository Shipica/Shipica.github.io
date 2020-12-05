macro_rules! params {
    ($($t: tt),+) => {
        use ::paste::paste;
        use ::std::any::TypeId;

        #[allow(non_camel_case_types)]
        #[derive(Clone, Debug, PartialEq)]
        pub enum Param {
            $($t($t)),+,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum ParamType {
            $($t),+,
            Unknown
        }

        impl ::std::fmt::Display for ParamType {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "{}", match self {
                    $(
                        ParamType::$t => stringify!($t)
                    ),+,
                    ParamType::Unknown => "Unknown"
                })
            }
        }

        impl ParamType {
            // Alexander:
            // TODO!: may be const fn, but compiler yelling on comparison
            // Probably because PartialEq is a trait and const fn and traits don't like each other
            pub fn type_for<T: ?Sized + 'static>() -> ParamType {
                let requested = TypeId::of::<T>();

                $(
                    // this one
                    if TypeId::of::<$t>() == requested {
                        return ParamType::$t
                    }
                )+

                return ParamType::Unknown;
            }

            pub const fn is_unknown(&self) -> bool {
                matches!(self, ParamType::Unknown)
            }
        }

        $(
            impl From<$t> for Param {
                fn from(param: $t) -> Param {
                    Param::$t(param)
                }
            }
        )+

        impl Param {
            pub fn get_type(&self) -> ParamType{
                match self {
                    $(
                        Param::$t(_) => ParamType::$t
                    ),+
                }
            }

            $(
                paste! {
                    pub fn [<from_ $t>](p: $t) -> Param {
                        Param::$t(p)
                    }

                    pub fn [<into_ $t>](self) -> Option<$t> {
                        match self {
                            Param::$t(p) => Some(p),
                            _ => None
                        }
                    }

                    pub fn [<is_ $t>](&self) -> bool {
                        matches!(self, Param::$t(_))
                    }
                }
            )+
        }
    }
}

params!(i64, f64, f32);

#[test]
fn check() {
    assert!(Param::i64(0).is_i64());
}
