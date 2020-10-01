//Inspired by golang prograing blog post
pub fn binary_search<T: PartialEq + PartialOrd>(item: &T, a: &[T]) -> i32{
    let mut index = -1;
    let len = a.len();

    match len{
        0 => index,
        _ => {
            let mut l = 0;
            let mut r = len - 1;

            while l < r {
                let mid = l + (r - l) / 2;
                if &a[mid] > item {
                    r = mid - 1;
                } else if &a[mid] < item{
                    l = mid + 1;
                } else{
                    l = mid;
                    break;
                }
            }
            
            if &a[l] == item{
                index = l as i32;
                index
            }else{
                index
            }
        }
    }
}

#[test]
fn string_test(){
    let index = binary_search(&"grng3r", &vec!["grng3r","muad-dib", "ivan", "rugby"]);
    assert_eq!(0, index);
}

#[test]
fn int_test(){
    let index = binary_search(&25, &vec![6,89,0,89,93,102]);
    assert_eq!(3, index);
}
