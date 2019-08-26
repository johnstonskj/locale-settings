/*!
An interface to all manner of locale-related information.

This crate provides a higher-level interface to locale settings accessed via POSIX (see
[ISO/IEC 15897](https://www.iso.org/standard/50707.html) operating system
functions. These are under the module
[`simple_locale::settings`](settings/index.html).

This crate uses bindgen for the creation of operating system bindings to the
`langinfo`, `localcharset`, `locale`, and `xlocale` headers. Another
crate () does something similar, however...

## Example - Settings

In the following example we have a naïve implementation of a currency formatter.
To format a US dollar amount correctly we first set the current locale for the
`Currency` [`Category`](settings/locale/enum.Category.html) and then call the
[`get_currency_format`](settings/currency/fn.get_currency_format.html). From this we use
only the simplest of the formatting options to display our currency amount.

```
use std::str::FromStr;
use locale_types::{Locale, LocaleString};
use locale_settings::locale::{Category, set_locale};
use locale_settings::currency::get_currency_format;

let amount: f64 = 5.909;
let en_us = LocaleString::from_str("en_US.UTF-8").unwrap();

if set_locale(&Locale::String(en_us), &Category::Currency) {
    let format = get_currency_format();
    let local = format.local_format.unwrap();
    println!(
        "{2}{0}{3}{1:.4$}",
        amount.trunc(),
        amount.fract(),
        local.currency_symbol,
        format.number_format.decimal_separator,
        local.decimal_precision
    );
}

```

## FFI Bindings

As mentioned above, this crate depends on FFI bindings to POSIX locale
functions, and there are O/S differences that make this a pain. The script
[`create-bindings.sh`](https://github.com/johnstonskj/simple-locale/blob/master/create-bindings.sh)
is used to generate these bindings (using cargo bindgen) in such a way that
different O/S bindings can be built effectively.

Typically we treat each of the categories defined by POSIX in `locale.h` as
modules. The categories are show in the table below.

| POSIX Category | Module     | Function(s)                                  |
|----------------|------------|----------------------------------------------|
| `LC_COLLATE`   | N/A        | |
| `LC_CTYPE`     | [`codeset`](codeset/index.html) | `get_code_set_format`, `get_code_set_format_for_locale` |
| `LC_MESSAGES`  | [`messages`](messages/index.html) | `get_message_format`, `get_message_format_for_locale` |
| `LC_MONETARY`  | [`currency`](currency/index.html) | `get_currency_format`, `get_currency_format_for_locale` |
| `LC_NUMERIC`   | [`numeric`](numeric/index.html) | `get_numeric_format`, `get_numeric_format_for_locale` |
| `LC_TIME`      | [`time`](time/index.html) | `get_date_time_format`, `get_date_time_format_for_locale`, `get_calendar_names`, `get_calendar_names_for_locale` |

> Note: the POSIX category `LC_COLLATE` is not mapped to
> modules as there are no calls to retrieve specific information.

For each module there is _at least_ a matching pair of functions, one which takes
zero parameters and returns the current locale settings, and one which takes
two parameters and allows the retrieval of settings from another locale. The first
of these parameters is a [`Locale`](../locale/enum.Locale.html) enum that denotes
the locale to query, and the second parameter is a boolean `inherit_current` that
determines how the specified locale should be interpreted.

Additionally, the module [`locale`](locale/index.html) has the necessary functions
to get and set the locale either for an individual category or for all. This modules
provides implementations that use the C API directly as well as an implementation
that uses standard environment variables.

## Relationship to the POSIX API

The POSIX locale API is spread across a number of functions including
[`localeconv`](https://man.openbsd.org/localeconv.3),
[`nl_langinfo`](https://man.openbsd.org/nl_langinfo.3), and
[`setlocale`](https://man.openbsd.org/setlocale.3). Also, the
different categories of data is mixed up in common structures. The intent
of this crate is to invert this abstraction, to group together data by common
category regardless of the underlying API being used.

Those functions in the table above that end in `_for_locale` use the `xlocale`
extended API, specifically
[`newlocale`](https://man.openbsd.org/newlocale.3),
[`uselocale`](https://man.openbsd.org/uselocale.3), and
[`freelocale`](https://man.openbsd.org/freelocale.3) to obtain the settings
for a locale other than the current.
*/

#[macro_use]
extern crate log;
extern crate locale_types;
extern crate regex;

// ------------------------------------------------------------------------------------------------
// Public Modules
// ------------------------------------------------------------------------------------------------

pub mod locale;
pub use locale::Category;

pub mod codeset;

pub mod currency;

pub mod messages;

pub mod numeric;

pub mod time;

// ------------------------------------------------------------------------------------------------
// Internal Modules
// ------------------------------------------------------------------------------------------------

mod ffi;