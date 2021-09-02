use std::env;
use weather::api::{get_forecast, get_point, Client, APP_USER_AGENT};
use weather::{Point, Result};

#[tokio::main]
async fn main() -> Result<()> {
  let args: Vec<String> = env::args().collect();
  let lat: &f32 = &args[1].parse::<f32>()?;
  let lng: &f32 = &args[2].parse::<f32>()?;
  let pnt = Point::new(lat, lng)?;
  let client = Client::builder().user_agent(APP_USER_AGENT).build()?;

  let res = get_point(&pnt, &client).await?;
  let resf = get_forecast(&res, &client).await?;
  for i in resf.properties.periods.iter() {
    println!("{:#?}, {:#?}", &i.name, &i.detailed_forecast);
  }
  Ok(())
}
