use std::collections::HashSet;

use super::*;

use rand::thread_rng;

#[test]
fn test_repeat_linear() {
    let mut set = HashSet::<i32>::new();
    let v = randge_linear(-100..100, 100, thread_rng());
    for i in v {
        assert!(set.insert(i));
    }
}

#[test]
fn test_repeat_tree() {
    let mut set = HashSet::<i32>::new();
    let v = randge_tree(-100..100, 100, thread_rng());
    for i in v {
        assert!(set.insert(i));
    }
}

#[test]
fn test_repeat_barrel() {
    let mut set = HashSet::<i32>::new();
    let v = randge_barrel(-100..100, 100, thread_rng());
    for i in v {
        assert!(set.insert(i));
    }
}