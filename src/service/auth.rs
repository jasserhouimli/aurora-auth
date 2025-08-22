use axum_extra::extract::cookie::{ Cookie, CookieJar, SameSite };
use chrono::{ Utc };
use time::{ Duration, OffsetDateTime };
pub fn set_auth_cookie(name: String, jar: CookieJar, token: String) -> CookieJar {
    let expires = OffsetDateTime::now_utc() + Duration::hours(1);
    let cookie = Cookie::build(Cookie::new(name, token))
        .http_only(true)
        .secure(true)
        .same_site(SameSite::Lax)
        .path("/")
        .expires(expires)
        .build();

    jar.add(cookie)
}
