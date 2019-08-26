/*!
Fetch locale-specific currency formatting settings.

The formatting information here does include the currency symbol,
_international symbol_, and precision all of which overlap with the
`symbol`, `alphabetic_code`, and subdivision `exponent` from the
`codes::currency` module. However, there is significantly more
information here and this should be used wherever necessary to
format currency values correctly.

However, values from the currency code can be used, for example the
name and subdivision name(s), when labeling currency values.

## Example

The following should display "19 US Dollars and 99 cents".

```
use std::str::FromStr;
use locale_types::{Locale, LocaleString};
use locale_codes::currency;
use locale_settings::locale::{Category, set_locale};
use locale_settings::currency::get_currency_format;

let amount: f64 = 19.9950;
let en_us = LocaleString::from_str("en_US.UTF-8").unwrap();
if set_locale(&Locale::String(en_us), &Category::Currency) {
    let format = get_currency_format();
    let currency = currency::lookup_by_alpha(&format.international_format.unwrap().currency_symbol.trim()).unwrap();
    let local = format.local_format.unwrap();
    let subdiv = currency.subdivisions.get(0).unwrap();
    let subdiv_name = &subdiv.name.clone().unwrap();
    println!(
        "{0} {2} and {1:.4$} {3}",
        amount.trunc(),
        amount.fract(),
        currency.name,
        subdiv_name,
        local.decimal_precision
    );
}
```
*/

use crate::ffi::localeconv;
use crate::ffi::utils::*;
use crate::numeric::NumericFormat;
use crate::Category;
use locale_types::{Locale, LocaleResult};
use std::os::raw::c_char;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

/// This enumeration defines the handling of sign placement and
/// choice for either positive or negative numeric values. The
/// examples for each use a value of minus 999.99.
///
/// Note that one format that cannot be easily represented here
/// is the often used spreadsheet _accounting format_ where the
/// numeric part of the value is right aligned but currency symbols
/// are left aligned in a cell and parenthesis are used to denote
/// negatives but again the opening parenthesis is left aligned and
/// the closing parenthesis is right aligned.
#[derive(Debug, Clone, PartialEq)]
pub enum SignLocation {
    /// Use parenthesis `($999.99)` around the value to denote sign.
    UseParenthesis,
    /// Place a symbol before the value `$-999.99`.
    BeforeString,
    /// Place a symbol after the value `$999.99-`.
    AfterString,
    /// Place a symbol before the currency sign `-$999.99`.
    BeforeCurrencySymbol,
    /// Place a symbol after the currency sign `$-999.99`.
    AfterCurrencySymbol,
}

/// This denotes the complete handling of sign placement for
/// a currency value.
#[derive(Debug, Clone, PartialEq)]
pub struct SignFormat {
    /// The currency symbol precedes (or follows) the numeric value.
    pub currency_symbol_precedes: bool,
    /// A space separates the numeric value and currency symbol.
    pub space_separated: bool,
    /// Placement of the sign symbol.
    pub sign_location: SignLocation,
}

/// The actual formatting specification for the currency-specific
/// part of a value.
#[derive(Debug, Clone, PartialEq)]
pub struct CurrencyCellFormat {
    /// currency symbol to use.
    pub currency_symbol: String,
    /// number of digits of precision typically used.
    pub decimal_precision: usize,
    /// The correct way to display positive values.
    pub positive_sign_format: SignFormat,
    /// The correct way to display negative values.
    pub negative_sign_format: SignFormat,
}

