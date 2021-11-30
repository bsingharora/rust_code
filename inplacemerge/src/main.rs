use std::mem::swap;
extern crate rand;

use rand::{thread_rng, prelude::SliceRandom};


///
/// In place merge of two sorted elements, hopefully test driven
/// 

/// Given slices of type `T`, merge elements from `left` and `right`
/// in place. Assumptions, left to right form one contiguous stream of
/// elements, merging will result is `left` pointing to the fully sorted
/// array.
/// 
fn in_place_merge<T: std::cmp::PartialOrd>(left: &mut [T], right: &mut [T])
{
    let left_size = left.len();
    let right_size = right.len();

    let mut i = 0;
    let mut k = 0;

    if left.is_empty() || right.is_empty() {
        return;
    }    

    while true {
        if i < left_size && left[i] < right[k] {
            i += 1;
        } else {
            swap(&mut left[i], &mut right[k]);
            if k < right_size-1 && right[k] > right[k+1] && right[k] > right[0] {
                // Split brain, best to merge
                let (tmp_left, tmp_right) = right.split_at_mut(k+1);
                in_place_merge(tmp_left, tmp_right);
                k = 0;
            } else if k < right_size-1 && right[k] > right[k+1] {
                k += 1;
            }
        }

        if i == left_size {
            if k != 0 && k < right_size {
                let (new_left, new_right) = right.split_at_mut(k); // right[0] points to the array with the larger elements
                in_place_merge(new_left, new_right);
                
            }
            break;
        }
    }

}

#[test]
fn test_single_elements()
{
    let mut vec1 = vec![1];
    let mut vec2 = vec![2];

    in_place_merge(&mut vec1, &mut vec2);
    assert_eq!(vec![1], vec1);
    assert_eq!(vec![2], vec2);

}

#[test]
fn test_asymetric_elements_left_right_one_or_two()
{
    let mut vec1 = vec![1, 3];
    let mut vec2 = vec![2];

    in_place_merge(&mut vec1, &mut vec2);
    assert_eq!(vec![1, 2], vec1);
    assert_eq!(vec![3], vec2);

    let mut vec1 = vec![1];
    let mut vec2 = vec![2, 3];

    in_place_merge(&mut vec1, &mut vec2);
    assert_eq!(vec![1], vec1);
    assert_eq!(vec![2, 3], vec2);

    let mut vec1 = vec![2];
    let mut vec2 = vec![1, 3];

    in_place_merge(&mut vec1, &mut vec2);
    assert_eq!(vec![1], vec1);
    assert_eq!(vec![2, 3], vec2);

}

#[test]
fn test_asymetric_elements_left_one_right_many()
{
    let mut vec1 = vec![1];
    let mut vec2 = vec![2, 5, 7, 9, 10];

    in_place_merge(&mut vec1, &mut vec2);
    assert_eq!(vec![1], vec1);
    assert_eq!(vec![2, 5, 7, 9, 10], vec2);

    let mut vec1 = vec![10];
    let mut vec2 = vec![2, 5, 7, 9];

    in_place_merge(&mut vec1, &mut vec2);
    assert_eq!(vec![2], vec1);
    assert_eq!(vec![5, 7, 9, 10], vec2);
}

#[test]
fn test_alternate_sequence_odd_first()
{
    let mut vec1 = vec![1, 3, 5, 7];
    let mut vec2 = vec![2, 4, 6, 8];

    in_place_merge(&mut vec1, &mut vec2);
    assert_eq!(vec![1, 2, 3, 4], vec1);
    assert_eq!(vec![5, 6, 7, 8], vec2);
}

#[test]
fn test_alternate_sequence_even_first()
{

    let mut vec1 = vec![2, 4, 6, 8];
    let mut vec2 = vec![1, 3, 5, 7];

    in_place_merge(&mut vec1, &mut vec2);
    assert_eq!(vec![1, 2, 3, 4], vec1);
    assert_eq!(vec![5, 6, 7, 8], vec2);

}

#[test]
fn test_different_sizes_off_by_two_left_larger()
{
    let mut vec1 = vec![2, 4, 5, 7, 8, 9];
    let mut vec2 = vec![0, 1, 3, 6];

    in_place_merge(&mut vec1, &mut vec2);
    assert_eq!(vec![0, 1, 2, 3, 4, 5], vec1);
    assert_eq!(vec![6, 7, 8, 9], vec2);
}

#[test]
fn random_test_one()
{
    let len = 10000;
    let mut vec: Vec<u32> = (0..len).collect();
    let vec_copy = vec.clone();

    vec.shuffle(&mut thread_rng());
    let (vec1, vec2) = vec.split_at_mut((len/2) as usize);
    vec1.sort();
    vec2.sort();
    
    in_place_merge(vec1, vec2);
    assert_eq!(vec, vec_copy);
}

fn main() {
    let mut vec1 = vec![1];
    let mut vec2 = vec![2];

    in_place_merge(&mut vec1, &mut vec2);
    println!("{:?},{:?}", vec1, vec2);
}
