use std::thread;
use std::thread::JoinHandle;

use axum::Router;
use axum::routing::get;
use log::info;
use tokio::runtime;

use crate::{ApiSettings, Response};

pub struct Api {
    settings: ApiSettings,
}

async fn hello() ->  String{
    // Send the protected data to the user
    String::from("hello")
}

impl Api {
    pub fn new(settings: ApiSettings) -> Api {
        Api { settings }
    }

    pub fn start(&mut self) -> anyhow::Result<JoinHandle<()>> {
        let server_thread = thread::Builder::new().name(String::from("api"));
        let server_url = self.settings.listen.clone();
        let handle = server_thread.spawn(move || {
            let mut runtime = runtime::Builder::new_current_thread();
            let runtime = runtime.enable_all().build().unwrap();
            runtime.block_on(async {

                let app = Router::new()
                    .route("/test", get(hello));
                let listener = tokio::net::TcpListener::bind(&server_url).await.unwrap();
                info!("start api server on {}", server_url);
                axum::serve(listener, app).await.unwrap();
            });
        })?;
        Ok(handle)
    }

    // {
    //       userId: '1',
    //       username: 'vben',
    //       realName: 'Vben Admin',
    //       avatar: '',
    //       desc: 'manager',
    //       password: '123456',
    //       token: 'fakeToken1',
    //       homePath: '/dashboard/analysis',
    //       roles: [
    //         {
    //           roleName: 'Super Admin',
    //           value: 'super',
    //         },
    //       ],
    //     },


}