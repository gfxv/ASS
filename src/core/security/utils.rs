use zxcvbn::zxcvbn;
use zxcvbn::ZxcvbnError::{BlankPassword, DurationOutOfRange};
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;

const DEFAULT_LENGTH: usize = 24;

pub fn analyze_password(password: &String) -> Result<(), String>{
    let entropy = match zxcvbn(password, &[]) {
        Ok(e) => e,
        Err(err) => return match err {
            BlankPassword => Err(String::from("[SEC.ERROR] Blank password")),
            DurationOutOfRange => Err(String::from("[SEC.ERROR] Converting duration out of range"))
        }
    };

    if entropy.score() < 3 {
        let err_message = format!(
            "[SEC.INFO] Your password isn't strong enough (entropy score = {})",
            entropy.score()
        );
        return Err(String::from(err_message));
    }

    Ok(())
}

pub fn generate_password() -> String {
    return thread_rng()
            .sample_iter(&Alphanumeric)
            .take(DEFAULT_LENGTH)
            .map(char::from)
            .collect();
}
