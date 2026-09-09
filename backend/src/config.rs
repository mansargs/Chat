use std::env;

#[derive(Clone, Debug)]
pub struct Config {
    pub database: DbConfig,
    pub jwt: JwtConfig,
    pub server: ServerConfig,
}

#[derive(Clone, Debug)]
pub struct DbConfig {
    pub db_url: String,
    pub max_connections: u32,
    pub min_connections: u32,
}

#[derive(Clone, Debug)]
pub struct JwtConfig {
    pub jwt_access_secret: String,
    pub jwt_refresh_secret: String,
    pub jwt_access_expiration: u32,
    pub jwt_refresh_expiration: u32,
}

#[derive(Clone, Debug)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

impl ServerConfig {
    pub fn load() -> Result<Self, String> {
        let host = env::var("SERVER_HOST")
            .unwrap_or_else(|_| "0.0.0.0".to_string());

        let port = env::var("SERVER_PORT")
            .unwrap_or_else(|_| "6000".to_string())
            .parse::<u16>()
            .map_err(|_| "SERVER_PORT must be a valid number".to_string())?;

        Ok(Self { host, port })
    }
}

fn parse_duration(value: &str) -> Result<u32, String> {
    if value.len() < 2 {
        return Err(format!("Invalid duration: {value}"));
    }

    let (number, unit) = value.split_at(value.len() - 1);

    let number = number
        .parse::<u32>()
        .map_err(|_| format!("Invalid duration number: {number}"))?;

    let seconds = match unit {
        "m" => number
            .checked_mul(60)
            .ok_or_else(|| format!("Duration is too large: {value}"))?,

        "h" => number
            .checked_mul(60 * 60)
            .ok_or_else(|| format!("Duration is too large: {value}"))?,

        "d" => number
            .checked_mul(60 * 60 * 24)
            .ok_or_else(|| format!("Duration is too large: {value}"))?,

        _ => {
            return Err(
                format!("Invalid duration unit in '{value}'. Use m, h, or d")
            );
        }
    };

    Ok(seconds)
}

impl JwtConfig {
    pub fn load() -> Result<Self, String> {
        let jwt_access_secret = env::var("JWT_ACCESS_SECRET")
            .map_err(|_| "JWT_ACCESS_SECRET must be set".to_string())?;

        let jwt_refresh_secret = env::var("JWT_REFRESH_SECRET")
            .map_err(|_| "JWT_REFRESH_SECRET must be set".to_string())?;

        let access_expiration = env::var("JWT_ACCESS_EXPIRATION")
            .map_err(|_| "JWT_ACCESS_EXPIRATION must be set".to_string())?;

        let refresh_expiration = env::var("JWT_REFRESH_EXPIRATION")
            .map_err(|_| "JWT_REFRESH_EXPIRATION must be set".to_string())?;

        let jwt_access_expiration = parse_duration(&access_expiration)?;
        let jwt_refresh_expiration = parse_duration(&refresh_expiration)?;

        Ok(Self {
            jwt_access_secret,
            jwt_refresh_secret,
            jwt_access_expiration,
            jwt_refresh_expiration,
        })
    }
}

impl DbConfig {
    pub fn load() -> Result<Self, String> {
        let max_connections = env::var("DB_MAX_CONNECTIONS")
            .unwrap_or_else(|_| "10".to_string())
            .parse::<u32>()
            .map_err(|_| {
                "DB_MAX_CONNECTIONS must be a valid number".to_string()
            })?;

        let min_connections = env::var("DB_MIN_CONNECTIONS")
            .unwrap_or_else(|_| "2".to_string())
            .parse::<u32>()
            .map_err(|_| {
                "DB_MIN_CONNECTIONS must be a valid number".to_string()
            })?;

        if min_connections > max_connections {
            return Err(
                "DB_MIN_CONNECTIONS cannot be greater than DB_MAX_CONNECTIONS"
                    .to_string(),
            );
        }

        let db_url = env::var("DATABASE_URL")
            .map_err(|_| "DATABASE_URL must be set".to_string())?;

        Ok(Self {
            db_url,
            max_connections,
            min_connections,
        })
    }
}

impl Config {
    pub fn load() -> Result<Self, String> {
        Ok(Self {
            database: DbConfig::load()?,
            jwt: JwtConfig::load()?,
            server: ServerConfig::load()?,
        })
    }
}