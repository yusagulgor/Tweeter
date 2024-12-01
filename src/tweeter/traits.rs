use std::collections::HashMap;
use super::types::*;

pub trait TweetT {
    fn full_show(&self);
    fn customer_show(&self);
    fn short_show(&self);
}

pub trait TweeterT {
    fn get_xid_wants_tweet(arr:&mut Vec<Tweet>,id:u8)->Option<&mut Tweet>;
    fn get_xid_user(users: &mut HashMap<u8, User>, id: u8) -> Option<&mut User>;
    fn show_tweets_c(&self);
    fn show_tweets_a(&self);
    fn short_show_tweets(&self);
    fn show_want_tweets(&self);
    fn want_mod_users(&self);
    fn get_users(&self);
}

pub trait UserT {
    fn read_all_your_tweet(&self,t: &Tweeter);
    fn write_tweet(&mut self, t: &mut Tweeter,title:String,description:String,tweet_message:String)->String;
}

// pub trait StatT {
//     fn str_to_stat(value:&str)->Status;
// }
