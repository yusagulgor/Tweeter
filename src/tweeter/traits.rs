use super::types::*;

pub trait TweeterT {
    fn show_tweets_c(&self);
    fn get_users(&self);
    fn show_tweets_a(&self);
    fn short_show_tweets(&self);
    fn show_want_tweets(&self);
}

pub trait TweetT {
    fn full_show(&self);
    fn customer_show(&self);
    fn short_show(&self);
}

pub trait UserT {
    fn read_all_your_tweet(&self,t: &Tweeter);
    fn write_tweet(&mut self, t: &mut Tweeter,title:String,description:String,tweet_message:String)->String;
}
