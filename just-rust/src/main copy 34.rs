// Traits
// ->  The “Iterator” Standard Trait Declaration

fn main() {
    // struct MyRangeIterator<T> {
    //     current: T,
    //     limit: T,
    // }
    // impl Iterator for MyRangeIterator<u32> {
    //     // default Iterator trait from standard library
    //     type Item = u32;
    //     fn next(&mut self) -> Option<Self::Item> {
    //         if self.current == self.limit {
    //             None
    //         } else {
    //             self.current += 1;
    //             Some(self.current - 1)
    //         }
    //     }
    // }
    // print!(
    //     "{:?}; ",
    //     MyRangeIterator {
    //         current: 10,
    //         limit: 13,
    //     }
    //     .collect::<Vec<_>>()
    // );
    // for i in (MyRangeIterator {
    //     current: 20,
    //     limit: 24,
    // }) {
    //     print!("{} ", i);
    // }

    // -----  Using Generic Iterators

    /* Question
         We wanted to write a generic function named "get_third", which gets any
    iterator and returns the third item produced by such iterator, if possible, or otherwise
    "None"
         */
    // fn get_third<Iter>(mut iterator: Iter) -> Option<Iter::Item>
    // where
    //     Iter: std::iter::Iterator,
    // {
    //     iterator.next();
    //     iterator.next();
    //     iterator.next()
    // }
    // print!(
    //     "{:?} {:?} {:?} {:?}",
    //     get_third(10..12),
    //     get_third(20..29),
    //     get_third([31, 32].iter()),
    //     get_third([41, 42, 43, 44].iter())
    // );

    /*
        BUT,
        the standard
    library already contains a function even more generic than that: the "nth" iterator
    consumer. The following program is equivalent to the previous one
         */

    print!(
        "{:?} {:?} {:?} {:?}",
        (10..12).nth(2),
        (20..29).nth(2),
        ([31, 32].iter()).nth(2),
        ([41, 42, 43, 44].iter()).nth(2)
    );
}
