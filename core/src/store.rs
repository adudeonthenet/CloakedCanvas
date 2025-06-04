/*
 * CloakedCanvas - MIT License
 */
use std::path::{Path, PathBuf};
use hyper::{Body, Request, Response, Server};
use hyper::service::{make_service_fn, service_fn};
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
        Self { dir, port, server: OnceCell::const_new() }
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
                                Ok(bytes) => Ok::<_, hyper::Error>(Response::new(Body::from(bytes))),
                                Err(_) => Ok::<_, hyper::Error>(Response::builder().status(404).body(Body::empty()).unwrap()),
                            }
                        }
                    }))
                }
            });
            let addr = ([127,0,0,1], self.port).into();
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
        Ok(format!("http://localhost:{}/{}", self.port, dest.file_name().unwrap().to_string_lossy()))
    }
}
