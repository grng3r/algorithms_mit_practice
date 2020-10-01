fn heapify_asc<T: Ord>(a: &mut[T], mut root: usize){
    let end = (a.len() - 2)/2;
    loop {
        let child;
        let l = 2 * root + 1;
        if l > end{
            break;
        }
        let r = l - 1;
        if r <= end && a[r] > a[l]{
            child = r;
        } else{
           child = l;
        }
        
        if a[child] > a[root]{
            a.swap(root, child);
        }

        root = child;
    }
}

pub fn heapsort_asc<T: Ord>(a: &mut[T]){
    let len= a.len();

    match len{
        0 | 1 => return,
        2 =>{
            if a[0] == a[1]{
                return
            }
        },
        _ => {
            let last_parent = (len - 2)/2;

            for i in (0..=last_parent).rev(){
                heapify_asc(a, i);
            }

            for end in(1..a.len()).rev(){
                a.swap(0, end);
                heapify_asc(&mut a[..end], 0);
            }
        }
    }
}

pub fn heapsort_vec_asc(mut input: Vec<usize>) -> Vec<usize>{
    heapsort_asc(&mut input);
    input
}


#[test]
fn zero_vec(){
    let test_seq: Vec<usize> = Vec::new();
    let sort_seq = heapsort_vec_asc(test_seq.clone());

    assert_eq!(test_seq, sort_seq);
}

#[test]
fn same_val_vec(){
    let test_seq: Vec<usize> = [0,0].to_vec();
    let sort_seq = heapsort_vec_asc(test_seq.clone());

    assert_eq!(test_seq, sort_seq);
}

#[test]
fn base_vec(){
    let test_seq: Vec<usize> = vec![1];
    let sort_seq = heapsort_vec_asc(test_seq.clone());

    assert_eq!(test_seq, sort_seq);
}


#[test]
fn multivalue_vec(){
    let test_seq: Vec<usize> = [1,54,89,8,155,9,53].to_vec();
    let sort_seq = heapsort_vec_asc(test_seq);
    assert_eq!([1,8,9,53,54,89,155].to_vec(), sort_seq);
}
