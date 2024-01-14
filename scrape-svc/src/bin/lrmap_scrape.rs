use chrono_tz::America::Chicago;
use geojson::{feature::Id, Feature, GeoJson, Geometry};
use gtfs_rt::FeedMessage;
use prost::Message;
use redis::JsonAsyncCommands;
use std::time::Duration;

const VEHICLES_URL: &str = "https://svc.metrotransit.org/mtgtfs/vehiclepositions.pb";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let redis_url = std::env::var("REDIS")?;

    let redis = redis::Client::open(redis_url)?;

    let mut con = redis.get_async_connection().await?;

    let rclient = reqwest::Client::default();

    let mut inter = tokio::time::interval(Duration::from_millis(15100));

    loop {
        let _event = inter.tick().await;

        let response = rclient.get(VEHICLES_URL).send().await?;

        let bytes = response.bytes().await?;

        let msg = FeedMessage::decode(bytes)?;

        let datetime =
            chrono::DateTime::from_timestamp(msg.header.timestamp.unwrap_or(0).try_into()?, 0)
                .map(|d| d.with_timezone(&Chicago))
                .map(|d| d.to_rfc2822())
                .unwrap_or_default();

        let locations = msg
            .entity
            .into_iter()
            .filter_map(|v| {
                let vp = v.vehicle.unwrap();

                let trip = vp.trip.unwrap();

                if trip.route_id() == "901" || trip.route_id() == "902" {
                    let pos = vp.position.unwrap();

                    let mut m = serde_json::Map::default();

                    m.insert("route".into(), trip.route_id().to_string().into());

                    Some(GeoJson::Feature(Feature {
                        id: Some(Id::String(trip.trip_id().to_string())),
                        bbox: None,
                        geometry: Some(Geometry {
                            bbox: None,
                            value: geojson::Value::Point(vec![
                                pos.longitude.into(),
                                pos.latitude.into(),
                            ]),
                            foreign_members: None,
                        }),
                        properties: Some(m),
                        foreign_members: None,
                    }))
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        let j = serde_json::json!({
            "msp_datetime": datetime,
            "msp_locations": locations,
        });

        con.json_set("metro-transit", ".", &j).await?;
    }

    Ok(())
}
