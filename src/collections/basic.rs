

pub fn basic_tuple() {
    // Tuples can contain various, different static types.
    let some_tuple = (40_000, "Hello");
    // example of tuple unpacking
    let (salary, some_str) = some_tuple;
    // we can also access by index.
    let salary = some_tuple.0;
    let some_str = some_tuple.1;
    // Main use case: returning multiple values from a function.

    // Empty tuple
    let empty_tuple = ();
}
