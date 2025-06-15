mod sensors;
mod types;

use axum::extract::ws::{Message, WebSocket};
use axum::extract::{ConnectInfo, State, WebSocketUpgrade};
use axum::{response::IntoResponse, routing::get, Router};
use axum_extra::TypedHeader;
use clap::Parser;
use evalexpr::{ContextWithMutableVariables, HashMapContext};
use sensors::{SensorDef, SensorInstance};
use serde::{Deserialize, Serialize, Serializer};
use std::collections::{HashMap, VecDeque};
use std::net::{IpAddr, SocketAddr};
use std::ops::ControlFlow;
use std::path::PathBuf;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tokio::sync::mpsc;
use tokio::sync::watch::{self};
use tower_http::services::{ServeDir, ServeFile};
use tower_http::trace::{DefaultMakeSpan, TraceLayer};
use types::Temp;

// define our configuration file
#[derive(Debug, Serialize, Deserialize, Clone)]
struct Config {
    /// A map of temperature sensor names and their locations
    #[serde(default)]
    temp_sensors: HashMap<String, SensorDef>,
    /// The location of the fan
    #[serde(skip_serializing_if = "Option::is_none")]
    fan_location: Option<String>,
    /// Poll rate in seconds
    #[serde(default = "default_poll_rate")]
    poll_rate: u64,
    /// alarms to trigger when temperatures reach a certain value
    #[serde(default = "HashMap::new")]
    alarms: HashMap<String, types::ParseableExpr>,
    /// length of the history array in seconds
    #[serde(default = "default_history_size")]
    history_size: u64,
}

// default to 15 hours. that should be plenty for most meats
fn default_history_size() -> u64 {
    15 * 60 * 60
}

fn default_poll_rate() -> u64 {
    5
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct SensorData {
    #[serde(serialize_with = "serialize_time")]
    ts: SystemTime,
    /// a map of temp probe names to temperatures in celsius
    temps: HashMap<String, Temp>,
}

fn serialize_time<S>(data: &SystemTime, ser: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    data.duration_since(UNIX_EPOCH)
        .expect("clock went backwards")
        .as_millis()
        .serialize(ser)
}

// we set up command line options with clap
#[derive(Parser, Debug, Clone)]
#[clap(name = "server", about = "A Raspberry Pi smoker controller")]
struct Opt {
    /// the config file
    #[clap(short = 'c', long = "config", default_value = "./config.json", value_hint = clap::ValueHint::FilePath)]
    config_file: std::path::PathBuf,

    /// control the log level
    #[clap(short = 'l', long = "log", default_value = "debug")]
    log_level: String,

    /// the listen address for the web server
    #[clap(short = 'a', long = "addr", default_value = "::1")]
    addr: IpAddr,

    /// the listen port
    #[clap(short = 'p', long = "port", default_value = "8080")]
    port: u16,

    /// the directory containing html files
    #[clap(long = "static-dir", default_value = "./public", value_hint = clap::ValueHint::DirPath)]
    static_dir: std::path::PathBuf,
}

#[derive(Debug, Clone, Serialize)]
struct TickData {
    history: VecDeque<SensorData>,
    active_alarms: Vec<String>,
}

#[derive(Debug, Serialize)]
struct InitialMessage<'a> {
    tick: &'a TickData,
    cfg: &'a Config,
}
#[derive(Debug, Serialize)]
struct UpdateMessage<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    hist_upd: Option<&'a SensorData>,
    active_alarms: &'a Vec<String>,
}
impl TickData {
    pub fn new() -> Self {
        TickData {
            history: VecDeque::new(),
            active_alarms: vec![],
        }
    }
}

#[derive(Debug, Clone)]
struct AppState {
    rx_data: watch::Receiver<TickData>,
    tx_cfg: mpsc::Sender<Config>,
    cfg: Config,
}

fn get_state(opt: &Opt) -> AppState {
    let cfg_file = std::fs::File::open(&opt.config_file).expect("config file should be readable");
    let cfg: Config = serde_json::from_reader(cfg_file).expect("config file should be parsable");
    return start_sensor_poll(cfg);
}

