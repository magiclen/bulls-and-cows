extern crate bulls_and_cows;

#[test]
fn random_answer() {
    let host = bulls_and_cows::Host::build_with_random_answer(
        bulls_and_cows::Letters::generate_numeric_letters(),
        4,
    )
    .unwrap();

    assert_eq!(4, host.get_answer_length());
}

#[test]
fn known_answer() {
    let host = bulls_and_cows::Host::build_with_known_answer(
        bulls_and_cows::Letters::generate_numeric_letters(),
        vec![1, 2, 3, 4],
    )
    .unwrap();

    assert_eq!((4, 0), host.answer(&vec![1, 2, 3, 4]).unwrap());
    assert_eq!((2, 2), host.answer(&vec![1, 2, 4, 3]).unwrap());
    assert_eq!((0, 4), host.answer(&vec![4, 3, 2, 1]).unwrap());
    assert_eq!((0, 0), host.answer(&vec![5, 6, 7, 8]).unwrap());
}
