// Copyright 2025 Gabriel Bjørnager Jensen.

macro_rules! log {
	(debug, $($fmt:tt)+) => {{
		if ::std::cfg!(debug_assertions) {
			::std::eprintln!($($fmt)*);
		}
	}};

	(note, $($fmt:tt)+) => {{
		if ::std::cfg!(debug_assertions) {
			::std::eprint!("\u{001B}[002m\u{001B}[096mnote\u{001B}[039m - ");
			::std::eprintln!($($fmt)*);
			::std::eprint!("\u{001B}[039m\u{001B}[022m");
		}
	}};

	(warning, $($fmt:tt)+) => {{
		::std::eprint!("\u{001B}[095mwarning\u{001B}[039m - ");
		::std::eprintln!($($fmt)*);
	}};

	(error, $($fmt:tt)+) => {{
		::std::eprint!("\u{001B}[091merror\u{001B}[039m - ");
		::std::eprintln!($($fmt)*);
	}};

	($($fmt:tt)+) => {{
		::std::eprintln!($($fmt)*);
	}};
}

pub(crate) use log;
