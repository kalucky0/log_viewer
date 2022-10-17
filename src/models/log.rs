#[derive(Debug)]
pub struct Log {
    ip: String,
    timestamp: String,
    method: String,
    path: String,
    status: String,
    referer: String,
    user_agent: String,
}

impl Log {
    pub fn new(ip: String, timestamp: String, method: String, path: String, status: String, referer: String, user_agent: String) -> Self {
        Log {
            ip,
            timestamp,
            method,
            path,
            status,
            referer,
            user_agent,
        }
    }

    pub fn to_slice(&self) -> [&str; 7] {
        [self.ip(), self.timestamp(), self.method(), self.path(), self.status(), self.referer(), self.user_agent()]
    }

    pub fn ip(&self) -> &str {
        self.ip.as_str()
    }

    pub fn timestamp(&self) -> &str {
        self.timestamp.as_str()
    }

    pub fn method(&self) -> &str {
        self.method.as_str()
    }

    pub fn path(&self) -> &str {
        self.path.as_str()
    }

    pub fn status(&self) -> &str {
        self.status.as_str()
    }

    pub fn referer(&self) -> &str {
        self.referer.as_str()
    }

    pub fn user_agent(&self) -> &str {
        self.user_agent.as_str()
    }
}