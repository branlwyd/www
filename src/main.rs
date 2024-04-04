use axum::{
    extract::Request,
    handler::Handler,
    http::{
        header::{
            CONTENT_SECURITY_POLICY, CONTENT_TYPE, ETAG, IF_NONE_MATCH, REFERRER_POLICY,
            STRICT_TRANSPORT_SECURITY, X_CONTENT_TYPE_OPTIONS, X_FRAME_OPTIONS, X_XSS_PROTECTION,
        },
        HeaderValue, StatusCode,
    },
    response::{IntoResponse, Response},
    routing::get,
    Router,
};
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use bytes::Bytes;
use ring::digest::{digest, SHA512_256};
use rust_embed::RustEmbed;
use std::{borrow::Cow, future, net::Ipv4Addr, sync::Arc, time::Duration};
use tower::ServiceBuilder;
use tower_http::{
    set_header::SetResponseHeaderLayer,
    timeout::TimeoutLayer,
    trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer},
};
use tracing::{info, Level};

#[derive(RustEmbed)]
#[folder = "$OUT_DIR/assets"]
struct Asset;

/// StaticHandler serves a static response based on given content type & content.
#[derive(Clone)]
struct StaticHandler {
    content_type: &'static str,
    etag: Arc<String>,
    content: Bytes,
}

impl StaticHandler {
    /// Creates a new static handler based on the given content type & content.
    fn new(content_type: &'static str, content: Bytes) -> Self {
        let etag = Arc::new(URL_SAFE_NO_PAD.encode(digest(&SHA512_256, &content)));
        Self {
            content_type,
            etag,
            content,
        }
    }

    /// Creates a static handler serving a given embedded asset, given the content type & asset
    /// name.
    ///
    /// # Panics
    ///
    /// This function panics if the asset cannot be found.
    fn from_asset(content_type: &'static str, asset: &str) -> Self {
        let file =
            Asset::get(asset).unwrap_or_else(|| panic!("Couldn't find static asset {asset}"));
        let content = match file.data {
            Cow::Borrowed(data) => Bytes::from_static(data),
            Cow::Owned(data) => Bytes::from(data),
        };
        Self::new(content_type, content)
    }
}

impl<S> Handler<(), S> for StaticHandler {
    type Future = future::Ready<Response>;

    fn call(self, req: Request, _: S) -> Self::Future {
        // Check if the result is cached, and return NOT_MODIFIED if so.
        if req.headers().get(IF_NONE_MATCH).map(HeaderValue::as_bytes) == Some(self.etag.as_bytes())
        {
            return future::ready(
                ([(ETAG, self.etag.as_ref())], StatusCode::NOT_MODIFIED).into_response(),
            );
        }

        // Otherwise, return the full response.
        future::ready(
            (
                [(CONTENT_TYPE, self.content_type), (ETAG, &self.etag)],
                self.content,
            )
                .into_response(),
        )
    }
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .event_format(
            tracing_subscriber::fmt::format()
                .with_target(false)
                .compact(),
        )
        .init();

    let app = Router::new()
        .route(
            "/",
            get(StaticHandler::from_asset(
                "text/html; charset=utf-8",
                "pages/index.html",
            )),
        )
        .route(
            "/style.css",
            get(StaticHandler::from_asset(
                "text/css; charset=utf-8",
                "static/style.css",
            )),
        )
        .route(
            "/favicon.ico",
            get(StaticHandler::from_asset("image/x-icon", "static/favicon.ico")),
        )
        .route(
            "/resume.pdf",
            get(StaticHandler::from_asset("application/pdf", "static/resume.pdf")),
        )
        .layer(
            ServiceBuilder::new()
                .layer(
                    TraceLayer::new_for_http()
                        .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
                        .on_response(DefaultOnResponse::new().level(Level::INFO)),
                )
                .layer(TimeoutLayer::new(Duration::from_secs(10)))
                .layer(SetResponseHeaderLayer::overriding(
                    STRICT_TRANSPORT_SECURITY,
                    HeaderValue::from_static("max-age=31536000; includeSubDomains; preload"),
                ))
                .layer(SetResponseHeaderLayer::overriding(
                    REFERRER_POLICY,
                    HeaderValue::from_static("no-referrer"),
                ))
                .layer(SetResponseHeaderLayer::overriding(
                    CONTENT_SECURITY_POLICY,
                    HeaderValue::from_static("default-src 'self'; style-src 'self' https://fonts.googleapis.com; font-src https://fonts.gstatic.com"),
                ))
                .layer(SetResponseHeaderLayer::overriding(
                    X_FRAME_OPTIONS,
                    HeaderValue::from_static("DENY"),
                ))
                .layer(SetResponseHeaderLayer::overriding(
                    X_XSS_PROTECTION,
                    HeaderValue::from_static("1; mode=block"),
                ))
                .layer(SetResponseHeaderLayer::overriding(
                    X_CONTENT_TYPE_OPTIONS,
                    HeaderValue::from_static("nosniff"),
                ))
        );

    #[cfg(feature = "test")]
    {
        use tokio::net::TcpListener;

        let listener = TcpListener::bind((Ipv4Addr::UNSPECIFIED, 8080))
            .await
            .expect("Couldn't listen on port 8080");
        info!("serving HTTP on port 8080");
        axum::serve(listener, app).await.unwrap();
    }

    #[cfg(not(feature = "test"))]
    {
        use rustls_acme::{caches::DirCache, AcmeConfig};
        use std::net::TcpListener;
        use tokio_stream::StreamExt as _;
        use tracing::error;

        let mut state = AcmeConfig::new(Vec::from(["bran.land"]))
            .contact(Vec::from(["mailto:bran@bran.land"]))
            .cache(DirCache::new("/home/www/certs"))
            .directory_lets_encrypt(true)
            .state();
        let mut config = match Arc::try_unwrap(state.default_rustls_config()) {
            Ok(config) => config,
            Err(arc_config) => arc_config.as_ref().clone(),
        };
        config.alpn_protocols = Vec::from([
            "h2".as_bytes().to_vec(),
            "http/1.1".as_bytes().to_vec(),
        ]);
        let acceptor = state.axum_acceptor(Arc::new(config));

        tokio::spawn(async move {
            loop {
                match state.next().await.unwrap() {
                    Ok(event) => info!("ACME event: {:?}", event),
                    Err(err) => error!("ACME error: {:?}", err),
                }
            }
        });

        let listener =
            TcpListener::bind((Ipv4Addr::UNSPECIFIED, 443)).expect("Couldn't listen on port 443");
        info!("serving HTTPS on port 443");
        axum_server::from_tcp(listener)
            .acceptor(acceptor)
            .serve(app.into_make_service())
            .await
            .unwrap();
    }
}
