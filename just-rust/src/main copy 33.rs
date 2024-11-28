// Traits
// -> Generic traits

fn main() {
    /*
         we would like to write a
    generic function named "is_present", which takes as arguments a generic collection
    and a numeric search key, and returns whether that key has been found in that
    collection
         */

    // //
    // trait Searchable<Key> {
    //     fn contains(&self, key: Key) -> bool;
    // }
    // struct RecordWithId {
    //     id: u32,
    //     _descr: String,
    // }
    // struct NameSetWithId {
    //     data: Vec<RecordWithId>,
    // }
    // impl Searchable<u32> for NameSetWithId {
    //     fn contains(&self, key: u32) -> bool {
    //         for record in self.data.iter() {
    //             if record.id == key {
    //                 return true;
    //             }
    //         }
    //         false
    //     }
    // }
    // // funtion
    // fn is_present<Collection>(coll: &Collection, id: u32) -> bool
    // where
    //     Collection: Searchable<u32>,
    // {
    //     coll.contains(id)
    // }
    // //
    // let names = NameSetWithId {
    //     data: vec![
    //         RecordWithId {
    //             id: 34,
    //             _descr: "John".to_string(),
    //         },
    //         RecordWithId {
    //             id: 49,
    //             _descr: "Jane".to_string(),
    //         },
    //     ],
    // };
    // print!("{} {}", is_present(&names, 48), is_present(&names, 49));

    /*
         This solution works, but is has some drawbacks.
     One is this.
     - The needed implementation of the "Searchable" trait had to be specific
    for the search key type "u32", and so it had to specify it as type parameter value, but in  the "where" clause of the "is_present" function declaration, such a type was pecified again as a type parameter value. It seems a useless repetition, and a future type change requires a double edit
         */

    // //-   But now consider a more complex situation. where two type parameterization required
    // trait Searchable<Key, Count> {
    //     fn contains(&self, key: Key) -> bool;
    //     fn count(&self, key: Key) -> Count;
    // }
    // struct RecordWithId {
    //     id: u32,
    //     _descr: String,
    // }
    // struct NameSetWithId {
    //     data: Vec<RecordWithId>,
    // }
    // impl Searchable<u32, usize> for NameSetWithId {
    //     fn contains(&self, key: u32) -> bool {
    //         for record in self.data.iter() {
    //             if record.id == key {
    //                 return true;
    //             }
    //         }
    //         false
    //     }
    //     fn count(&self, key: u32) -> usize {
    //         let mut c = 0;
    //         for record in self.data.iter() {
    //             if record.id == key {
    //                 c += 1;
    //             }
    //         }
    //         c
    //     }
    // }

    // //
    // fn is_present<Collection>(coll: &Collection, id: u32) -> bool
    // where
    //     Collection: Searchable<u32, usize>,
    // {
    //     coll.contains(id)
    // }

    // //
    // let names = NameSetWithId {
    //     data: vec![
    //         RecordWithId {
    //             id: 34,
    //             _descr: "John".to_string(),
    //         },
    //         RecordWithId {
    //             id: 49,
    //             _descr: "Jane".to_string(),
    //         },
    //     ],
    // };

    // // use
    // print!(
    //     "{}, {}; {} {}",
    //     names.count(48),
    //     names.count(49),
    //     is_present(&names, 48),
    //     is_present(&names, 49)
    // );
    // //Here the "Searchable" generic trait has a new function signature, a new type parameter, needed by such function .What is not so obvious is that the signature of the "is_present" generic function must also specify a type for the new trait parameter.

    // solutionv: Using Associated Types to Simplify Generic  Traits Use
    struct RecordWithId {
        id: u32,
        _descr: String,
    }
    struct NameSetWithId {
        data: Vec<RecordWithId>,
    }

    trait Searchable {
        type Key; //2
        type Count; //3
        fn contains(&self, key: Self::Key) -> bool; //4
        fn count(&self, key: Self::Key) -> Self::Count; //5
    }
    impl Searchable for NameSetWithId {
        // for every parameter it has it's own implementation, i guess
        //6
        type Key = u32; //7
        type Count = usize; //8
        fn contains(&self, key: Self::Key) -> bool {
            //9
            for record in self.data.iter() {
                if record.id == key {
                    return true;
                }
            }
            false
        }
        fn count(&self, key: Self::Key) -> usize {
            //10
            let mut c = 0;
            for record in self.data.iter() {
                if record.id == key {
                    c += 1;
                }
            }
            c
        }
    }

    //funtion
    fn is_present<Collection>(coll: &Collection, id: <Collection as Searchable>::Key,) -> bool
    where
        Collection: Searchable, //12
    {
        coll.contains(id)
    }
    

    let names = NameSetWithId {
        data: vec![
            RecordWithId {
                id: 34,
                _descr: "John".to_string(),
            },
            RecordWithId {
                id: 49,
                _descr: "Jane".to_string(),
            },
        ],
    };
    print!(
        "{}, {}; {} {}",
        names.count(48),
        names.count(49),
        is_present(&names, 48),
        is_present(&names, 49)
    );
}
