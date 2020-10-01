/*inspiration from cbzehner and nbigaouette git implementations to implement merge sort in place(sort of, temporary array merged)*/
pub fn merge_sort_asc_inpl<T: Copy + PartialOrd>(input: &mut[T] ) {
    let len = input.len();
    //nothing to do
    if len < 2 || len == 0{
        return;
    }else if len == 2 {
        //manual sort
        if input.first() > input.last(){
            input.swap(0,1);
        }
    }else{
        //divide input in half
        let m = len/2;

        //allocate vector with the capacity of input size,to hold sorted input sizes temp
        //prevents moving the elements inside the input vector directly & makes the implementation
        //O(N) for space complexity
        let mut merged = Vec::with_capacity(len);

        //new code block enforcing the lifetime of lef & right, which otherwise would continue the
        //ownership of merge, required to copy back to input
        {
            //split vector in 2 & sorteach half recursevly
            let (left, right) = input.split_at_mut(m);
            merge_sort_asc_inpl(left);
            merge_sort_asc_inpl(right);
            
            //keep peekable iterr into l & r slices, we use peekable as we must not iterrate over
            //each slice every loop iteration
            let mut iter_left = left.iter().peekable();
            let mut iter_right = right.iter().peekable();
            
            //iterate throu l & r slices finding the next lowest value
            while let(Some(&l), Some(&r)) = (iter_left.peek(), iter_right.peek()){
                // if value from r smaller than val from l collect r, else collect r
                if *l <= *r {
                    merged.push(*(iter_left.next().unwrap()));
                } else{
                    merged.push(*(iter_right.next().unwrap()));
                }
            }

            //if any values remain in right or left push them on to merged
            for l in iter_left{
                merged.push(*l);
            }

            for r in iter_right{
                merged.push(*r);
            }

            //copy values of merged into input
            assert!(merged.len() == input.len(), "All input elements should be in the sorted output");
        }

        for n in 0..input.len(){
            input[n] = merged[n];
        }

    }
}


pub fn merge_sort_desc_inpl<T: Copy + PartialOrd>(input: &mut[T]) {
    //nothing to do
    if input.len() < 2 || input.len() == 0{
        return;
    }else if input.len() == 2 {
        //manual sort
        if input.first() < input.last(){
            input.swap(0,1);
        }
    }else{
        //divide input in half
        let m = input.len()/2;

        //allocate vector with the capacity of input size,to hold sorted input sizes temp
        //prevents moving the elements inside the input vector directly & makes the implementation
        //O(N) for space complexity
        let mut merged = Vec::with_capacity(input.len());

        //new code block enforcing the lifetime of lef & right, which otherwise would continue the
        //ownership of merge, required to copy back to input
        {
            //split vector in 2 & sorteach half recursevly
            let (left, right) = input.split_at_mut(m);
            merge_sort_desc_inpl(left);
            merge_sort_desc_inpl(right);
            
            //keep peekable iterr into l & r slices, we use peekable as we must not iterrate over
            //each slice every loop iteration
            let mut iter_left = left.iter().peekable();
            let mut iter_right = right.iter().peekable();
            
            //iterate throu l & r slices finding the next lowest value
            while let(Some(&l), Some(&r)) = (iter_left.peek(), iter_right.peek()){
                // if value from r larger than val from l collect r, else collect r
                if *l >= *r {
                    merged.push(*(iter_left.next().unwrap()));
                } else{
                    merged.push(*(iter_right.next().unwrap()));
                }
            }

            //if any values remain in right or left push them on to merged
            for l in iter_left{
                merged.push(*l);
            }

            
            for r in iter_right{
                merged.push(*r);
            }

            //copy values of merged into input
            assert!(merged.len() == input.len(), "All input elements should be in the sorted output");
        }

        for n in 0..input.len(){
            input[n] = merged[n];
        }
    }
} 

pub fn merge_sort_asc(mut input:Vec<usize>) -> Vec<usize>{
    merge_sort_asc_inpl(&mut input);
    input
}

pub fn merge_sort_desc(mut input:Vec<usize>) -> Vec<usize>{
    merge_sort_desc_inpl(&mut input);
    input
}



#[test]
fn zero_vec(){
    let test_seq: [usize;0] = [];
    let seq1 = merge_sort_asc(test_seq.to_vec());
    assert_eq!(seq1,  test_seq);
    let seq2 = merge_sort_desc(test_seq.to_vec());
    assert_eq!(seq2, test_seq);
}

#[test]
fn base_case_vec(){
    let test_seq = [6];
    let seq1 = merge_sort_asc(test_seq.to_vec());
    assert_eq!(seq1, test_seq);
    let seq2 = merge_sort_desc(test_seq.to_vec());
    assert_eq!(seq2, [6]);
}

#[test]
fn multi_value_vec(){
    let test_seq = [89,32,4,11,44,1,9,0,22];
    let seq1 = merge_sort_asc(test_seq.to_vec());
    assert_eq!(seq1, [0,1,4,9,11,22,32,44,89]);
    let seq2 = merge_sort_desc(test_seq.to_vec());
    assert_eq!(seq2, [89,44,32,22,11,9,4,1,0]);
}

#[test]
fn already_sorted_vec(){
    let test_seq = [6,8,10,54,55];
    let test_seq1 = [55,54,10,8,6];
    let seq1 = merge_sort_asc(test_seq.to_vec());
    assert_eq!(seq1, test_seq);
    let seq2 = merge_sort_desc(test_seq1.to_vec());
    assert_eq!(seq2, test_seq1);
}



