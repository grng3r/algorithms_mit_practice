//An improvement on the implementation would be using the median of three to select pivot for quicksort
use crate::insertion_sort;
use crate::quicksort::quicksort_asc_h;
use crate::heapsort::heapsort_asc;


pub fn introsort<T: Ord + Send>(a: &mut[T]){
    let size = a.len();
    let size_float = size as f32;
    let maxdepth = 2 * (num_traits::Float::log(size_float, 10 as f32) as usize);

    if size < 16{
        insertion_sort::insertion_sort_asc(a);
    }

    match maxdepth{
        0 => heapsort_asc(a),
        _=> quicksort_asc_h(a)
    } 
}


pub fn introsort_vec(mut input: Vec<usize>) -> Vec <usize>{
    introsort(&mut input);
    input
}

#[test]
fn zero_vec(){
    let test_seq: Vec <usize> = Vec::new();
    let sort_seq =introsort_vec(test_seq.clone());
    assert_eq!(test_seq, sort_seq);
}

#[test]
fn base_case(){
    let test_seq: Vec <usize> = vec![5];
    let sort_seq =introsort_vec(test_seq.clone());
    assert_eq!(test_seq, sort_seq);
}

#[test]
fn same_val(){
    let test_seq: Vec <usize> = vec![5,5];
    let sort_seq =introsort_vec(test_seq.clone());
    assert_eq!(test_seq, sort_seq);
}

#[test]
fn sorted(){
    let test_seq: Vec <usize> = vec![1,2,3,4,5];
    let sort_seq =introsort_vec(test_seq.clone());
    assert_eq!(test_seq, sort_seq);
}

#[test]
fn multi_vec(){
    let test_seq: Vec <usize> = vec![1,54,8,89,9,155,53];
    let sort_seq =introsort_vec(test_seq.clone());
    assert_eq!(vec![1,8,9,53,54,89,155], sort_seq);
}

#[test]
fn val16_vec(){
    let test_seq: Vec <usize> = vec![1,54,8,89,9,155,53,22,76,100,13,2,45,90,69,4,17];
    let sort_seq =introsort_vec(test_seq.clone());
    assert_eq!(vec![1,2,4,8,9,13,17,22,45,53,54,69,76,89,90,100,155], sort_seq);
}

