#[macro_export]
macro_rules! defvar {
    { $name:ident: $type:ty = $default:expr, or try $var:ident => $transform:block $(;)? } => {
        static $name: ::once_cell::sync::Lazy<$type> = ::once_cell::sync::Lazy::new(|| {
            match ::std::env::var(stringify!($name)) {
                Ok($var) => {
                    match $transform {
                        Ok(var) => var,
                        Err(err) => {
                            eprintln!(
                                "Failed to transform environment variable “{}” with value “{:?}” to type “{}” using specified transform at {}:{}.  Using default “{:?}”.",
                                stringify!(ident),
                                $var,
                                stringify!($type),
                                file!(),
                                line!(),
                                $default,
                            );
                            $default
                        }
                    }
                },
                Err(::std::env::VarError::NotPresent) => {
                    eprintln!("Environment variable “{}” is not present.  Using default “{:?}”.", stringify!($name), $default);
                    $default
                }
                Err(::std::env::VarError::NotUnicode(envar)) => {
                    eprintln!(
                        "Environment variable “{}” does not contain valid Unicode data.  Data: [ {:?} ].  Using default “{:?}”.",
                        stringify!($ident),
                        envar,
                        $default
                    );
                    $default
                }
            }
        });
    };
    // Macro recursion limits would need to be set in the calling
    // crate, which is undesirable.  Thus all syntactic sugar forms
    // expand directly to the main form to keep recursion within
    // default limits.
    { $name:ident: $type:ty = $default:expr, or try $var:ident => $transform:expr $(;)? } => {
        defvar! { $name: $type = $default, or try $var => { $transform } }
    };
    { $name:ident: $type:ty = $default:expr, or $var:ident => $transform:block $(;)? } => {
        defvar! { $name: $type = $default, or try $var => { Result::<$type, ()>::Ok($transform) } }
    };
    { $name:ident: $type:ty = $default:expr, or $var:ident => $transform:expr $(;)? } => {
        defvar! { $name: $type = $default, or try $var => { Result::<$type, ()>::Ok($transform) } }
    };
    { $name:ident: String = $default:expr $(;)? } => {
        defvar! { $name: String = $default, or try v => { Result::<String, ()>::Ok(v.clone()) } }
    };
}
