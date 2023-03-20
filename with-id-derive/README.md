# With id derive

Derive macros for with-id crate.
See docs for with-id.

# Implementation details
- When deriving implementation for WithRefId, for String field macro will generate &str return type, 
so type of derived implementation will be WithRefId\<str>, not WithRefId\<String>
- Macro does not check if type implements ToString (in case of WithStringId) and Clone in case of  WithId, and will fail if it doesn't