/// The complete currency formatting rules.
#[derive(Debug, Clone)]
pub struct CurrencyFormat {
    /// Numeric handling, we reuse this.
    pub number_format: NumericFormat,
    /// Formatting values in its native locale.
    pub local_format: Option<CurrencyCellFormat>,
    /// Formatting a value outside its native locale.
    pub international_format: Option<CurrencyCellFormat>,
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

/// Fetch the current local-specific currency formatting rules.
pub fn get_currency_format() -> CurrencyFormat {
    unsafe {
        let char_max = c_char::max_value();
        let lconv = localeconv();
        CurrencyFormat {
            number_format: NumericFormat {
                positive_sign: cstr_to_string((*lconv).positive_sign),
                negative_sign: cstr_to_string((*lconv).negative_sign),
                decimal_separator: cstr_to_string((*lconv).mon_decimal_point),
                thousands_separator: cstr_to_string((*lconv).mon_thousands_sep),
                grouping: grouping_vector((*lconv).mon_grouping),
            },
            local_format: if (*lconv).frac_digits == char_max {
                None
            } else {
                Some(CurrencyCellFormat {
                    currency_symbol: cstr_to_string((*lconv).currency_symbol),
                    decimal_precision: (*lconv).frac_digits as usize,
                    positive_sign_format: SignFormat {
                        currency_symbol_precedes: ((*lconv).p_cs_precedes == 1),
                        space_separated: ((*lconv).p_sep_by_space == 1),
                        sign_location: match (*lconv).p_sign_posn {
                            0 => SignLocation::UseParenthesis,
                            1 => SignLocation::BeforeString,
                            2 => SignLocation::AfterString,
                            3 => SignLocation::BeforeCurrencySymbol,
                            4 => SignLocation::AfterCurrencySymbol,
                            _ => panic!("Bad sign location {}", (*lconv).p_sign_posn),
                        },
                    },
                    negative_sign_format: SignFormat {
                        currency_symbol_precedes: ((*lconv).n_cs_precedes == 1),
                        space_separated: ((*lconv).n_sep_by_space == 1),
                        sign_location: match (*lconv).n_sign_posn {
                            0 => SignLocation::UseParenthesis,
                            1 => SignLocation::BeforeString,
                            2 => SignLocation::AfterString,
                            3 => SignLocation::BeforeCurrencySymbol,
                            4 => SignLocation::AfterCurrencySymbol,
                            _ => panic!("Bad sign location {}", (*lconv).n_sign_posn),
                        },
                    },
                })
            },
            international_format: if (*lconv).int_frac_digits == char_max {
                None
            } else {
                Some(CurrencyCellFormat {
                    currency_symbol: cstr_to_string((*lconv).int_curr_symbol),
                    decimal_precision: (*lconv).int_frac_digits as usize,
                    positive_sign_format: SignFormat {
                        currency_symbol_precedes: ((*lconv).int_p_cs_precedes == 1),
                        space_separated: ((*lconv).int_p_sep_by_space == 1),
                        sign_location: match (*lconv).int_p_sign_posn {
                            0 => SignLocation::UseParenthesis,
                            1 => SignLocation::BeforeString,
                            2 => SignLocation::AfterString,
                            3 => SignLocation::BeforeCurrencySymbol,
                            4 => SignLocation::AfterCurrencySymbol,
                            _ => panic!("Bad sign location {}", (*lconv).int_p_sign_posn),
                        },
                    },
                    negative_sign_format: SignFormat {
                        currency_symbol_precedes: ((*lconv).int_n_cs_precedes == 1),
                        space_separated: ((*lconv).int_n_sep_by_space == 1),
                        sign_location: match (*lconv).int_n_sign_posn {
                            0 => SignLocation::UseParenthesis,
                            1 => SignLocation::BeforeString,
                            2 => SignLocation::AfterString,
                            3 => SignLocation::BeforeCurrencySymbol,
                            4 => SignLocation::AfterCurrencySymbol,
                            _ => panic!("Bad sign location {}", (*lconv).int_n_sign_posn),
                        },
                    },
                })
            },
        }
    }
}

/// Fetch the currency formatting rules for a specified `Locale`.
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
pub fn get_currency_format_for_locale(
    locale: Locale,
    inherit_current: bool,
) -> LocaleResult<CurrencyFormat> {
    get_format_for_locale(
        Category::Currency,
        locale,
        &get_currency_format,
        inherit_current,
    )
}

// ------------------------------------------------------------------------------------------------
// Unit Tests
// ------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use crate::currency::{get_currency_format, get_currency_format_for_locale};
    use crate::locale::{set_locale, Category};
    use locale_types::{Locale, LocaleString};
    use std::str::FromStr;

    // --------------------------------------------------------------------------------------------
    #[test]
    fn test_currency_settings() {
        if set_locale(&Locale::POSIX, &Category::Currency) {
            let format = get_currency_format();
            println!("{:#?}", format);
            assert_eq!(format.number_format.decimal_separator, "");
            assert_eq!(format.local_format, None);
            assert_eq!(format.international_format, None);
        } else {
            panic!("set_locale returned false");
        }
    }

    #[test]
    fn test_currency_settings_us() {
        let en_us = LocaleString::from_str("en_US.UTF-8").unwrap();
        if set_locale(&Locale::String(en_us), &Category::Currency) {
            let format = get_currency_format();
            println!("{:#?}", format);
            assert_eq!(format.number_format.decimal_separator, ".");
            assert_eq!(format.local_format.unwrap().currency_symbol, "$");
            assert_eq!(format.international_format.unwrap().currency_symbol, "USD ");
        } else {
            panic!("set_locale returned false");
        }
    }

    // --------------------------------------------------------------------------------------------
    #[test]
    fn test_currency_settings_for_locale() {
        if set_locale(&Locale::POSIX, &Category::Currency) {
            let locale = Locale::from_str("ja_JP.SJIS").unwrap();
            let format = get_currency_format_for_locale(locale, false).unwrap();
            println!("{:#?}", format);
            assert_eq!(format.number_format.decimal_separator, ".");
            assert!(format.local_format.is_some());
            assert!(format.international_format.is_some());
            assert_eq!(
                format.international_format.unwrap().currency_symbol,
                "JPY ".to_string()
            );
        } else {
            panic!("set_locale returned false");
        }
    }
}
