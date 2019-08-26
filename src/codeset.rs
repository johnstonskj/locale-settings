/*!
Fetch locale-specific code-set settings.

This module is relatively simple as only a small number of
settings are defined by POSIX for the code-set category.

*/

use crate::ffi::utils::*;
use crate::ffi::{___mb_cur_max, CODESET};
use crate::Category;
use locale_types::{Locale, LocaleResult};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

/// Settings related to message display.
#[derive(Debug, Clone, PartialEq)]
pub struct CodeSetFormat {
    /// The code set to use when displaying messages in the current locale.
    /// Note that this does not return standard code set identifiers and so
    /// this value cannot be used with the `codes::codesets` module.
    pub code_set: Option<String>,
    /// The maximum number of bytes needed to represent a single wide
    /// character in the current locale.
    pub multibyte_max_bytes: Option<u32>,
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

/// Fetch the code-set settings for the current locale.
pub fn get_code_set_format() -> CodeSetFormat {
    let mb_max_bytes = unsafe { ___mb_cur_max() as u32 };
    CodeSetFormat {
        code_set: get_nl_string(CODESET),
        multibyte_max_bytes: Some(mb_max_bytes),
    }
}

/// Fetch the code-set rules for a specified `Locale`.
///
/// # Arguments
///
/// * `locale` - The locale to query.
/// * `inherit_current` - Whether the specified locale should inherit
///   from the current locale.
///
/// If `inherit_current` is `false` the `locale` specified will be treated
/// as an entirely new and complete locale when calling the C
/// [`newlocale`](https://man.openbsd.org/newlocale.3) function. If it is
/// `true` the `locale` is assumed to be a partially specified one and inherits
/// any unspecified components from the current locale. For example, if the
/// current locale is `en_US.UTF-8` and the parameters passed are `_NZ` and
/// `true` then the resulting locale will be `en_NZ.UTF-8`.
pub fn get_code_set_format_for_locale(
    locale: Locale,
    inherit_current: bool,
) -> LocaleResult<CodeSetFormat> {
    get_format_for_locale(
        Category::CharacterTypes,
        locale,
        &get_code_set_format,
        inherit_current,
    )
}

// ------------------------------------------------------------------------------------------------
// Unit Tests
// ------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use crate::codeset::{get_code_set_format, get_code_set_format_for_locale};
    use crate::locale::{set_locale, Category};
    use locale_types::Locale;
    use std::str::FromStr;

    // --------------------------------------------------------------------------------------------
    #[test]
    fn test_get_code_set_format() {
        if set_locale(&Locale::POSIX, &Category::CharacterTypes) {
            let format = get_code_set_format();
            println!("{:#?}", format);
            assert_eq!(format.code_set, Some("US-ASCII".to_string()));
            assert_eq!(format.multibyte_max_bytes, Some(1));
        } else {
            panic!("set_locale returned false");
        }
    }

    // --------------------------------------------------------------------------------------------
    #[test]
    fn test_get_code_set_format_for_locale() {
        if set_locale(&Locale::POSIX, &Category::CharacterTypes) {
            let format = get_code_set_format_for_locale(Locale::from_str("fr_FR").unwrap(), false);
            println!("{:#?}", format);
            let format = format.unwrap();
            assert_eq!(format.code_set, None);
            assert_eq!(format.multibyte_max_bytes, Some(4));
        } else {
            panic!("set_locale returned false");
        }
    }

    #[test]
    fn test_get_code_set_format_for_locale_2() {
        if set_locale(&Locale::POSIX, &Category::CharacterTypes) {
            let format =
                get_code_set_format_for_locale(Locale::from_str("fr_FR.UTF-8").unwrap(), false);
            println!("{:#?}", format);
            let format = format.unwrap();
            assert_eq!(format.code_set, Some("UTF-8".to_string()));
            assert_eq!(format.multibyte_max_bytes, Some(4));
        } else {
            panic!("set_locale returned false");
        }
    }
}
