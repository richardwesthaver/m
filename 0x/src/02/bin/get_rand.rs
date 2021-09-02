use rand::seq::IteratorRandom;
use ron::de::from_str;
use std::io::{self, BufRead};
use std::time::Duration;
use std::{fs::File, path::Path};
use tokio::fs::File as TFile;
use tokio::io::{AsyncWriteExt, BufWriter};
use weather::{api, City, Point, Result, WeatherBundle};

/// Collect (lat,lng) values from file, return a Vec<Point>
fn collect_lines(input_path: &str) -> Result<Vec<City>> {
  let mut result = Vec::new();
  if let Ok(lines) = read_lines(&input_path) {
    for line in lines
      .into_iter()
      .choose_multiple(&mut rand::thread_rng(), 16)
    {
      if let Ok(city) = line {
        let pcity: City = from_str(&city)?;
        result.push(pcity);
      }
    }
  }
  Ok(result)
}

/// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(path: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
  P: AsRef<Path>,
{
  let file = File::open(path)?;
  Ok(io::BufReader::new(file).lines())
}

#[tokio::main]
async fn main() -> Result<()> {
  // Create a single reqwest::Client that is re-used
  let client = api::Client::builder()
    .user_agent(api::APP_USER_AGENT)
    .build()?;

  let input_path: &str = "data/uscities.ron"; // ../data/uscities.ron
  let cities: Vec<City> = collect_lines(&input_path)?; // get our list of cities

  let out_file = TFile::create("data/forecast.ron").await?;

  let mut handles = Vec::new(); // stores async tasks

  // loop over the sample, spawning two new async handles for the
  // network client and out_file writer.
  for city in cities {
    let pnt = Point::new(&city.lat, &city.lng)?;
    let out = out_file.try_clone().await?;
    let client = client.clone();
    let mut writer = BufWriter::new(out);
    let handle = tokio::spawn(async move {
      let res = api::get_point(&pnt, &client).await.unwrap();
      let result = api::get_forecast_hourly(&res, &client).await;
      tokio::time::sleep(Duration::from_millis(1)).await;
      if let Ok(r) = result {
        println!("{:#?} - got result", &city.city);
        let output = WeatherBundle::new(city, r);
        let ronr = ron::ser::to_string(&output) // ron::ser::PrettyConfig::new()
          .unwrap()
          .into_bytes();
        writer.write_all(&ronr).await.unwrap();
        writer.write_all(b"\n").await.unwrap();
        writer.flush().await.unwrap();
      }
    });
    handles.push(handle)
  }

  for handle in handles {
    handle.await?;
  }
  Ok(())
}
