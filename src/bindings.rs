use async_compat::Compat;
use core::fmt;
use fairy_vite::{ViteConfig, ViteError};
use std::{future::Future, path::Path, pin::Pin, sync::Arc};

pub use fairy_vite::{Asset, AssetKind};

pub async fn create(client: Arc<dyn HttpClient>, path: String) -> Result<Arc<Fairy>, FairyError> {
    async_compat::Compat::new(async move {
        let config = ViteConfig::load(Path::new(&*path)).await?;

        let fairy = fairy_vite::Fairy::new(config, WrappedHttp(client)).await?;

        Result::<_, ViteError>::Ok(Arc::new(Fairy {
            inner: Arc::new(fairy),
        }))
    })
    .await
    .map_err(|err| FairyError::Api {
        reason: err.to_string(),
    })
}

#[derive(Clone)]
struct WrappedHttp(Arc<dyn HttpClient>);

impl reggie::HttpClientFactory for WrappedHttp {
    type Client<B> = Self where
    B: reggie::http_body::Body + Send + 'static,
    B::Data: Into<reggie::bytes::Bytes> + Send,
    B::Error: Into<reggie::Error> + Send;

    fn create<B>(&self) -> Self::Client<B>
    where
        B: reggie::http_body::Body + Send + 'static,
        B::Data: Into<reggie::bytes::Bytes> + Send,
        B::Error: Into<reggie::Error> + Send,
    {
        self.clone()
    }
}

impl<B> reggie::HttpClient<B> for WrappedHttp
where
    B: reggie::http_body::Body + Send + 'static,
    B::Data: Into<reggie::bytes::Bytes> + Send,
    B::Error: Into<reggie::Error>,
{
    type Body = reggie::Body;

    type Future<'a> = Pin<
        Box<dyn Future<Output = Result<reggie::Response<reggie::Body>, reggie::Error>> + Send + 'a>,
    >;

    fn send<'a>(&'a self, request: reggie::http::Request<B>) -> Self::Future<'a> {
        Box::pin(Compat::new(async move {
            let (parts, body) = request.into_parts();

            use reggie::http_body_util::BodyExt;

            let bytes = BodyExt::collect(body)
                .await
                .map(|buf| buf.to_bytes())
                .map_err(Into::into)?;

            let req = Request {
                uri: parts.uri.to_string(),
                body: bytes.to_vec(),
            };

            let resp = self.0.send(req).await.unwrap();

            Ok(reggie::Response::builder()
                .status(resp.status)
                .body(reggie::Body::from(resp.body))
                .unwrap())
        }))
    }
}

pub enum Method {
    Get,
    Post,
    Put,
    Patch,
    Head,
    Delete,
}

#[derive(Debug)]
pub enum FairyError {
    Api { reason: String },
}

impl fmt::Display for FairyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Api { reason } => write!(f, "{reason}"),
        }
    }
}

impl std::error::Error for FairyError {}

#[derive(Clone)]
pub struct Fairy {
    inner: Arc<fairy_vite::Fairy>,
}

impl Fairy {
    pub fn renderer(&self, entry: Option<String>) -> Arc<Renderer> {
        Arc::new(Renderer {
            inner: self
                .inner
                .create_renderer(entry.as_ref().map(|m| m.as_str())),
        })
    }
}

pub struct Renderer {
    inner: fairy_vite::FairyRenderer,
}

impl Renderer {
    pub async fn render(&self, request: Request) -> Result<RenderResult, FairyError> {
        Compat::new(async move {
            let req = fairy_render::reggie::Request::builder()
                .method(reggie::Method::GET)
                .uri(request.uri)
                .body(reggie::Body::from(request.body))
                .unwrap();

            let result = self.inner.render(req).await?;

            Result::<_, fairy_vite::ViteError>::Ok(RenderResult {
                content: String::from_utf8(result.content).unwrap(),
                assets: result.assets,
                head: result.head,
            })
        })
        .await
        .map_err(|err| FairyError::Api {
            reason: err.to_string(),
        })
    }
}

pub struct Request {
    pub uri: String,
    pub body: Vec<u8>,
}

pub struct Response {
    pub status: u16,
    pub body: Vec<u8>,
}

pub struct RenderResult {
    pub content: String,
    pub assets: Vec<Asset>,
    pub head: Vec<String>,
}

#[async_trait::async_trait]
pub trait HttpClient: Send + Sync {
    async fn send(&self, req: Request) -> Result<Response, FairyError>;
}
