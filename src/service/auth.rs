use axum_extra::extract::cookie::{ Cookie, CookieJar, SameSite };

pub fn set_auth_cookie(name: String, jar: CookieJar, token: String) -> CookieJar {
    let cookie = Cookie::build(Cookie::new(name, token))
        .http_only(true)
        .secure(true)
        .same_site(SameSite::Lax)
        .path("/")
        .build();

    jar.add(cookie)
}
