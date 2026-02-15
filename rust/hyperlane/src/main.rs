use hyperlane::*;

fn init_server_config() -> ServerConfig {
    let mut server_config: ServerConfig = ServerConfig::default();
    server_config
        .set_address(Server::format_bind_address(DEFAULT_HOST, 3000))
        .set_nodelay(Some(false));
    server_config
}

fn init_request_config() -> RequestConfig {
    RequestConfig::low_security()
}

#[derive(Clone, Copy, Default)]
struct Index;

impl ServerHook for Index {
    async fn new(_: &mut Context) -> Self {
        Self
    }

    async fn handle(self, ctx: &mut Context) {
        ctx.get_mut_response().set_header(CONNECTION, KEEP_ALIVE);
        if ctx.get_request().get_method().is_get() {
            ctx.get_mut_response().set_status_code(200);
        } else {
            ctx.get_mut_response().set_status_code(404);
        }
        if ctx.try_send().await.is_err() {
            ctx.set_closed(true);
            return;
        }
        while ctx.http_from_stream().await.is_ok() {
            if ctx.get_request().get_method().is_get() {
                ctx.get_mut_response().set_status_code(200);
            } else {
                ctx.get_mut_response().set_status_code(404);
            }
            if ctx.try_send().await.is_err() {
                ctx.set_closed(true);
                return;
            }
        }
        ctx.set_closed(true);
    }
}

#[derive(Clone, Copy, Default)]
struct User;

impl ServerHook for User {
    async fn new(_: &mut Context) -> Self {
        Self
    }

    async fn handle(self, ctx: &mut Context) {
        ctx.get_mut_response().set_header(CONNECTION, KEEP_ALIVE);
        if ctx.get_request().get_method().is_post() {
            ctx.get_mut_response().set_status_code(200);
        } else {
            ctx.get_mut_response().set_status_code(404);
        }
        if ctx.try_send().await.is_err() {
            ctx.set_closed(true);
            return;
        }
        while ctx.http_from_stream().await.is_ok() {
            if ctx.get_request().get_method().is_post() {
                ctx.get_mut_response().set_status_code(200);
            } else {
                ctx.get_mut_response().set_status_code(404);
            }
            if ctx.try_send().await.is_err() {
                ctx.set_closed(true);
                return;
            }
        }
        ctx.set_closed(true);
    }
}

#[derive(Clone, Copy, Default)]
struct UserId;

impl ServerHook for UserId {
    async fn new(_: &mut Context) -> Self {
        Self
    }

    async fn handle(self, ctx: &mut Context) {
        ctx.get_mut_response().set_header(CONNECTION, KEEP_ALIVE);
        if ctx.get_request().get_method().is_get() {
            let id: String = ctx.try_get_route_param("id").unwrap_or_default();
            ctx.get_mut_response().set_status_code(200).set_body(id);
        } else {
            ctx.get_mut_response().set_status_code(404);
        }
        if ctx.try_send().await.is_err() {
            ctx.set_closed(true);
            return;
        }
        while ctx.http_from_stream().await.is_ok() {
            if ctx.get_request().get_method().is_get() {
                let id: String = ctx.try_get_route_param("id").unwrap_or_default();
                ctx.get_mut_response().set_status_code(200).set_body(id);
            } else {
                ctx.get_mut_response().set_status_code(404);
            }
            if ctx.try_send().await.is_err() {
                ctx.set_closed(true);
                return;
            }
        }
        ctx.set_closed(true);
    }
}

#[tokio::main]
async fn main() {
    Server::default()
        .server_config(init_server_config())
        .request_config(init_request_config())
        .route::<Index>("/")
        .route::<User>("/user")
        .route::<UserId>("/user/{id}")
        .run()
        .await
        .unwrap()
        .wait()
        .await;
}
