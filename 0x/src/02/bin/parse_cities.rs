use std::fs::File;
use std::io::Write;
use weather::City;
fn main() -> Result<(), csv::Error> {
  let mut input = csv::Reader::from_path("data/uscities.csv")?;
  let mut output = File::create("data/uscities.ron")?;

  for city in input.deserialize() {
    let city: City = city?;
    write!(&mut output, "{}\n", ron::ser::to_string(&city).unwrap())?;
  }
  Ok(())
}
