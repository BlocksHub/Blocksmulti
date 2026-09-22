use std::{
    sync::{RwLock, RwLockReadGuard},
    time::{Duration, SystemTime},
};

use serde::{de::DeserializeOwned, Serialize};
use url::Url;

use crate::{
    constants::USER_AGENT,
    http::{endpoints::Endpoint, errors::HttpError},
    session::Session,
};

pub struct HttpManager {
    transport: reqwest::Client,
    base_url: Url,
    session: RwLock<Session>,
}

impl HttpManager {
    pub(crate) fn transport() -> Result<reqwest::Client, HttpError> {
        reqwest::ClientBuilder::new()
            .user_agent(USER_AGENT)
            .connect_timeout(Duration::from_secs(10))
            .timeout(Duration::from_secs(30))
            .build()
            .map_err(HttpError::Request)
    }

    pub(crate) fn from_token(
        transport: reqwest::Client,
        base_url: Url,
        access_token: String,
    ) -> Self {
        Self {
            transport,
            base_url,
            session: RwLock::new(Session {
                access_token,
                expire_at: SystemTime::now() + Duration::from_secs(5 * 3600),
            }),
        }
    }

    fn session(&self) -> Result<RwLockReadGuard<'_, Session>, HttpError> {
        self.session.read().map_err(|_| HttpError::PoisonedSession)
    }

    pub(crate) fn access_token(&self) -> Result<String, HttpError> {
        let session = self.session()?;
        if session.is_expired() {
            return Err(HttpError::Unauthenticated);
        };

        Ok(session.access_token.clone())
    }

    pub(crate) async fn post_anonymous<T, K>(
        transport: &reqwest::Client,
        base_url: &Url,
        endpoint: Endpoint,
        payload: Option<T>,
    ) -> Result<K, HttpError>
    where
        T: Serialize,
        K: DeserializeOwned,
    {
        let request =
            Self::build_request(transport, base_url, reqwest::Method::POST, endpoint, None, payload)?;
        Self::send::<K>(request).await
    }

    pub(crate) async fn get<K: DeserializeOwned>(
        &self,
        endpoint: Endpoint,
    ) -> Result<K, HttpError> {
        let token = if endpoint.require_auth() {
            Some(self.access_token()?)
        } else {
            None
        };
        let request = Self::build_request(
            &self.transport,
            &self.base_url,
            reqwest::Method::GET,
            endpoint,
            token,
            None::<()>,
        )?;
        Self::send::<K>(request).await
    }

    pub(crate) async fn get_query<K: DeserializeOwned>(
        &self,
        endpoint: Endpoint,
        params: &[(&str, &str)],
    ) -> Result<K, HttpError> {
        let token = if endpoint.require_auth() {
            Some(self.access_token()?)
        } else {
            None
        };
        let mut url = endpoint.url(&self.base_url)?;
        {
            let mut query_pairs = url.query_pairs_mut();
            for (k, v) in params {
                query_pairs.append_pair(k, v);
            }
        }
        let mut request = self.transport.request(reqwest::Method::GET, url);
        if let Some(token) = token {
            request = request.bearer_auth(token);
        }
        Self::send::<K>(request).await
    }

    pub(crate) async fn post<K: DeserializeOwned, T: Serialize>(
        &self,
        endpoint: Endpoint,
        payload: Option<T>,
    ) -> Result<K, HttpError> {
        let token = if endpoint.require_auth() {
            Some(self.access_token()?)
        } else {
            None
        };
        let request = Self::build_request(
            &self.transport,
            &self.base_url,
            reqwest::Method::POST,
            endpoint,
            token,
            payload,
        )?;
        Self::send::<K>(request).await
    }

    pub(crate) async fn post_text<T: Serialize>(
        &self,
        endpoint: Endpoint,
        payload: Option<T>,
    ) -> Result<String, HttpError> {
        let token = if endpoint.require_auth() {
            Some(self.access_token()?)
        } else {
            None
        };
        let request = Self::build_request(
            &self.transport,
            &self.base_url,
            reqwest::Method::POST,
            endpoint,
            token,
            payload,
        )?;
        Self::send_text(request).await
    }

    pub(crate) async fn delete<K: DeserializeOwned, T: Serialize>(
        &self,
        endpoint: Endpoint,
        payload: Option<T>,
    ) -> Result<K, HttpError> {
        let token = if endpoint.require_auth() {
            Some(self.access_token()?)
        } else {
            None
        };
        let request = Self::build_request(
            &self.transport,
            &self.base_url,
            reqwest::Method::DELETE,
            endpoint,
            token,
            payload,
        )?;
        Self::send::<K>(request).await
    }

    #[allow(dead_code)]
    pub(crate) async fn patch<K: DeserializeOwned, T: Serialize>(
        &self,
        endpoint: Endpoint,
        payload: Option<T>,
    ) -> Result<K, HttpError> {
        let token = if endpoint.require_auth() {
            Some(self.access_token()?)
        } else {
            None
        };
        let request = Self::build_request(
            &self.transport,
            &self.base_url,
            reqwest::Method::PATCH,
            endpoint,
            token,
            payload,
        )?;
        Self::send::<K>(request).await
    }

    async fn send<K>(request: reqwest::RequestBuilder) -> Result<K, HttpError>
    where
        K: DeserializeOwned,
    {
        request
            .send()
            .await
            .map_err(HttpError::Request)?
            .error_for_status()
            .map_err(HttpError::Request)?
            .json()
            .await
            .map_err(HttpError::Request)
    }

    async fn send_text(request: reqwest::RequestBuilder) -> Result<String, HttpError> {
        request
            .send()
            .await
            .map_err(HttpError::Request)?
            .error_for_status()
            .map_err(HttpError::Request)?
            .text()
            .await
            .map_err(HttpError::Request)
    }

    pub(crate) fn build_request<T>(
        transport: &reqwest::Client,
        base_url: &Url,
        method: reqwest::Method,
        endpoint: Endpoint,
        token: Option<String>,
        payload: Option<T>,
    ) -> Result<reqwest::RequestBuilder, HttpError>
    where
        T: Serialize,
    {
        let mut request = transport.request(method, endpoint.url(base_url)?);
        if let Some(token) = token {
            request = request.bearer_auth(token);
        };

        if let Some(body) = payload {
            request = request.json(&body);
        }

        Ok(request)
    }
}
