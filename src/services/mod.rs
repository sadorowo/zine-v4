use poise::serenity_prelude::Context;
use std::future::Future;
use std::time::Duration;
use tokio::{task, time};

mod temp_bans;
mod temp_mutes;

pub fn start_services(ctx: Context, db: mongodb::Database) -> Vec<Service> {
    vec![
        Service::with(ctx.clone(), db.clone()).run("temp_bans", 60, temp_bans::listen),
        Service::with(ctx.clone(), db.clone()).run("temp_mutes", 60, temp_mutes::listen),
    ]
}

#[derive(Clone)]
pub struct Service {
    pub name: &'static str,
    pub interval: u64,
    pub enabled: bool,
    pub started_date: Option<chrono::DateTime<chrono::Utc>>,
    ctx: Option<Context>,
    db: Option<mongodb::Database>,
}

impl Service {
    pub fn with(ctx: Context, db: mongodb::Database) -> Self {
        Self {
            name: "",
            interval: 0,
            enabled: false,
            started_date: None,
            ctx: Some(ctx),
            db: Some(db),
        }
    }

    pub fn run<F, Fut>(&mut self, name: &'static str, interval: u64, f: F) -> Self
        where
            F: Fn(Context, mongodb::Database) -> Fut + Send + Sync + 'static,
            Fut: Future<Output = ()> + Send + 'static,
    {
        println!("starting service: {}", name);
        self.name = name;
        self.enabled = true;
        self.interval = interval;
        self.started_date = Some(chrono::Utc::now());

        let cloned_self = self.clone();
        task::spawn(async move {
            loop {
                if !cloned_self.enabled {
                    continue;
                }

                f(cloned_self.ctx.as_ref().unwrap().clone(), cloned_self.db.as_ref().unwrap().clone()).await;
                time::sleep(Duration::from_secs(cloned_self.interval)).await;
            }
        });

        self.clone()
    }


    pub fn pause(&mut self) {
        self.enabled = false;
    }

    pub fn resume(&mut self) {
        self.enabled = true;
    }

    pub fn is_running(&mut self) -> bool {
        self.enabled
    }
}