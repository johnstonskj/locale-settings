/*!
Fetch locale-specific message formatting settings.

This module is relatively simple as only a small number of
settings are defined by POSIX for the messages category.

## Example

```
use locale_settings::locale::{Category, get_locale};
use locale_settings::messages::get_message_format;
use locale_types::Locale;
use std::str::FromStr;

let locale = get_locale(&Category::Time).unwrap();
if locale == Locale::POSIX {
    let format = get_message_format();
    assert_eq!(format.yes_expression, Some("^[yY]".to_string()));
    assert_eq!(format.no_string, Some("no".to_string()));
} else {
    panic!("expecting POSIX locale");
}
```
*/

use crate::ffi::utils::*;
use crate::ffi::*;
use crate::Category;
use locale_types::{Locale, LocaleResult};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

/// Settings related to message display.
#[derive(Debug, Clone, PartialEq)]
pub struct MessageFormat {
    /// An expression to validate 'yes' values.
    pub yes_expression: Option<String>,
    /// A default string for 'yes'.
    pub yes_string: Option<String>,
    /// An expression to validate 'no' values.
    pub no_expression: Option<String>,
    /// A default string for 'no'.
    pub no_string: Option<String>,
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

/// Fetch the message formatting settings for the current locale.
pub fn get_message_format() -> MessageFormat {
    MessageFormat {
        yes_expression: get_nl_string(YESEXPR),
        yes_string: get_nl_string(YESSTR),
        no_expression: get_nl_string(NOEXPR),
        no_string: get_nl_string(NOSTR),
    }
}

/// Fetch the message formatting rules for a specified `Locale`.
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
pub fn get_message_format_for_locale(
    locale: Locale,
    inherit_current: bool,
) -> LocaleResult<MessageFormat> {
    get_format_for_locale(
        Category::Message,
        locale,
        &get_message_format,
        inherit_current,
    )
}

// ------------------------------------------------------------------------------------------------
// Unit Tests
// ------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use crate::locale::{set_locale, Category};
    use crate::messages::{get_message_format, get_message_format_for_locale};
    use locale_types::Locale;
    use std::str::FromStr;

    // --------------------------------------------------------------------------------------------
    #[test]
    fn test_get_message_format() {
        if set_locale(&Locale::POSIX, &Category::Message) {
            let format = get_message_format();
            println!("{:#?}", format);
            assert_eq!(format.yes_expression, Some("^[yY]".to_string()));
            assert_eq!(format.no_string, Some("no".to_string()));
        } else {
            panic!("set_locale returned false");
        }
    }

    // --------------------------------------------------------------------------------------------
    #[test]
    fn test_get_message_format_for_locale() {
        if set_locale(&Locale::POSIX, &Category::Message) {
            let format = get_message_format_for_locale(Locale::from_str("fr_FR").unwrap(), false);
            println!("{:#?}", format);
            let format = format.unwrap();
            assert_eq!(format.yes_expression, Some("^[oOyY].*".to_string()));
            assert_eq!(format.no_string, Some("no".to_string()));
        } else {
            panic!("set_locale returned false");
        }
    }
}
