#[macro_export]
/// ## How to use
///
/// ### Standard
/// Requires every argument to implement `.as_bytes()`.
/// Might also not need to pass any arguments at all
/// ```
///
/// // Example
/// let output = dmenu!("1", "2", "3");
/// ```
///
/// ---
///
/// ### Stringify
/// Same as standard but passes every argument to `stringify!()`.
/// Again, might also not need to pass any arguments at all
/// ```
///
/// // Example
/// let output = dmenu!(stringify 1, 2, 3);
/// ```
///
/// ---
///
/// ### Iter
/// Accepts an iterator.
/// Every value yielded by the iterator needs to implement `.as_bytes()`
/// ```
///
/// // Example
/// let mut a = vec!["1", "2", "3"];
/// a.push("4");
///
/// let output = dmenu!(iter a);
/// ```
///
/// ---
///
/// ### Prompt
/// Only shows a prompt with the specified prompt text
/// ```
///
/// // Example
/// let username = dmenu!(prompt "What's your name?");
/// ```
///
/// ---
///
/// ### Optional arguments
/// You can specify optional arguments to pass to dmenu by separating them with `; args`
/// ```
///
/// // Example
/// let output = dmenu!(stringify 1, 2, 3; args "-p", "Choose a number", "-l", "3");
/// ```
///
/// ---
///
/// ### Arguments only
/// Empty call, only allows to specify arguments to pass on to dmenu
/// ```
///
/// // Example
/// let output = dmenu!(args
///     "-p", "What's your name?",
///     "--nb", "#FFFFFF",
///     "--nf", "#000000"
/// );
/// ```
macro_rules! dmenu {
    (__@internal_start@__ $(args $($arg:expr)+)?) => {{
        let mut dmenu = ::std::process::Command::new("dmenu")
            $(.args([$($arg),+]))?
            .stdin(::std::process::Stdio::piped())
            .stdout(::std::process::Stdio::piped())
            .spawn()
            .unwrap();

        (
            dmenu.stdin.take().unwrap(),
            dmenu.stdout.take().unwrap()
        )
    }};

    (__@internal_end@__ $stdin:expr, $stdout:expr) => {{
        drop($stdin);

        let mut buf = String::new();
        ::std::io::Read::read_to_string(&mut $stdout, &mut buf).unwrap();
        buf.trim().to_string()
    }};

    (iter $e:expr $(;args $($arg:expr),+)?) => {{
        let (mut stdin, mut stdout) = dmenu!(__@internal_start@__ $(args $($arg)+)?);

        for x in $e {
            ::std::io::Write::write_all(&mut stdin, x.as_bytes()).unwrap();
            ::std::io::Write::write_all(&mut stdin, b"\n").unwrap();
        }

        dmenu!(__@internal_end@__ stdin, stdout)
    }};

    (stringify $($e:expr),* $(;args $($arg:expr),+)?) => {{
        let (mut stdin, mut stdout) = dmenu!(__@internal_start@__ $(args $($arg)+)?);

        $(
            ::std::io::Write::write_all(&mut stdin, stringify!($e).as_bytes()).unwrap();
            ::std::io::Write::write_all(&mut stdin, b"\n").unwrap();
        )*

        dmenu!(__@internal_end@__ stdin, stdout)

        // Copypasted because of unfixable recursion limit
    }};

    (args $($arg:expr),+) => {{
        let (mut stdin, mut stdout) = dmenu!(__@internal_start@__ args $($arg)+);

        dmenu!(__@internal_end@__ stdin, stdout)
    }};

    (prompt $text:expr $(;args $($arg:expr),+)?) => {{
        let (mut stdin, mut stdout) = dmenu!(__@internal_start@__ args "-p" $text $($arg)*);

        dmenu!(__@internal_end@__ stdin, stdout)
    }};

    ($($e:expr),* $(;args $($arg:expr),+)?) => {{
        let (mut stdin, mut stdout) = dmenu!(__@internal_start@__ $(args $($arg)+)?);

        $(
            ::std::io::Write::write_all(&mut stdin, $e.as_bytes()).unwrap();
            ::std::io::Write::write_all(&mut stdin, b"\n").unwrap();
        )*

        dmenu!(__@internal_end@__ stdin, stdout)
    }};
}
