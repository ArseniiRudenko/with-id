# with-str-id
Bunch of auxiliary traits providing id method that gets value from some struct field and macro derivations for them.

- WithStrId - trait that returns str ref as id for some string field in the struct

- WithStringId - trait that returns owned string for some field, expects that filed type imlements ToString

- WithRefId<T> - generic trait that returns reference to value of type T from the struct

- WithId<T> - generic trait that returns owned value of type T from the struct, expects T to implement Clone

Refer to tests for usage examples.
