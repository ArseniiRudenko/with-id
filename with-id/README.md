# With id

Small crate containing a couple of traits providing id method.
Useful when you need to limit some other trait to types that have id field.
# Usage
```
[dependencies]
with-id = {version = "1", features=["derive"]}
```
# Example 
```
use with_id::WithRefId;

#[derive(WithRefId)]
struct Record{
    id: String,
    some_other: String
}

trait TakesRecord<T: WithRefId<str>>{
    fn get_endpoint(&self,record:T)->String;
}

struct Client{
    url:String
}

impl TakesRecord<Record> for Client{
    fn get_endpoint(&self,record:Record)->String{
        self.url.to_owned()+record.id()
    }
}

```
