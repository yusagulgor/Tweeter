use std::collections::HashMap;

#[derive(Debug,Clone)]
enum Bin {
    Zero,
    One
}

impl Bin {
    fn value(&self)->u8{
        match self {
            Bin::Zero =>0,
            Bin::One =>1,
        }
    }
}


#[derive(Debug,Clone,PartialEq)]
pub enum AdminLevel {
    Regulator =2, // title and tweet edit
    Author = 1, // tweet atabilen bir kullanıcı
    Possibleator= 3, // tweetleri okeyleyen 
    Moderator = 4, // herşeyi yöneten
    Customer = 0, // kullanıcı
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
    Not
}

impl Status {
    pub fn stat(&self) -> &str {
        match self {
            Status::Oke => "iyi durumda",
            Status::Editing => "düzenlenmeli",
            Status::Not => "silinmeli",
        }
    }
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

#[derive(Debug,Clone)]
pub struct User{
    pub(crate) name: String,
    pub(crate) email_adress:Email,
    pub(crate) adminlevel:AdminLevel,
    pub(crate) tweets:Vec<Tweet>,
    pub(crate) want_be_mod: bool,
}

#[derive(Debug,Clone)]
pub struct Tweeter{
    pub(crate) tweets:Vec<Tweet>,
    pub(crate) users: HashMap<String, User>,
    pub(crate) wants_tweets:Vec<Tweet>,
    pub(crate) wants_mod:Vec<User> // tüm yetkiler farketmeksizin
}