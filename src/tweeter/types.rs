use std::collections::HashMap;

// #[derive(Debug,Clone)]
// enum Bin {
//     Zero,
//     One
// }

// impl Bin {
//     fn value(&self)->u8{
//         match self {
//             Bin::Zero =>0,
//             Bin::One =>1,
//         }
//     }
// }

// pub enum a{
//     Description(String),
//     Title(String),
//     Tweet(String),
//     Id(u8),
//     Author(String),
//     Status(Status)
// }

// let value = a::Id(8);



#[derive(Debug,Clone,PartialEq)]
pub enum AdminLevel {
    
    Customer = 0, // kullanıcı
    Author = 1, // tweet atabilen bir kullanıcı
    Possibleator= 2, // tweetleri okeyleyen 
    Regulator =3, // title and tweet edit
    Moderator = 4, // herşeyi yöneten

}


#[derive(Debug,Clone,PartialEq,Eq)]
pub enum Mail {
    Gmail,
    Hotmail,
    Outlook,
}

pub static VALID_DOMAINS: &[(&str, Mail)] = &[
    ("gmail.com", Mail::Gmail),
    ("hotmail.com", Mail::Hotmail),
    ("outlook.com", Mail::Outlook),
];

#[derive(Debug,Clone,PartialEq,Eq)]
pub struct Email {
    pub mail_type: Mail,
    pub address: String,
}

#[derive(Debug, Clone,PartialEq,Eq,Hash)]
pub enum Status{
    Oke,
    Editing,
    Edited,
    Not
}

#[derive(Debug,Clone,PartialEq)]
pub enum EditableTweetSection{
    Description(String),
    Title(String),
    Tweet(String),
    Id(u8),
    Author(String),
    Status(Status)
}

#[derive(Debug,Clone,PartialEq,Eq,Hash)]
pub struct Tweet {
    pub(crate) id: u8,
    pub(crate) author: String,
    pub(crate) title: String,
    pub(crate) description: String,
    pub(crate) tweet: String,
    pub(crate) status: Status,
}

#[derive(Debug,Clone,PartialEq)]
pub struct User{
    pub(crate) id:u8,
    pub(crate) name: String,
    pub(crate) email_adress:Email,
    pub(crate) adminlevel:AdminLevel,
    pub(crate) tweets:Vec<Tweet>,
    pub(crate) want_be_mod: bool,
}

#[derive(Debug,Clone,PartialEq)]
pub struct Tweeter{
    pub(crate) tweets:Vec<Tweet>,
    pub(crate) users: HashMap<u8, User>,
    pub(crate) wants_tweets:Vec<Tweet>,
    pub(crate) wants_mod:Vec<User> // tüm yetkiler farketmeksizin
}