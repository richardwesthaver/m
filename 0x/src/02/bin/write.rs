use rocksdb::{ColumnFamilyDescriptor, Options, DB};
use ron::de::from_str;
use std::fs::File;
use std::io::{self, BufRead};
use weather::WeatherBundle;
// from rust_rocksdb/tests
// fn get_byte_slice<T: AsRef<[u8]>>(source: &'_ T) -> &'_ [u8] {
//   source.as_ref()
// }

fn main() {
  let input_path: &str = "data/forecast.ron";
  let input = File::open(&input_path).expect("couldnt open file");
  let mut result: Vec<WeatherBundle> = Vec::new();
  let db_path: &str = "data/db";

  {
    for line in io::BufReader::new(input).lines() {
      // this is bloat!
      match from_str(&line.unwrap()) {
        Ok(x) => result.push(x),
        Err(e) => {
          println!("error: {}", e);
          std::process::exit(1);
        }
      }
    }
  }

  {
    let loc_opts = Options::default();
    let fc_opts = Options::default();
    let mut db_opts = Options::default();
    db_opts.create_if_missing(true);
    db_opts.create_missing_column_families(true);
    let loc_cf = ColumnFamilyDescriptor::new("location", loc_opts);
    let fc_cf = ColumnFamilyDescriptor::new("forecast", fc_opts);
    let db = DB::open_cf_descriptors(&db_opts, db_path, vec![loc_cf, fc_cf]).unwrap();
    let cf1 = db.cf_handle("location").unwrap();
    let cf2 = db.cf_handle("forecast").unwrap();
    for i in result.into_iter() {
      let fc = i.forecast;
      for c in fc.into_iter() {
        let key = bincode::serialize(&i.location.city).unwrap();
        let val = bincode::serialize(&[
          c.start.timestamp().to_string(),
          c.wind_speed,
          c.wind_direction,
          c.temperature.to_string(),
          c.short_forecast,
        ])
        .unwrap();
        db.put_cf(cf2, key, val).unwrap();
      }
      let key = bincode::serialize(&i.location.city).unwrap();
      let val = bincode::serialize(&[i.location.lat, i.location.lng]).unwrap();
      db.put_cf(cf1, &key, val).unwrap();
      println!("inserted: {}", i.location.city);
      if let Some(i) = db.get_cf(cf1, &key).unwrap() {
        let (_pre, res, _suf) = unsafe { i.align_to::<f32>() };
        println!("coords: {:#?}", res);
      }
    }
    db.compact_range_cf(cf1, None::<&[u8]>, None::<&[u8]>);
    db.compact_range_cf(cf2, None::<&[u8]>, None::<&[u8]>);
  }
}
