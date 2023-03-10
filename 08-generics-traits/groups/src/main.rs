struct Groups<T> {
  inner: Vec<T>,
}

impl<T> Groups<T> {
  fn new(inner: Vec<T>) -> Self {
    Self { inner }
  }
}

impl<T: PartialEq> Iterator for Groups<T> {
  type Item = Vec<T>;

  fn next(&mut self) -> Option<Self::Item> {
    if self.inner.len() == 0 {
      return None;
    }
    // while let Some(current_element) = self.inner.iter().next() {
    //   if first_element == current_element {
    //     group.push(current_elemen);
    //     self.last_position += 1;
    //   } else {
    //     return Some(group);
    //   }
    // }

    let first_element: &T = &self.inner[0];
    let mut last_position: usize = 1;
    for current_element in self.inner[1..].iter() {
      if current_element == first_element {
        last_position += 1;
      } else {
        break;
      }
    }

    let group = self.inner.drain(0..last_position).collect();

    Some(group)
  }
}

fn main() {
  let data = vec![4, 1, 1, 2, 1, 3, 3, -2, -2, -2, 5, 5];
  // groups:     |->|---->|->|->|--->|----------->|--->|
  assert_eq!(
    Groups::new(data).into_iter().collect::<Vec<Vec<_>>>(),
    vec![
      vec![4],
      vec![1, 1],
      vec![2],
      vec![1],
      vec![3, 3],
      vec![-2, -2, -2],
      vec![5, 5],
    ]
  );

  let data2 = vec![1, 2, 2, 1, 1, 2, 2, 3, 4, 4, 3];
  // groups:      |->|---->|---->|----|->|----->|->|
  assert_eq!(
    Groups::new(data2).into_iter().collect::<Vec<Vec<_>>>(),
    vec![
      vec![1],
      vec![2, 2],
      vec![1, 1],
      vec![2, 2],
      vec![3],
      vec![4, 4],
      vec![3],
    ]
  )
}
