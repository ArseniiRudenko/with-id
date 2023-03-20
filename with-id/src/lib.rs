#[cfg(feature = "derive")]
pub use with_id_derive::*;


pub trait WithStringId{
    fn id(&self) -> String;
}

pub trait WithRefId<T: ?Sized>{
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

    impl WithId<String> for Test{
        fn id(&self) -> String {
            self.id.to_string()
        }
    }


    pub struct Test2{
        pub id:String,
        pub other: i64
    }

    impl WithRefId<str> for Test2{
        fn id(&self) -> &str {
          self.id.as_str()
        }
    }



    #[test]
    fn it_works() {
        let test = Test{
            id:"im-an-id".to_string()
        };
        assert_eq!(test.id(), test.id);

        let test = Test2{
            id:"im-an-id".to_string(),
            other: 4i64,
        };
        assert_eq!(test.id(), test.id);
    }
}
