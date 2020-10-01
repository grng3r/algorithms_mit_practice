

pub fn insertion_sort_asc<T: Ord>(sequence : &mut[T]){
    let len = sequence.len();
    for i in 0..len{
        for j in (0..i).rev(){
            while sequence[j] > sequence[j+1]{
                sequence.swap(j, j+1);
            }
        }
    }
}

/*pub fn insertion_sort_asc_vec(mut input: Vec<usize>) -> Vec<usize>{
    insertion_sort_asc(&mut input);
    input
}*/

pub fn insertion_sort_desc(seq: &mut[i32]) -> &mut[i32]{
    let len = seq.len();
    for i in 0..len{
        for j in (0..i).rev(){
            while seq[j] < seq[j+1]{
                seq.swap(j, j+1);
            }
        }
    }
    seq
}



#[test]
fn test_ins_sort_0(){
    let mut seq: [i32;0] = [];

    insertion_sort_asc(&mut seq);
    assert_eq!(seq,[]);

    let seq1 = insertion_sort_desc(&mut seq);
    assert_eq!(seq1,[]);
}

#[test]
fn test_ins_sort_1(){
    let mut seq = [1];

    insertion_sort_asc(&mut seq);
    assert_eq!(seq,[1]);

    let seq1 = insertion_sort_desc(&mut seq);
    assert_eq!(seq1,[1]);
}

#[test]
fn test_ins_sort_multi(){
    let mut seq = [3,5,7,11,4,8,6,10,9];
    insertion_sort_asc(&mut seq);
    let val_ex: Vec<_> = (3..12).collect();
    assert_eq!(val_ex, seq);

    let seq1 = insertion_sort_desc(&mut seq);
    let val_ex1: Vec<_> = [11,10,9,8,7,6,5,4,3].to_vec();
    assert_eq!(val_ex1, seq1);

}

