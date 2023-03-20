/// Asserts a codepoint is a "noncharacter" based on a certain range of
/// Unicode code points.
///
/// > A noncharacter is a code point that is in the range U+FDD0 to U+FDEF,
/// > inclusive, or U+FFFE, U+FFFF, U+1FFFE, U+1FFFF, U+2FFFE, U+2FFFF, U+3FFFE,
/// > U+3FFFF, U+4FFFE, U+4FFFF, U+5FFFE, U+5FFFF, U+6FFFE, U+6FFFF, U+7FFFE,
/// > U+7FFFF, U+8FFFE, U+8FFFF, U+9FFFE, U+9FFFF, U+AFFFE, U+AFFFF, U+BFFFE,
/// > U+BFFFF, U+CFFFE, U+CFFFF, U+DFFFE, U+DFFFF, U+EFFFE, U+EFFFF, U+FFFFE,
/// > U+FFFFF, U+10FFFE, or U+10FFFF.
///
/// [whatwg-infra-dfn]: https://infra.spec.whatwg.org/#noncharacter
/// # Examples
///
/// ```
/// use whatwg_infra::is_noncharacter;
///
/// assert!(is_noncharacter(&'\u{FDD0}'));
/// assert!(is_noncharacter(&'\u{FDD1}'));
/// assert!(is_noncharacter(&'\u{FFFE}'));
/// assert!(is_noncharacter(&'\u{10FFFF}'));
/// ```
#[rustfmt::skip]
#[must_use]
#[inline]
pub const fn is_noncharacter(c: &char) -> bool {
	matches!(c,
		| '\u{FDD0}'..='\u{FDEF}'
		| '\u{FFFE}' | '\u{FFFF}' | '\u{1FFFE}' | '\u{1FFFF}'
		| '\u{2FFFE}' | '\u{2FFFF}' | '\u{3FFFE}' | '\u{3FFFF}'
		| '\u{4FFFE}' | '\u{4FFFF}' | '\u{5FFFE}' | '\u{5FFFF}'
		| '\u{6FFFE}' | '\u{6FFFF}' | '\u{7FFFE}' | '\u{7FFFF}'
		| '\u{8FFFE}' | '\u{8FFFF}' | '\u{9FFFE}' | '\u{9FFFF}'
		| '\u{AFFFE}' | '\u{AFFFF}' | '\u{BFFFE}' | '\u{BFFFF}'
		| '\u{CFFFE}' | '\u{CFFFF}' | '\u{DFFFE}' | '\u{DFFFF}'
		| '\u{EFFFE}' | '\u{EFFFF}' | '\u{FFFFE}' | '\u{FFFFF}'
		| '\u{10FFFE}' | '\u{10FFFF}'
	)
}

/// Checks if a character is a **C0 control**, as originally defined
/// by the ANSI X3.4 standard, and redefined by the
/// [WHATWG Infra Standard][whatwg-infra-dfn].
///
/// Any character is a C0 control if it's within the inclusive range
/// of U+0000 NULL or U+001F INFORMATION SEPARATOR ONE.
///
/// This method is subtly different than [char::is_ascii_control] and
/// [u8::is_ascii_control], which also checks for U+007F DELETE.
///
/// See also: [WHATWG Infra Standard definition][whatwg-infra-dfn]
///
/// [whatwg-infra-dfn]: https://infra.spec.whatwg.org/#c0-control
///
/// # Examples
/// ```
/// use whatwg_infra::is_c0_control;
///
/// assert!(is_c0_control(&'\u{0000}'));
/// assert!(is_c0_control(&'\u{001E}'));
/// assert!(is_c0_control(&'\u{001F}'));
/// ```
#[must_use]
#[inline]
pub const fn is_c0_control(c: &char) -> bool {
	*c <= '\u{001F}'
}

