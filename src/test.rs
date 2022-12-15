use crate::*;

#[test]
fn greater_end() {
    let range = RangeInclusive::new(0 , 1);
    assert!(!range.is_empty());
    for r in range.iter() {
        println!("{:?}", r);
    }
    println!("original range: {:?} \n", range);

    let range = RangeInclusive::new(10 , 17);
    assert!(!range.is_empty());
    for r in range.iter() {
        println!("{:?}", r);
    }
    println!("original range: {:?} \n", range);
    
    let range = RangeInclusive::new(3 , 22);
    assert!(!range.is_empty());
    for r in range.iter() {
        println!("{:?}", r);
    }
    println!("original range: {:?} \n", range);
    
    let range = RangeInclusive::new(597 , 782);
    assert!(!range.is_empty());
    for r in range.iter() {
        println!("{:?}", r);
    }
    println!("original range: {:?} \n", range);
}

#[test]
fn equal_start_end() {
    let range = RangeInclusive::new(0 , 0);
    assert!(!range.is_empty());
    for r in range.iter() {
        println!("{:?}", r);
    }
    println!("original range: {:?} \n", range);
    
    let range = RangeInclusive::new(597 , 597);
    assert!(!range.is_empty());
    for r in range.iter() {
        println!("{:?}", r);
    }
    println!("original range: {:?} \n", range);
}

#[test]
fn greater_start() {
    let range = RangeInclusive::new(782 , 597);
    assert!(range.is_empty());
    for r in range.iter() {
        println!("{:?}", r);
    }
    println!("original range: {:?} \n", range);
    
    let range = RangeInclusive::new(1 , 0);
    assert!(range.is_empty());
    for r in range.iter() {
        println!("{:?}", r);
    }
    println!("original range: {:?} \n", range);
}