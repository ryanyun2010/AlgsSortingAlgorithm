use core::f64;

use rand::Rng;

pub async fn sorting_alg(arr: Vec<f64>, p: usize) -> Vec<f64>{
    let n = arr.len();
    let per_process = (n as f64 / p as f64).floor();
    let mut extra = n - per_process as usize * p;
    let mut local_arrs = vec![];
    let mut start = 0;
    for _ in 0..p {
        if extra > 0 {
            local_arrs.push(arr[start..(start + per_process as usize + 1)].to_vec());
            start += per_process as usize + 1;
            extra -= 1;
        }
        else {
            local_arrs.push(arr[start..(start + per_process as usize)].to_vec());
            start += per_process as usize;
        }
    }
    let mut processes = vec![];
    
    while !local_arrs.is_empty() {
        processes.push(tokio::task::spawn(
        sort_local( local_arrs.remove(0))
        )) 
    };

    
    let mut results = vec![];
    for process in processes {
        results.push(process.await.unwrap());
    }
    merge_n_sorted_arrays(&results)
}


fn merge_n_sorted_arrays(arrays: &[Vec<f64>]) -> Vec<f64> {
    let mut result = Vec::new();
    let mut indices = vec![0; arrays.len()]; 

    loop {
        let mut min_value: Option<f64> = None;
        let mut min_index = None;
        for (i, array) in arrays.iter().enumerate() {
            if indices[i] < array.len() {
                let value = array[indices[i]];
                if min_value.is_none() || value < min_value.unwrap() {
                    min_value = Some(value);
                    min_index = Some(i);
                }
            }
        }
        if let Some(i) = min_index {
            result.push(min_value.unwrap());
            indices[i] += 1;
        } else {
            break; 
        }
    }

    result
}

pub async fn sort_local(arr: Vec<f64>) -> Vec<f64> {
    merge_sort(arr)
}

fn merge_sort(arr: Vec<f64>) -> Vec<f64> {
    if arr.len() <= 1 {
        return arr;
    }

    let mid = arr.len() / 2;
    let left = merge_sort(arr[..mid].to_vec());
    let right = merge_sort(arr[mid..].to_vec());

    merge(left, right)
}

fn merge(left: Vec<f64>, right: Vec<f64>) -> Vec<f64> {
    let mut result = Vec::with_capacity(left.len() + right.len());
    let (mut i, mut j) = (0, 0);
    
    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            result.push(left[i]);
            i += 1;
        } else {
            result.push(right[j]);
            j += 1;
        }
    }
    
    result.extend_from_slice(&left[i..]);
    result.extend_from_slice(&right[j..]);

    result
}
fn generate_random_f64s(x: usize) -> Vec<f64> {
    let mut rng = rand::rng();
    (0..x).map(|_| rng.random_range(-100.0..100.0)).collect()
}



const FLOATS: usize = 2000000;
#[tokio::main]
pub async fn main() {
    let arr = generate_random_f64s(FLOATS);
    println!("Sorting {} floats", FLOATS);


    let time = std::time::Instant::now();
    let mut sorted_dumb = arr.clone();
    sorted_dumb.sort_by(|a, b| a.partial_cmp(b).unwrap());
    println!("Time taken by simple rust sort (O(nlog(n)): {:?}", time.elapsed());
    let mut times = vec![];
    for i in 1..20 {
        let time = std::time::Instant::now();
        let sorted_arr = sorting_alg(arr.clone(), i).await;
        println!("Time taken by {} partition sort: {:?}",i, time.elapsed());
        times.push(time.elapsed().as_secs_f64());
        assert_eq!(sorted_dumb, sorted_arr);
    }
    let mut min_time = f64::INFINITY;
    let mut min_index = 0;
    for (i, &time) in times.iter().enumerate() {
        if time < min_time {
            min_index = i;
            min_time = time;
        }
    }
    println!("Minimum time taken by {} partition sort: {:?}s", min_index + 1, min_time);
    
}
