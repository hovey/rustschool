#[test]
fn iterator_demonstration() {
    let v1 = vec![10, 11, 12];
    let mut v1_iter = v1.iter();

    assert_eq!(v1_iter.next(), Some(&10));
    assert_eq!(v1_iter.next(), Some(&11));
    assert_eq!(v1_iter.next(), Some(&12));
    assert_eq!(v1_iter.next(), None);

    let v2 = vec![1, 2, 4];
    // A consuming adaptor is a method that calls next.  Calling
    // them uses up the iterator.
    let total: i32 = v2.iter().sum();
    assert_eq!(total, 7);

    // Iterator adaptor are methods on the Iterator trait that
    // don't consume the iterator.  Instead, they produce different
    // iterators by changing some aspect of the original iterator.
    let v3 = vec![1, 2, 3];
    // Just after this point, we get a warning that "iterators are
    // lazy and do nothing unless consumed".  We need to consume
    // the iterator.  To fix this warning, we can use the collect
    // method, which consumes the iterator and collects the resulting
    // values intoa data type.
    let v4: Vec<_> = v3.iter().map(|x| x + 1).collect();
    assert_eq!(v4, vec![2, 3, 4]);
}

#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    // we call into_iter to create an iterator that takes ownership
    // of the vector, then call filter to adapt that iterators into
    // a new iterator that only contains elements for which the
    // closure returns true,
    // and
    // calling collect gathers the values returned by the adapter
    // iterator into a vector that's retuned by the function.
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filters_by_size() {
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 13,
                style: String::from("sandal"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ];

        let in_my_size = shoes_in_size(shoes, 10);

        assert_eq!(
            in_my_size,
            vec![
                Shoe {
                    size: 10,
                    style: String::from("sneaker"),
                },
                Shoe {
                    size: 10,
                    style: String::from("boot"),
                },
            ]
        )
    }
}
