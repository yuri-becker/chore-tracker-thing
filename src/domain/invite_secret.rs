use base64::engine::general_purpose::URL_SAFE;
use base64::Engine;
use log::{debug, warn};
use rand::RngCore;

pub struct InviteSecret {
    pub secret: String,
    pub digest: String,
}

impl InviteSecret {
    pub fn generate() -> Result<InviteSecret, argon2::Error> {
        let mut rng = rand::rng();

        let mut secret = [0u8; 16];
        rng.fill_bytes(&mut secret);
        let mut salt = [0u8; 8];
        rng.fill_bytes(&mut salt);

        let digest = argon2::hash_encoded(&secret, &salt, &argon2::Config::default())?;
        let secret = URL_SAFE.encode(secret);
        Ok(InviteSecret { secret, digest })
    }

    pub fn validate(&self) -> bool {
        URL_SAFE
            .decode(&self.secret)
            .inspect_err(|err| {
                debug!(
                    "Could not decode secret, given string is likely invalid: {:?}",
                    err
                )
            })
            .map(|secret| argon2::verify_encoded(&self.digest, secret.as_slice()))
            .unwrap_or(Ok(false))
            .inspect_err(|err| {
                warn!(
                    "Failed to decode secret, digest is likely invalid: {:?}",
                    err
                )
            })
            .unwrap_or(false)
    }
}

#[cfg(test)]
mod test {
    use crate::domain::invite_secret::InviteSecret;

    #[test]
    fn test_secret_round_trip() {
        let secret = InviteSecret::generate().unwrap();
        let result = InviteSecret::validate(&secret);
        assert!(result);
    }

    #[test]
    fn test_invalid_secret() {
        let secret = InviteSecret::generate().unwrap();
        let result = InviteSecret {
            digest: secret.digest,
            secret: secret.secret + "!",
        }
        .validate();
        assert!(!result);
    }
}