fn make_sensor_instances(
    temp_sensors: &HashMap<String, SensorDef>,
) -> HashMap<String, Box<dyn SensorInstance + Send>> {
    HashMap::from_iter(temp_sensors.iter().flat_map(|(k, v)| {
        let inst = v.to_instance();
        match inst {
            Err(e) => {
                log::error!("Failed to load sensor driver {k}: {}", e);
                vec![]
            }
            Ok(v) => vec![(k.to_owned(), v)],
        }
    }))
}

fn start_sensor_poll(cfg: Config) -> AppState {
    let (tx_cfg, rx_cfg) = mpsc::channel(100);
    let (tx, rx) = watch::channel(TickData::new());
    // don't await this, since we'll block until it returns
    let cloned_cfg = cfg.clone();
    tokio::task::spawn(async move { poll_loop(cloned_cfg, rx_cfg, tx).await });

    return AppState {
        rx_data: rx,
        tx_cfg,
        cfg,
    };
}

fn truncate_history(history: &mut VecDeque<SensorData>, hist_size: u64) {
    let last = history.back();
    if last.is_none() {
        return;
    }
    // the minimum possible timestamp
    let duration = Duration::from_secs(hist_size);
    let ts = last.unwrap().ts - duration;
    let mut found_idx: Option<usize> = None;
    for (idx, val) in history.iter().enumerate() {
        if val.ts >= ts {
            found_idx = Some(idx);
            break;
        }
    }
    match found_idx {
        None => {
            history.clear();
        }
        Some(idx) => {
            history.drain(0..idx);
        }
    };
}

async fn poll_loop(
    mut cfg: Config,
    mut rx_cfg: mpsc::Receiver<Config>,
    tx_data: watch::Sender<TickData>,
) {
    let mut interval = tokio::time::interval(Duration::from_secs(cfg.poll_rate));
    let mut sensors = make_sensor_instances(&cfg.temp_sensors);
    let mut history = VecDeque::<SensorData>::new();

    loop {
        tokio::select! {
            _ = interval.tick() => {
                let mut data = SensorData {
                    ts: SystemTime::now(),
                    temps: HashMap::new(),
                };

                let mut ctx = HashMapContext::new();
                for (key, inst) in sensors.iter_mut() {
                    let val: sensors::Result<Temp> = inst.poll(&data.ts);

                    // we always need to set a value in the context so that expressions that depend on multiple
                    // sensor readings still work. There's no good way to indicate a missing value, so we use 0.0
                    // setting a value shouldn't fail. the only cause of failure is if an old value exists with a different
                    // type. we're always inserting floats into an empty map
                    ctx.set_value(key.to_string(), evalexpr::Value::Float(*val.as_ref().unwrap_or(&0.0))).expect("insertion shouldn't fail");
                    match val {
                        Err(e) => {
                            log::error!("error polling sensor {key}: {}", e);
                        },
                        Ok(val) => {
                            log::debug!("adding data `{key}` ({:?}, {})", data.ts, val);
                            data.temps.insert(key.to_string(), val);
                        }
                    }
                }

                // now check alarms
                let mut active_alarms: Vec<String> = vec![];
                for (name, def) in cfg.alarms.iter() {
                    let result = def.data.eval_boolean_with_context(&mut ctx);
                    match result {
                        Err(e) => {
                            log::error!("alarm {name} was invalid: {}", e);
                        }
                        Ok(active) => {
                            if active {
                                active_alarms.push(name.to_string());
                            }
                        }
                    }
                }

                history.push_back(data);
                truncate_history(&mut history, cfg.history_size);
                tx_data.send(TickData{ history: history.clone(), active_alarms }).unwrap();
                log::debug!("ticked poll loop");
            }
            // new_cfg = rx_cfg.recv() => {
            //     match new_cfg {
            //         Some(new_cfg) => {
            //             if new_cfg.poll_rate != cfg.poll_rate {
            //                 interval = tokio::time::interval(Duration::from_millis(new_cfg.poll_rate));
            //             }
            //             if new_cfg.temp_sensors != cfg.temp_sensors {
            //                 sensors = make_sensor_instances(&new_cfg.temp_sensors);
            //             }
            //             cfg = new_cfg
            //         },
            //         None => {
            //             log::error!("Failed to receive config change");
            //         }
            //     }
            // }
        }
    }
}

