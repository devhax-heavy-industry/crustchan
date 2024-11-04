use crate::auth::AuthnToken;
use crustchan::rejections::Unauthorized;
use warp::cookie;
use warp::{filters::BoxedFilter, Filter, Rejection, Reply};

pub fn authn() -> BoxedFilter<(impl Reply,)> {
    cookie_authn().boxed()
}

pub fn cookie_authn() -> impl Filter<Extract = (AuthnToken,), Error = Rejection> + Copy {
    //impl Filter<Extract = (AuthnToken,), Error = Rejection> + Copy {
    cookie("token").and_then(cookie_authn_step2)
}
async fn cookie_authn_step2(token_str: String) -> Result<AuthnToken, Rejection> {
    let token = AuthnToken::from_str(&token_str);
    let verified_token = match token {
        Ok(to_verify) => to_verify,
        Err(_) => {
            return Err(warp::reject::custom(Unauthorized));
        }
    };
    match verified_token.verify() {
        Ok(_) => Ok(verified_token),
        Err(_) => Err(warp::reject::custom(Unauthorized)),
    }
    // let verified_token = token.verify().map_err(|_| warp::reject::custom(Unauthorized)).unwrap();
    // Ok(verified_token)
}
