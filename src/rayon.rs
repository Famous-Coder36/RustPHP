use ext_php_rs::prelude::*;
use rayon::prelude::*;

#[php_class]
pub struct RayonClass;

#[php_impl]
impl RayonClass {

    pub fn par_iter(arr: Vec<i32>) -> Vec<i32> {
        arr.par_iter().copied().collect()
    }

    pub fn into_par_iter(arr: Vec<i32>) -> Vec<i32> {
        arr.into_par_iter().collect()
    }
    
    pub fn map(arr: Vec<i32>, op: String, value: i32) -> Vec<i32> {
    arr.into_par_iter()
        .map(|v| match op.as_str() {
            "mul" => v * value,
            "add" => v + value,
            "sub" => v - value,
            "div" => if value != 0 { v / value } else { v },
            _ => v,
        })
        .collect()
      }
      
      pub fn flat_map(arr: Vec<i32>, value: i32) -> Vec<i32> {
         arr.into_par_iter()
        .flat_map(|v| vec![v, v * value])
        .collect()
      }
      
      pub fn filter(arr: Vec<i32>, op: String, value: i32) -> Vec<i32> {
         arr.into_par_iter()
        .filter(|v| match op.as_str() {
            "gt" => *v > value,
            "lt" => *v < value,
            "eq" => *v == value,
            "gte" => *v >= value,
            "lte" => *v <= value,
            _ => false,
        })
        .collect()
      }
      
      pub fn filter_map(arr: Vec<i32>, value: i32) -> Vec<i32> {
         arr.into_par_iter()
        .filter_map(|v| {
            if v > value {
                Some(v * 2)
            } else {
                None
            }
        })
        .collect()
     }
     
     pub fn reduce(arr: Vec<i32>, op: String) -> i32 {
    if arr.is_empty() {
        return 0;
    }

    arr.into_par_iter().reduce(
        || 0,
        |a, b| match op.as_str() {
            "add" => a + b,
            "mul" => a * b,
            "max" => a.max(b),
            "min" => a.min(b),
            _ => a,
        }
    )
}

pub fn fold(arr: Vec<i32>, op: String, init: i32) -> i32 {
    arr.into_par_iter()
        .fold(|| init, |acc, v| match op.as_str() {
            "add" => acc + v,
            "mul" => acc * v,
            _ => acc,
        })
        .reduce(|| init, |a, _| a)
}
    
    pub fn sum(arr: Vec<i32>) -> i32 {
    arr.into_par_iter().sum()
}

pub fn product(arr: Vec<i32>) -> i32 {
    arr.into_par_iter().product()
}

pub fn for_each(arr: Vec<i32>) -> Vec<i32> {
    arr.into_par_iter().map(|v| {
        // bu yerda faqat Rust ishlaydi
        v
    }).collect()
}

pub fn for_eachp(arr: Vec<i32>, action: String) {
    arr.into_par_iter().for_each(|v| {
        if action == "print" {
            php_println!("{}", v);
        }
    });
}

pub fn for_each_with(arr: Vec<i32>, value: i32) {
    arr.into_par_iter().for_each(|v| {
        let _ = v + value;
    });
}
    
    pub fn any(arr: Vec<i32>, op: String, value: i32) -> bool {
    arr.into_par_iter().any(|v| match op.as_str() {
        "gt" => v > value,
        "lt" => v < value,
        "eq" => v == value,
        _ => false,
    })
}

pub fn all(arr: Vec<i32>, op: String, value: i32) -> bool {
    arr.into_par_iter().all(|v| match op.as_str() {
        "gt" => v > value,
        "lt" => v < value,
        "eq" => v == value,
        _ => false,
    })
}

pub fn collect(arr: Vec<i32>) -> Vec<i32> {
    arr.into_par_iter().collect()
}

pub fn chunks(arr: Vec<i32>, size: usize) -> Vec<Vec<i32>> {
    arr.par_chunks(size)
        .map(|c| c.to_vec())
        .collect()
}

pub fn reduce_with(arr: Vec<i32>, op: String) -> i32 {
    if arr.is_empty() {
        return 0;
    }

    arr.into_par_iter().reduce(
        || 0,
        |a, b| match op.as_str() {
            "add" => a + b,
            "mul" => a * b,
            _ => a,
        }
    )
}

pub fn try_reduce(arr: Vec<i32>, op: String) -> Option<i32> {
    if arr.is_empty() {
        return None;
    }

    Some(
        arr.into_par_iter().reduce(
            || 0,
            |a, b| match op.as_str() {
                "add" => a + b,
                "mul" => a * b,
                _ => a,
            }
        )
    )
}

}