/// Checks if a character is a **C0 control** or space (U+0020 SPACE).
///
/// See also: [WHATWG Infra Standard definition][whatwg-infra-dfn]
///
/// [whatwg-infra-dfn]: https://infra.spec.whatwg.org/#c0-control-or-space
#[must_use]
#[inline]
pub const fn is_c0_control_space(c: &char) -> bool {
	*c <= '\u{0020}'
}

/// Checks if a codepoint is equivalent to one of three ASCII whitespace code points
/// * U+0009 TAB
/// * U+000A LINE FEED (LF)
/// * U+000D CARRIAGE RETURN (CR)
///
/// See also: [WHATWG Infra Standard definition][whatwg-infra-dfn]
///
/// [whatwg-infra-dfn]: https://infra.spec.whatwg.org/#ascii-tab-or-newline
#[must_use]
#[inline]
pub const fn is_ascii_tab_newline(c: &char) -> bool {
	matches!(*c, '\u{0009}' | '\u{000A}' | '\u{000D}')
}

/// Replaces every U+000D U+000A pair of codepoints with a single U+000A
/// codepoint, and any remaining U+000D codepoint with a U+000A codepoint.
///
/// See also: [WHATWG Infra Standard definition][whatwg-infra-dfn]
///
/// [whatwg-infra-dfn]: https://infra.spec.whatwg.org/#normalize-newlines
#[must_use]
#[inline]
pub fn normalize_newlines(s: &str) -> String {
	s.replace("\u{000D}\u{000A}", "\u{000A}")
		.as_str()
		.replace('\u{000D}', "\u{000A}")
}

/// A string without any U+000A LINE FEED (LF) or U+000D CARIAGE RETURN (CR)
/// codepoints, as defined by the [WHATWG Infra Standard][whatwg-infra-dfn].
///
/// See also: [WHATWG Infra Standard definition][whatwg-infra-dfn]
///
/// [whatwg-infra-dfn]: https://infra.spec.whatwg.org/#strip-newlines
#[must_use]
#[inline]
pub fn strip_newlines(s: &str) -> String {
	let mut result = String::with_capacity(s.len());
	let mut stripped_codepoints = 0usize;

	for c in s.chars() {
		if c != '\u{000A}' && c != '\u{000D}' {
			result.push(c);
			stripped_codepoints += 1usize;
		}
	}

	if result.len() != s.len() {
		result.shrink_to(s.len() - stripped_codepoints);
	}

	result
}

/// Removes ASCII whitespace from before and after a string, and collapses
/// runs of ASCII whitespaces by replacing them with a single U+0020 SPACE code point.
///
/// See also: [WHATWG Infra Standard definition][whatwg-infra-dfn]
///
/// [whatwg-infra-dfn]: https://infra.spec.whatwg.org/#strip-leading-and-trailing-ascii-whitespace
#[must_use]
pub fn trim_ascii_whitespace(s: &str) -> &str {
	s.trim_matches(|c: char| c.is_ascii_whitespace())
}

/// Collects a sequence of Unicode codepoints given a predicate function
/// and position to move forward.
///
/// See also: [WHATWG Infra Standard definition][whatwg-infra-dfn]
///
/// [whatwg-infra-dfn]: https://infra.spec.whatwg.org/#collect-a-sequence-of-code-points
#[must_use]
pub fn collect_codepoints<P>(s: &str, position: &mut usize, mut predicate: P) -> String
where
	P: FnMut(&char) -> bool,
{
	if s.is_empty() || position >= &mut s.len() {
		return String::new();
	}

	let mut result = String::with_capacity(s.len() - *position);
	let rest = s.chars().skip(*position);
	let starting_position = *position;

	for c in rest {
		if position < &mut s.len() && predicate(&c) {
			*position += 1;
		} else {
			break;
		}
	}

	result.push_str(&s[starting_position..*position]);
	if result.len() < s.len() - *position {
		result.shrink_to_fit();
	}

	result
}
