use std::cmp::PartialOrd;

/// Quicksort
/// Simple sort algorithm that is type aware and is unique in that it's test driven
/// 
fn partition<T: PartialOrd>(elements: &mut [T]) -> usize {
    // Explictly set the pivot to the lowest element
    let mut i : usize;
    let mut j : usize;

    if elements.is_empty() {
        return 0;
    }

    let high = elements.len();

    i = 0;
    j = high - 1;

    while i < j {
        while elements[i] <= elements[0] && i < (high-1) { i += 1; } // Bounds 2
        while elements[j] >= elements[0] && j > 0 { j -= 1; } // Bounds 3
        
        if i >= j {break; } // Bounds 4
        
        elements.swap(i, j); // Check 5
    }
    elements.swap(0, j); // Check 6
    j
}

fn quicksort<T: PartialOrd + std::fmt::Debug>(elements: &mut [T])
{
    let low = 0;
    let high = elements.len();

    if elements.is_empty() {
        return;
    }

    let p = lomuto_partition(&mut elements[low..high]);
    // p - 1 should not be negative
    quicksort(&mut elements[low..p]);
    quicksort(&mut elements[p+1..high]);
}

///
/// Use the algorithm that has fewer boundary conditions to worry about
fn lomuto_partition<T: std::cmp::PartialOrd>(elements: &mut [T]) -> usize
{
    let mut i: usize = 0;
    let size: usize = elements.len();
    let mut j : usize = 0;

    if elements.is_empty()  { // bounds 1
        return 0;
    }

    while i < size-1 { // bounds 3
        i += 1;
        if elements[i] <= elements[0] { // bounds 4
            j += 1;
            elements.swap(i, j);
        }
    }

    elements.swap(0, j);
    j
}



#[test]
fn partition_bounds_test_single()
{
    let mut elements = vec![1];
    assert_eq!(0, partition(&mut elements));
}

#[test]
fn partition_bounds_test_mostly_sorted()
{
    let mut elements = vec![10, 2, 4, 5, 6];
    let expected = vec![6, 2, 4, 5, 10];

    assert_eq!(4, partition(&mut elements));
    assert_eq!(expected, elements);

    let mut elements = vec![2, 4, 5, 6, 10];
    let expected = vec![2, 4, 5, 6, 10];

    
    assert_eq!(0, partition(&mut elements));
    assert_eq!(expected, elements);
}

#[test]
fn partition_bounds_test_2_elements_sorted()
{
    let mut elements = vec![15, 10];
    let expected = vec![10, 15];

    assert_eq!(1, partition(&mut elements));
    assert_eq!(expected, elements);

    let mut elements = vec![10, 15];
    let expected = vec![10, 15];

    assert_eq!(0, partition(&mut elements));
    assert_eq!(expected, elements);

    let mut elements = vec![15, 5, 10];
    let expected = vec![10, 5, 15];

    assert_eq!(2, partition(&mut elements));
    assert_eq!(expected, elements);
}

#[test]
fn quicksort_test()
{
    let mut vec = vec![1, 5, 10, 2, 15];
    let mut vec2 = vec.clone();

    quicksort(&mut vec);
    vec2.sort();
    assert_eq!(vec![1, 2, 5, 10, 15], vec);
    assert_eq!(vec2, vec);
    
    let mut vec = vec![1.1, 1.15, 5.5, 1.123, 2.0];
    quicksort(&mut vec);
    assert_eq!(vec, vec![1.1, 1.123, 1.15, 2.0, 5.5]);

    let mut vec = vec![1, 5, 10, 15, 2];
    let mut vec2 = vec.clone();

    quicksort(&mut vec);
    vec2.sort();
    assert_eq!(vec2, vec);

    let mut vec = vec![15, 5, 10];
    let mut vec2 = vec.clone();

    quicksort(&mut vec);
    vec2.sort();
    assert_eq!(vec2, vec);


    let mut vec = vec![1, 2, 3, 4, 5];
    let mut vec2 = vec.clone();

    quicksort(&mut vec);
    vec2.sort();
    assert_eq!(vec2, vec);

    let mut vec = vec![5, 4, 3, 2, 1];
    let mut vec2 = vec.clone();

    quicksort(&mut vec);
    vec2.sort();
    assert_eq!(vec2, vec);

}

fn main()
{
    
}