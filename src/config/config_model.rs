#[derive(Debug, Clone)]

pub struct Server(pub u16, pub u64, pub u64);

#[derive(Debug, Clone)]

pub struct Database {
    pub url: String,
}

#[derive(Debug, Clone)]

pub struct DotEnvyConfig {
    pub server: Server,
    pub database: Database,
    pub secret: String,
}

