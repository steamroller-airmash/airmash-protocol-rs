use bstr::ByteSlice;

pub(crate) fn variant_eq(value: &[u8], variant: &str) -> bool {
  let mut it1 = value.bytes();
  let mut it2 = variant.bytes();

  // If the first letter is uppercase then we don't want to support adding
  // '-' and '_' chars at the front of the string.
  match (it1.next(), it2.next()) {
    (None, None) => return true,
    (Some(b1), Some(b2)) if b1.eq_ignore_ascii_case(&b2) => (),
    _ => return false,
  }

  loop {
    let (b1, b2) = match (it1.next(), it2.next()) {
      (None, None) => return true,
      (Some(b1), Some(b2)) => match b1 {
        b'-' | b'_' if b2.is_ascii_uppercase() => match it1.next() {
          Some(b1) => (b1, b2),
          None => return false,
        },
        _ => (b1, b2),
      },
      _ => return false,
    };

    if !b1.eq_ignore_ascii_case(&b2) {
      return false;
    }
  }
}

#[cfg(test)]
mod variant_eq_tests {
  use super::variant_eq;

  macro_rules! test_variant_eq {
    {
      $( $name:ident => $value:literal $tt:tt $variant:literal; )*
    } => {
      $( test_variant_eq!($name => $value $tt $variant); )*
    };
    ($name:ident => $value:literal == $variant:literal) => {
      #[test]
      fn $name() {
        let value: &str = $value;
        let variant: &str = $variant;

        assert!(
          variant_eq(value.as_bytes(), variant),
          "`{}` != `{}`",
          value,
          variant
        );
      }
    };
    ($name:ident => $value:literal != $variant:literal) => {
      #[test]
      fn $name() {
        let value: &str = $value;
        let variant: &str = $variant;

        assert!(
          !variant_eq(value.as_bytes(), variant),
          "`{}` == `{}`",
          value,
          variant
        );
      }
    };
  }

  test_variant_eq! {
    // case-insensitivity tests
    case_insensitive_1 => "blah" == "Blah";
    case_insensitive_2 => "bLaH" == "Blah";
    case_insensitive_3 => "BLEH" == "Bleh";
    case_insensitive_4 => "bleh" == "Bleh";
    case_insensitive_5 => "bleh" != "blah";

    // kebab-case tests
    kebab_1 => "blah-bleh-blah" == "BlahBlehBlah";
    kebab_2 => "blahbleh-blah" == "BlahBlehBlah";
    kebab_3 => "bl-ahBLEHblah" != "BlahBlehBlah";
    kebab_4 => "-blahblehblah" != "BlahBlehBlah";

    // SCREAMING_CASE tests
    screaming_1 => "BLAH_BLEH" == "BlahBleh";
    screaming_2 => "BL_AHBL_EH" != "BlahBleh";
    screaming_3 => "BlAh_bLeH" == "BlahBleh";
  }
}
