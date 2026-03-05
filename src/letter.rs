
const DIGIT_1: &str = r#"
  $$\
$$$$ |
\_$$ |
  $$ |
  $$ |
  $$ |
$$$$$$\
\______|
"#;

const DIGIT_2: &str = r#"
 $$$$$$\
$$  __$$\
\__/  $$ |
 $$$$$$  |
$$  ____/
$$ |
$$$$$$$$\
\________|
"#;
const DIGIT_3: &str = r#"
 $$$$$$\
$$ ___$$\
\_/   $$ |
  $$$$$ /
  \___$$\
$$\   $$ |
\$$$$$$  |
 \______/
"#;

const DIGIT_4: &str = r#"
$$\   $$\
$$ |  $$ |
$$ |  $$ |
$$$$$$$$ |
\_____$$ |
      $$ |
      $$ |
      \__|
"#;

const DIGIT_5: &str = r#"
$$$$$$$\
$$  ____|
$$ |
$$$$$$$\
\_____$$\
$$\   $$ |
\$$$$$$  |
 \______/
"#;

const DIGIT_6: &str = r#"
 $$$$$$\
$$  __$$\
$$ /  \__|
$$$$$$$\
$$  __$$\
$$ /  $$ |
 $$$$$$  |
 \______/
"#;

const DIGIT_7: &str = r#"
$$$$$$$$\
\____$$  |
    $$  /
   $$  /
  $$  /
 $$  /
$$  /
\__/
"#;


const DIGIT_8: &str = r#"
 $$$$$$\
$$  __$$\
$$ /  $$ |
 $$$$$$  |
$$  __$$<
$$ /  $$ |
\$$$$$$  |
 \______/
"#;

const DIGIT_9: &str = r#"
 $$$$$$\
$$  __$$\
$$ /  $$ |
\$$$$$$$ |
 \____$$ |
$$\   $$ |
\$$$$$$  |
 \______/
"#;

const DIGIT_0: &str = r#"
 $$$$$$\
$$$ __$$\
$$$$\ $$ |
$$\$$\$$ |
$$ \$$$$ |
$$ |\$$$ |
\$$$$$$  /
 \______/
"#;

const COLON: &str = r#"
    _
   (")

    _
   (")
"#;

const UNKNOWN: &str = r#"
 ___
(__ )
 (_/
 (_)
"#;
pub enum Letter {
    Digit1,
    Digit2,
    Digit3,
    Digit4,
    Digit5,
    Digit6,
    Digit7,
    Digit8,
    Digit9,
    Digit0,
    Colon,
    Unknown,
}


impl From<&str> for Letter {
    fn from(s: &str) -> Self {
        match s {
            "1" => Letter::Digit1,
            "2" => Letter::Digit2,
            "3" => Letter::Digit3,
            "4" => Letter::Digit4,
            "5" => Letter::Digit5,
            "6" => Letter::Digit6,
            "7" => Letter::Digit7,
            "8" => Letter::Digit8,
            "9" => Letter::Digit9,
            "0" => Letter::Digit0,
            ":" => Letter::Colon,
            _ => Letter::Unknown
        }
    }
}

impl Letter {
    pub fn as_str(&self) -> &'static str {
        match self {
            Letter::Digit1 => DIGIT_1,
            Letter::Digit2 => DIGIT_2,
            Letter::Digit3 => DIGIT_3,
            Letter::Digit4 => DIGIT_4,
            Letter::Digit5 => DIGIT_5,
            Letter::Digit6 => DIGIT_6,
            Letter::Digit7 => DIGIT_7,
            Letter::Digit8 => DIGIT_8,
            Letter::Digit9 => DIGIT_9,
            Letter::Digit0 => DIGIT_0,
            Letter::Colon => COLON,
            Letter::Unknown => UNKNOWN,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::letter::{Letter, DIGIT_0, DIGIT_1, DIGIT_3, DIGIT_4, DIGIT_5, DIGIT_6, DIGIT_7, DIGIT_8, DIGIT_9};
    use crate::letter::DIGIT_2;
    #[test]
    pub fn test() {
        let width = 10;
        DIGIT_1.lines().map(|l| format!("{:<width$}", l)).for_each(|l| println!("{}", l));
    }
}