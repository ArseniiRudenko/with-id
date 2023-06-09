use std::path::PathBuf;
use with_id::{WithStringId, WithId,WithRefId};

#[derive(WithStringId)]
struct Test1{
    pub id: i64,
    pub other: String
}

#[test]
fn test(){
    let t = Test1{
        id: 0,
        other: "".to_string(),
    };
    assert_eq!(t.id(),t.id.to_string())
}

#[derive(WithStringId)]
struct Test2{
    pub id: String,
    #[id]
    pub actual_id: f32,
    pub other: String
}

#[test]
fn test2(){
    let t = Test2{
        id: "abc".to_string(),
        actual_id: 12f32,
        other: "".to_string(),
    };
    assert_eq!(t.id(),t.actual_id.to_string())
}

#[derive(WithRefId)]
struct Test3{
    pub id: String
}

#[test]
fn test3(){
    let t = Test3{
        id: "abc".to_string()
    };
    assert_eq!(t.id(),t.id)
}


#[derive(WithRefId)]
struct Test4<'a, 'b> {
    pub id: &'a str,
    pub not_id: &'b str
}


#[test]
fn test4(){
    let t = Test4{
        id: "abc",
        not_id: "c",
    };
    assert_eq!(t.id(),t.id)
}



#[derive(WithId)]
struct Test5{
    #[id]
    pub pb: PathBuf
}

#[test]
fn test5(){
    let t = Test5{
        pb: PathBuf::from("/")
    };
    assert_eq!(t.id(),t.pb)
}


#[derive(WithId)]
struct Test6{
    #[id]
    pub int: i32
}

#[test]
fn test6(){
    let t = Test6{
        int: 12i32
    };
    assert_eq!(t.id(),t.int)
}

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


#[derive(WithRefId)]
struct RecordWithLifetimes<'a>{
    id: String,
    some_other: &'a str
}

#[test]
fn test7(){
    let t = RecordWithLifetimes{
        id: "as".to_string(),
        some_other: "a"
    };
    assert_eq!(t.id(),t.id)
}

#[derive(WithStringId)]
struct StrRecordWithLifetimes<'a>{
    id: String,
    some_other: &'a str
}

#[test]
fn test8(){
    let t = StrRecordWithLifetimes{
        id: "as".to_string(),
        some_other: "a"
    };
    assert_eq!(t.id(),t.id)
}


#[derive(WithId)]
struct IdRecordWithLifetimes<'a>{
    id: String,
    some_other: &'a str
}

#[test]
fn test9(){
    let t = IdRecordWithLifetimes{
        id: "as".to_string(),
        some_other: "a"
    };
    assert_eq!(t.id(),t.id)
}