#[tokio::main]
async fn main() {
    let opt = Opt::parse();
    let state = get_state(&opt);

    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", format!("{},hyper=info,mio=info", opt.log_level))
    }
    tracing_subscriber::fmt::init();

    // to support a SPA, we create a service that serves the static directory
    // and always serves the index.html when a file isn't found
    let static_service = ServeDir::new(&opt.static_dir).not_found_service(ServeFile::new(
        PathBuf::from(&opt.static_dir).join("index.html"),
    ));
    let app = Router::new()
        .route("/ws", get(ws_handler))
        .with_state(state)
        .fallback_service(static_service)
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::default().include_headers(true)),
        );

    let sock_addr = SocketAddr::from((opt.addr, opt.port));
    let listener = tokio::net::TcpListener::bind(sock_addr).await.unwrap();

    log::info!(
        "server is listening on http://{} and serving files from {}",
        sock_addr,
        opt.static_dir.to_string_lossy()
    );

    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .expect("Unable to start server");
}

async fn ws_handler(
    ws: WebSocketUpgrade,
    user_agent: Option<TypedHeader<headers::UserAgent>>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    let user_agent = if let Some(TypedHeader(user_agent)) = user_agent {
        user_agent.to_string()
    } else {
        String::from("Unknown browser")
    };
    log::info!("`{user_agent}` at {addr} connected.");
    ws.on_upgrade(move |socket| handle_socket(socket, addr, state))
}

fn make_data(cfg: &Config, data: &TickData, initial: bool) -> Option<String> {
    // log::info!("sending data {:?}", data.history);
    let json_data = if initial {
        let initial_data = InitialMessage { tick: data, cfg };
        serde_json::to_string(&initial_data)
    } else {
        let update_data = UpdateMessage {
            hist_upd: data.history.back(),
            active_alarms: &data.active_alarms,
        };
        serde_json::to_string(&update_data)
    };
    match json_data {
        Ok(json) => Some(json),
        Err(e) => {
            log::error!("Failed to serialize json: {}", e);
            None
        }
    }
}
async fn send_data(data: Option<String>, socket: &mut WebSocket) -> bool {
    match data {
        Some(json) => socket.send(Message::Text(json)).await.is_err(),
        None => false,
    }
}

async fn handle_socket(mut socket: WebSocket, who: SocketAddr, mut state: AppState) {
    let json_data = {
        let initial_tick = state.rx_data.borrow();
        make_data(&state.cfg, &initial_tick, true)
    };
    if send_data(json_data, &mut socket).await {
        log::info!("client {who} abruptly disconnected");
        return;
    }
    let mut counter: u32 = 5;

    loop {
        tokio::select! {
            recv = socket.recv() => {
                match recv {
                    None => {
                        log::info!("client {who} abruptly disconnected");
                        return;
                    },
                    Some(Err(e)) => {
                        log::error!("Got error when receiving: {}", e);
                    },
                    Some(Ok(data)) => {
                        let result: ControlFlow<(), ()> = process_message(data, who);
                        if result.is_break() {
                            return;
                        }
                    }
                }
            },
            change = state.rx_data.changed() => {
                match change {
                    Err(e) => {
                        log::error!("Got data receive error: {}", e);
                    },
                    Ok(_) => {
                        log::debug!("received new data");
                        let full_send = counter != 0;
                        if counter != 0 {
                            counter = counter - 1;
                        }
                        let json_data = make_data(&state.cfg, &state.rx_data.borrow_and_update(), full_send);
                        if send_data(json_data, &mut socket).await {
                            log::info!("client {who} abruptly disconnected");
                            return;
                        }
                    }
                }
            },
        }
    }
}

fn process_message(msg: Message, who: SocketAddr) -> ControlFlow<(), ()> {
    match msg {
        Message::Text(t) => {
            log::info!(">>> {who} sent str: {t:?}");
        }
        Message::Binary(d) => {
            log::info!(">>> {who} sent {} bytes: {:?}", d.len(), d);
        }
        Message::Close(c) => {
            if let Some(cf) = c {
                log::info!(
                    ">>> {who} sent close with code {} and reason `{}`",
                    cf.code,
                    cf.reason
                );
            }
            return ControlFlow::Break(());
        }
        _ => {}
    }
    ControlFlow::Continue(())
}
