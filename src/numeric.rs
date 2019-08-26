/*!
Fetch locale-specific number formatting settings.

This module provides basic formatting rules for most rational
numbers; floating point numbers in scientific notation, fractional
numbers, and imaginary numbers are not covered.

## Example

```
use locale_settings::locale::{Category, get_locale};
use locale_settings::numeric::get_numeric_format;
use locale_types::Locale;

if get_locale(&Category::Currency).unwrap() == Locale::POSIX {
    let format = get_numeric_format();
    assert_eq!(format.decimal_separator, ".");
} else {
    panic!("expecting POSIX locale");
}
```
*/

use crate::ffi::localeconv;
use crate::ffi::utils::*;
use crate::Category;
use locale_types::{Locale, LocaleResult};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

/// Numeric formatting settings.
#[derive(Debug, Clone)]
pub struct NumericFormat {
    /// The symbol to use for positive values.
    pub positive_sign: String,
    /// The symbol to use for negative values.
    pub negative_sign: String,
    /// The radix character.
    pub decimal_separator: String, // TODO: is radix_character better?
    /// The separator between groups.
    pub thousands_separator: String, // TODO: is group_separator better?
    /// The grouping rules.
    pub grouping: Vec<usize>,
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

/// Fetch the numeric formatting settings for the current locale.
pub fn get_numeric_format() -> NumericFormat {
    unsafe {
        let lconv = localeconv();
        NumericFormat {
            positive_sign: cstr_to_string((*lconv).positive_sign),
            negative_sign: cstr_to_string((*lconv).negative_sign),
            decimal_separator: cstr_to_string((*lconv).decimal_point),
            thousands_separator: cstr_to_string((*lconv).thousands_sep),
            grouping: grouping_vector((*lconv).grouping),
        }
    }
}

/// Fetch the numeric formatting rules for a specified `Locale`.
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
pub fn get_numeric_format_for_locale(
    locale: Locale,
    inherit_current: bool,
) -> LocaleResult<NumericFormat> {
    get_format_for_locale(
        Category::Numeric,
        locale,
        &get_numeric_format,
        inherit_current,
    )
}

#[cfg(experimental)]
pub mod fmt {
    use std::fmt::Display;

    pub fn format<T>(value: T) -> String
    where
        T: Display,
    {
        let initial = value.to_string();
        if initial.chars().all(|c| c.is_digit(10) || c == '.') {
            initial
        } else {
            panic!("doesn't look like a nunber");
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Unit Tests
// ------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use crate::locale::{set_locale, Category};
    use crate::numeric::get_numeric_format;
    use locale_types::Locale;

    // --------------------------------------------------------------------------------------------
    #[test]
    fn test_numeric_settings() {
        if set_locale(&Locale::POSIX, &Category::Numeric) {
            let format = get_numeric_format();
            println!("{:#?}", format);
            assert_eq!(format.decimal_separator, ".");
        } else {
            panic!("set_locale returned false");
        }
    }
}
