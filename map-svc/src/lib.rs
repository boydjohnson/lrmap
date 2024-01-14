use std::pin::Pin;

use redis::aio::AsyncStream;

use {
    axum::{extract::State, http::StatusCode, routing::get},
    geojson::Feature,
    map::{draw, to_drawable_distance, to_drawable_distance_label},
    plotters::{
        backend::SVGBackend,
        coord::{cartesian::Cartesian2d, types::RangedCoordf64},
        drawing::IntoDrawingArea,
        style::RGBColor,
    },
    proj::Proj,
    redis::JsonAsyncCommands,
    std::{
        io::{BufRead, BufReader},
        sync::Arc,
    },
    tokio::sync::{Mutex, MutexGuard},
};

pub mod map;

static BYTES: &[u8] = include_bytes!("../../transit.ndjson");

pub fn get_router() -> axum::Router<RedisClient> {
    axum::Router::new().route("/", get(root))
}

pub async fn root(redis: State<RedisClient>) -> Result<String, axum::http::StatusCode> {
    let mut con = redis.connection().await;

    let info_string_json: String = (*con)
        .json_get("metro-transit", ".")
        .await
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    let points: LightRailPoints = serde_json::from_str(&info_string_json)
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    let _datetime = points.msp_datetime;

    let _locations = points.msp_locations;

    let reader = BufReader::with_capacity(100_000, BYTES);

    let mut writer = String::default();
    {
        let root = SVGBackend::with_string(&mut writer, (1000, 1000)).into_drawing_area();

        let transform = Proj::new_known_crs("EPSG:4326", "EPSG:32615", None).unwrap();

        let center_t = transform.convert((-93.20, 44.9056)).unwrap();

        let v = 13000.0;

        let min_x = center_t.0 - v;
        let min_y = center_t.1 - v;
        let max_x = center_t.0 + v;
        let max_y = center_t.1 + v;

        let root = root.apply_coord_spec(Cartesian2d::<RangedCoordf64, RangedCoordf64>::new(
            min_x..max_x,
            max_y..min_y,
            (0..1000, 0..1000),
        ));

        root.fill(&RGBColor(240, 240, 240)).unwrap();

        for line in reader.lines() {
            let feature = line
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
                .parse::<Feature>()
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

            draw(feature, &root, &transform);
        }

        for feature in _locations {
            draw(feature, &root, &transform);
        }

        root.draw(&to_drawable_distance::<SVGBackend>(
            vec![
                (max_x - 2000.0, max_y - 500.0),
                (max_x - 2000.0 + 1609.34, max_y - 500.0),
            ],
            4,
            RGBColor(34, 31, 32),
        ))
        .unwrap();

        root.draw(&to_drawable_distance_label(
            vec![
                (max_x - 2000.0, max_y - 500.0),
                (max_x - 2000.0 + 1609.34, max_y - 500.0),
            ],
            1,
            RGBColor(34, 31, 32),
        ))
        .unwrap();

        root.present().unwrap();
    }

    Ok(writer)
}

#[derive(Clone)]
pub struct RedisClient {
    con: Arc<Mutex<redis::aio::Connection<Pin<Box<dyn AsyncStream + Send + Sync>>>>>,
}

impl RedisClient {
    pub fn new(con: redis::aio::Connection<Pin<Box<dyn AsyncStream + Send + Sync>>>) -> Self {
        RedisClient {
            con: Arc::new(Mutex::new(con)),
        }
    }

    pub async fn connection(
        &self,
    ) -> MutexGuard<redis::aio::Connection<Pin<Box<dyn AsyncStream + Send + Sync>>>> {
        self.con.lock().await
    }
}

#[derive(serde::Deserialize)]
pub struct LightRailPoints {
    pub msp_datetime: String,
    pub msp_locations: Vec<geojson::Feature>,
}
