use anyhow::{Context, Error as AnyErr};
use bytes::Bytes;
use chrono::{Duration, Local};
use crustchan::rejections::Unauthorized;
use crypto::blake2b::Blake2b; // WARNING: use Blake2b-512 or Keccak-512
use crypto::digest::Digest;
use ed25519_dalek::{self as ed, Keypair, PublicKey, Signature, SignatureError, Signer};
use rand::rngs::OsRng;
use serde::{Deserialize, Serialize};
use std::fs;
use warp::reject::Rejection;
use warp::Reply;

const KEYS_FOLDER: &'static str = "./.cache/keys"; // WARNING pass via configMap, use fs::Path
lazy_static::lazy_static! {
    pub static ref KEYPAIR_AUTHN:KeyPair = KeyPair::from_file_or_new("keypair_tkn_sign").expect("failed generating keypair for token signing");
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub iat: i64,
    pub exp: i64,
    pub userID: i64,
}
impl Claims {
    fn from_user_id(user_id: i64) -> Self {
        Self {
            userID: user_id,
            iat: Local::now().timestamp(),
            exp: (Local::now() + Duration::hours(24)).timestamp(),
        }
    }
    fn hash(&self) -> [u8; 32] {
        let mut ret = [0u8; 32];
        let mut hasher = Blake2b::new(32);
        hasher.input(&self.userID.to_be_bytes());
        hasher.input(&self.iat.to_be_bytes());
        hasher.input(&self.exp.to_be_bytes());
        hasher.result(&mut ret);
        ret
    }
    fn sign(self) -> Result<AuthnToken, Rejection> {
        let sig = KEYPAIR_AUTHN.sign(&self.hash());
        Ok(AuthnToken { claims: self, sig })
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthnToken {
    pub claims: Claims,
    pub sig: ed::Signature,
}
impl AuthnToken {
    pub fn from_user_id(user_id: i64) -> Result<AuthnToken, Rejection> {
        Claims::from_user_id(user_id).sign()
    }
    pub fn verify(&self) -> Result<(), Rejection> {
        if self.claims.exp < Local::now().timestamp() {
            return Err(warp::reject::custom(Unauthorized));
        }
        KEYPAIR_AUTHN
            .verify(&self.claims.hash(), &self.sig)
            .map_err(|_| warp::reject::custom(Unauthorized));
        Ok(())
    }
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut b = Bytes::new();
        b.extend_from_slice(&self.claims.iat.to_be_bytes());
        b.extend_from_slice(&self.claims.exp.to_be_bytes());
        b.extend_from_slice(&self.claims.userID.to_be_bytes());
        b.extend_from_slice(&self.sig.to_bytes());
        // b.len is 88
        b.to_vec()
    }
    pub fn from_bytes<'a>(bytes: &'a [u8]) -> Result<Self, AnyErr> {
        let mut buf = [0u8; 8];
        buf.copy_from_slice(&bytes[0..8]);
        let iat: i64 = i64::from_be_bytes(buf);
        buf.copy_from_slice(&bytes[8..16]);
        let exp: i64 = i64::from_be_bytes(buf);
        buf.copy_from_slice(&bytes[16..24]);
        let userID: i64 = i64::from_be_bytes(buf);

        let sig: Signature = Signature::from_bytes(&bytes[24..])?;
        Ok(AuthnToken {
            claims: Claims { iat, exp, userID },
            sig,
        })
    }
    pub fn to_str(&self) -> String {
        base64::encode(&self.to_bytes())
    }
    pub fn from_str(token: &str) -> Result<Self, AnyErr> {
        let bytes = base64::decode(&token)?;
        Ok(Self::from_bytes(&bytes)?)
    }
    pub fn header_val(&self) -> String {
        format!(
            "token={};Path=/;SameSite=Strict;Secure;HttpOnly",
            self.to_str()
        )
    }
}
impl Reply for AuthnToken {
    fn into_response(self) -> warp::reply::Response {
        warp::reply::json(&self).into_response()
    }
}

#[derive(Debug)]
pub struct KeyPair(ed::Keypair);
impl KeyPair {
    pub fn generate() -> Self {
        Self(Keypair::generate(&mut OsRng {}))
    }
    pub fn _pubkey(&self) -> PublicKey {
        self.0.public
    }
    pub fn sign(&self, message: &[u8]) -> Signature {
        self.0.try_sign(message).unwrap()
    }
    pub fn verify(&self, message: &[u8], signature: &Signature) -> Result<(), SignatureError> {
        self.0.verify(message, signature)
    }
    pub fn from_bytes<'a>(bytes: &'a [u8]) -> Result<Self, SignatureError> {
        Ok(Self(Keypair::from_bytes(bytes)?))
    }
    pub fn to_bytes(&self) -> [u8; 64] {
        self.0.to_bytes()
    }
    pub fn from_str(s: &str) -> Result<Self, AnyErr> {
        Ok(Self::from_bytes(&base64::decode(s)?.to_vec())?)
    }
    pub fn to_str(&self) -> String {
        base64::encode(self.to_bytes().to_vec())
    }
    fn to_file(&self, keyfile: &str) -> Result<&Self, AnyErr> {
        fs::create_dir_all(KEYS_FOLDER)?;
        fs::write(keyfile, self.to_str()).context("failed writing file")?;
        Ok(self)
    }
    fn from_file(keyfile: &str) -> Result<Self, AnyErr> {
        let content_str = fs::read_to_string(keyfile)?;
        Ok(Self::from_str(&content_str)?)
    }
    fn from_file_or_new(keyfile: &str) -> Result<Self, AnyErr> {
        let keyfile = format!("{}/{}", KEYS_FOLDER, keyfile);
        match Self::from_file(&keyfile) {
            Ok(identity) => Ok(identity),
            Err(_err) => {
                let newWallet = Self::generate();
                newWallet.to_file(&keyfile)?;
                Ok(newWallet)
            }
        }
    }
}
