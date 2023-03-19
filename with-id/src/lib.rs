#[cfg(feature = "derive")]
pub use with_id_derive as derive;


pub trait WithStrId<'a>{
    fn id(&'a self) -> &'a str;
}

pub trait WithStringId{
    fn id(&self) -> String;
}

pub trait WithRefId<T>{
    fn id(&self) -> &T;
}

pub trait WithId<T> where T:Clone{
    fn id(&self) -> T;
}

#[cfg(test)]
mod tests {
    use super::*;

    pub struct Test{
        pub id:String
    }

    impl<'a> WithStrId<'a> for Test{
        fn id(&'a self) -> &'a str {
            self.id.as_str()
        }
    }


    pub struct Test2{
        pub id:String,
        pub other: i64
    }

    impl WithIdRef<String> for Test{
        fn id(&self) -> &String {
            &self.id
        }
    }



    #[test]
    fn it_works() {
        let test = Test{
            id:"im-an-id".to_string()
        };
        assert_eq!(test.id(), test.id);


        let test = Test2{
            id:"im-an-id".to_string()
        };
        assert_eq!(test.id(), test.id);
    }
}
