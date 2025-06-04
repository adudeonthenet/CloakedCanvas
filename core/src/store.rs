/*
 * CloakedCanvas - MIT License
 */
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use reqwest::blocking::Client;
use std::path::{Path, PathBuf};
use tokio::sync::OnceCell;

pub trait VaultStore {
    fn put(&self, local_file: &Path) -> Result<String, Box<dyn std::error::Error + Send + Sync>>;
}

pub struct LocalDisk {
    pub dir: PathBuf,
    pub port: u16,
    server: OnceCell<()>,
}

impl LocalDisk {
    pub fn new(dir: PathBuf, port: u16) -> Self {
        Self {
            dir,
            port,
            server: OnceCell::const_new(),
        }
    }

    async fn ensure_server(&self) {
        if self.server.get().is_none() {
            let dir = self.dir.clone();
            let make_svc = make_service_fn(move |_| {
                let dir = dir.clone();
                async move {
                    Ok::<_, hyper::Error>(service_fn(move |req: Request<Body>| {
                        let path = dir.join(req.uri().path().trim_start_matches('/'));
                        async move {
                            match tokio::fs::read(path).await {
                                Ok(bytes) => {
                                    Ok::<_, hyper::Error>(Response::new(Body::from(bytes)))
                                }
                                Err(_) => Ok::<_, hyper::Error>(
                                    Response::builder().status(404).body(Body::empty()).unwrap(),
                                ),
                            }
                        }
                    }))
                }
            });
            let addr = ([127, 0, 0, 1], self.port).into();
            tokio::spawn(Server::bind(&addr).serve(make_svc));
            self.server.set(()).ok();
        }
    }
}

impl VaultStore for LocalDisk {
    fn put(&self, local_file: &Path) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        std::fs::create_dir_all(&self.dir)?;
        let dest = self.dir.join(local_file.file_name().ok_or("bad name")?);
        std::fs::copy(local_file, &dest)?;
        let rt = tokio::runtime::Runtime::new()?;
        rt.block_on(self.ensure_server());
        Ok(format!(
            "http://localhost:{}/{}",
            self.port,
            dest.file_name().unwrap().to_string_lossy()
        ))
    }
}

pub struct S3 {
    pub bucket: String,
    client: Client,
}

impl S3 {
    pub fn new(bucket: String) -> Self {
        Self {
            bucket,
            client: Client::new(),
        }
    }
}

impl VaultStore for S3 {
    fn put(&self, local_file: &Path) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let key = local_file.file_name().ok_or("bad name")?.to_string_lossy();
        let url = format!("https://{}.s3.amazonaws.com/{}", self.bucket, key);
        let data = std::fs::read(local_file)?;
        let resp = self.client.put(&url).body(data).send()?;
        if resp.status().is_success() {
            Ok(url)
        } else {
            Err("upload failed".into())
        }
    }
}

pub struct GoogleDrive {
    pub endpoint: String,
    client: Client,
}

impl GoogleDrive {
    pub fn new(endpoint: String) -> Self {
        Self {
            endpoint,
            client: Client::new(),
        }
    }
}

impl VaultStore for GoogleDrive {
    fn put(&self, local_file: &Path) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let data = std::fs::read(local_file)?;
        let resp = self.client.post(&self.endpoint).body(data).send()?;
        if resp.status().is_success() {
            Ok(self.endpoint.clone())
        } else {
            Err("upload failed".into())
        }
    }
}

pub struct Dropbox {
    pub endpoint: String,
    client: Client,
}

impl Dropbox {
    pub fn new(endpoint: String) -> Self {
        Self {
            endpoint,
            client: Client::new(),
        }
    }
}

impl VaultStore for Dropbox {
    fn put(&self, local_file: &Path) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let data = std::fs::read(local_file)?;
        let resp = self.client.post(&self.endpoint).body(data).send()?;
        if resp.status().is_success() {
            Ok(self.endpoint.clone())
        } else {
            Err("upload failed".into())
        }
    }
}
