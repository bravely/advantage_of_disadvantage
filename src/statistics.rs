pub struct DataSet {
  pub data: Vec<u8>,
  pub mean: Option<f32>,
  pub std_deviation: Option<f32>
}

impl DataSet {
  pub fn new(data: Vec<u8>) -> DataSet {
    let data = data.clone();
    DataSet {
      mean: DataSet::mean(&data),
      std_deviation: DataSet::std_deviation(&data),
      data,
    }
  }

  fn mean(data: &Vec<u8>) -> Option<f32> {
      let sum = data.iter().clone().map(|&e| e as u32).sum::<u32>() as f32;

      let count = data.len();

      match count {
          positive if positive > 0 => Some(sum / count as f32),
          _ => None,
      }
  }

  fn std_deviation(data: &Vec<u8>) -> Option<f32> {
      match (DataSet::mean(data), data.len()) {
          (Some(data_mean), count) if count > 0 => {
              let variance = data.iter().map(|value| {
                  let diff = data_mean - (*value as f32);

                  diff * diff
              }).sum::<f32>() / count as f32;

              Some(variance.sqrt())
          },
          _ => None
      }
  }

  pub fn zscore(&self, index: usize) -> Option<f32> {
    match (self.mean, self.std_deviation) {
      (Some(mean), Some(std_deviation)) => {
        let difference = self.data[index] as f32 - mean;

        Some(difference / std_deviation)
      },
      _ => None
    }
  }
}
