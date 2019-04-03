pub fn edit_distance(_left: &str, _right: &str) -> usize {
  let mut left = _left;
  let mut right = _right;
  
  if left == right {
    return 0
  };

  let swap = left;
  let mut left_length = left.chars().count();
  let mut right_length = right.chars().count();
  
  if left_length > right_length {
    std::mem::swap(&mut left_length, &mut right_length);
    left = right;
    right = swap;
  }

  // Performing suffix trimming:
  // We can linearly drop suffix common to both strings since they
  // don't increase distance at all
  let mut left_chars_rev = left.chars().rev();
  let mut right_chars_rev = right.chars().rev();
  while left_length > 0 && left_chars_rev.next() == right_chars_rev.next() {
    left_length -= 1;
    right_length -= 1;
  }

  let mut start = 0;

  let mut left_chars = left.chars();
  let mut right_chars = right.chars();
  while start < left_length && left_chars.next() == right_chars.next() {
    start += 1;
  }

  if left_length == 0 {
    return right_length;
  }
  
  let mut cur = vec![0; left_length];
  for i in 0..(left_length - start) {
    cur[i] = i + 1;
  }

  let mut temp: usize;
  let mut temp2: usize;
  let mut result: usize = 0;

  for (mut i, r_ch) in split_str(right, start, right_length).enumerate() {
    temp = i;
    i += 1;
    result = i;

    for (j, l_ch) in split_str(left, start, left_length).enumerate() {
      temp2 = if l_ch == r_ch {
        temp
      } else {
        temp + 1
      };

      temp = cur[j];

      cur[j] = std::cmp::min(
        temp + 1,
        std::cmp::min(
          result + 1, 
          temp2
        )
      );

      result = cur[j];
    }
  }

  return result;

}

fn split_str<'a>(left: &'a str, start: usize, end: usize) -> impl Iterator<Item=char> + 'a {
  left.chars().skip(start).take(end - start)
}

#[test]
fn simple() {
    assert_eq!(edit_distance("kitten", "sitting"), 3);
    assert_eq!(edit_distance("Tier", "Tor"), 2);
}

#[test]
fn same() {
    assert_eq!(edit_distance("kitten", "kitten"), 0);
}

#[test]
fn empty_a() {
    assert_eq!(edit_distance("", "kitten"), 6);
}

#[test]
fn empty_b() {
    assert_eq!(edit_distance("sitting", ""), 7);
}

#[test]
fn empty_both() {
    assert_eq!(edit_distance("", ""), 0);
}

#[test]
fn unicode_misc() {
    assert_eq!(edit_distance("üö", "uo"), 2);
}

#[test]
fn unicode_thai() {
    assert_eq!(edit_distance("ฎ ฏ ฐ", "a b c"), 3);
}

#[test]
fn unicode_misc_equal() {
    assert_eq!(
        edit_distance("☀☂☃☄", "☀☂☃☄"),
        0
    );
}

#[test]
fn random_test() {
  assert_eq!(
    edit_distance("\u{80}", ""),
    1
  )
}

use quickcheck::quickcheck;

#[test]
fn at_least_size_difference_property() {
    fn at_least_size_difference(a: String, b: String) -> bool {
        let size_a = a.chars().count();
        let size_b = b.chars().count();
        let diff = if size_a > size_b {
            size_a - size_b
        } else {
            size_b - size_a
        };
        edit_distance(&a, &b) >= diff
    }

    quickcheck(at_least_size_difference as fn(a: String, b: String) -> bool);
}

#[test]
fn at_most_length_of_longer_property() {
    fn at_most_size_of_longer(a: String, b: String) -> bool {
        let upper_bound = *[a.chars().count(), b.chars().count()].iter().max().unwrap();
        edit_distance(&a, &b) <= upper_bound
    }

    quickcheck(at_most_size_of_longer as fn(a: String, b: String) -> bool);
}

#[test]
fn zero_iff_a_equals_b_property() {
    fn zero_iff_a_equals_b(a: String, b: String) -> bool {
        let d = edit_distance(&a, &b);

        if a == b {
            d == 0
        } else {
            d > 0
        }
    }

    quickcheck(zero_iff_a_equals_b as fn(a: String, b: String) -> bool);
}

#[test]
fn triangle_inequality_property() {
    fn triangle_inequality(a: String, b: String, c: String) -> bool {
        edit_distance(&a, &b)
            <= edit_distance(&a, &c) + edit_distance(&b, &c)
    }

    quickcheck(triangle_inequality as fn(a: String, b: String, c: String) -> bool);
}