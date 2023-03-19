# with-id
Bunch of auxiliary traits providing id method that gets value from some struct field and macro derivations for them.

- WithStringId - trait that returns owned String for some field, expects that field type implements ToString

- WithRefId<T> - generic trait that returns reference to value of type T from the struct. Returns &str in case of String.

- WithId<T> - generic trait that returns owned value of type T from the struct, expects T to implement Clone

Refer to tests for usage examples.
