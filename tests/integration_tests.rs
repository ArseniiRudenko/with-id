use std::path::PathBuf;
use with_id::{WithStringId, WithId,WithRefId};
use with_id::derive::{WithStringId,WithId,WithRefId};

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

