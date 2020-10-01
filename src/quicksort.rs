/*Inspired by the rayon data parallelism in rust blog post*/
use rayon::join;

pub fn quicksort_asc_h<T: PartialOrd + Send>(input: &mut[T]){
    let h = input.len();
    match h {
        0 | 1 => return,
        _ =>{
            let pivot_loc = partition_asc(input);
            let(lo, hi) = input.split_at_mut(pivot_loc);
            join(|| quicksort_asc_h(lo),
            || quicksort_asc_h(hi));

        }
    }
}



pub fn partition_asc<T: PartialOrd + Send>(input: &mut[T]) -> usize{
    let pivot = input.len() - 1;
    let mut i = 0;

    for j in 0..pivot{
        if input[j] <= input[pivot]{
            input.swap(i,j);
            i += 1;
        }
    }
    input.swap(i, pivot);
    i
}

pub fn quicksort_asc(mut input: Vec<usize>) -> Vec<usize>{
    quicksort_asc_h(&mut input);
    input
}

pub fn quicksort_desc_h<T: PartialOrd + Send>(input: &mut[T]){
    let h = input.len();
    match h {
        0 | 1 => return,
        _ =>{
            let pivot_loc = partition_desc(input);
            let(lo, hi) = input.split_at_mut(pivot_loc);
            join(|| quicksort_desc_h(lo),
            || quicksort_desc_h(hi));

        }
    }
}



pub fn partition_desc<T: PartialOrd + Send>(input: &mut[T]) -> usize{
    let pivot = input.len() - 1;
    let mut i = 0;

    for j in 0..pivot{
        if input[j] >= input[pivot]{
            input.swap(j, i);
            i += 1;
        }
    }
    input.swap(i, pivot);
    i
}

pub fn quicksort_desc(mut input: Vec<usize>) -> Vec<usize>{
    quicksort_desc_h(&mut input);
    input
}

#[test]
fn zero_vec(){
    let test_seq: Vec<usize> = [].to_vec();
    let sort_seq = quicksort_asc(test_seq.clone());
    assert_eq!(test_seq, sort_seq);

    let sort_seq1 = quicksort_desc(test_seq.clone());
    assert_eq!(test_seq, sort_seq1);

}

#[test]
fn same_val_vec(){
    let test_seq: Vec<usize> = [0,0].to_vec();
    let sort_seq = quicksort_asc(test_seq.clone());
    assert_eq!(test_seq, sort_seq);

    let sort_seq1 = quicksort_desc(test_seq.clone());
    assert_eq!(test_seq, sort_seq1);

}

#[test]
fn base_case_vec(){
    let test_seq: Vec<usize> = [1].to_vec();
    let sort_seq = quicksort_asc(test_seq.clone());
    assert_eq!(test_seq, sort_seq);

    let sort_seq1 = quicksort_asc(test_seq.clone());
    assert_eq!(test_seq, sort_seq1);

}

#[test]
fn sorted_values_vec(){
    let test_seq: Vec<usize> = [6,8,29,45,64].to_vec();
    let sort_seq = quicksort_asc(test_seq.clone());
    assert_eq!(test_seq, sort_seq);

    let test_seq1 = vec![64,45,29,8,6];
    let sort_seq1 = quicksort_desc(test_seq.clone());
    assert_eq!(test_seq1, sort_seq1);

}

#[test]
fn multi_value_vec(){
    let test_seq: Vec<usize> = [1,54,32,6,8,89,150,52].to_vec();
    let sort_seq = quicksort_asc(test_seq.clone());
    assert_eq!([1,6,8,32,52,54,89,150].to_vec(), sort_seq);

    let sort_seq1 = quicksort_desc(test_seq.clone());
    assert_eq!([150,89,54,52,32,8,6,1].to_vec(), sort_seq1);

}

