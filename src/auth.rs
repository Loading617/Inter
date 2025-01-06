pub async fn login(credentials: Credentials) -> Result<Token, Error> {
    // Validate user credentials
    let user = find_user_in_db(&credentials).await?;
    if verify_password(&user.password, &credentials.password)? {
        let token = generate_jwt(&user)?;
        Ok(token)
    } else {
        Err(Error::Unauthorized)
    }
}