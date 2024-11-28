use super::{traits::*,types::*,utils::*};
use std::{collections::{HashMap, HashSet}, fmt::{Display, Formatter}};
use rand::Rng;

impl AdminLevel {
    pub fn level_name(&self)->&str{
        match self {
            AdminLevel::Author => "Author", // ok
            AdminLevel::Customer => "Customer", // ok
            AdminLevel::Possibleator => "Possibleator",
            AdminLevel::Moderator => "Moderator", // ok
            AdminLevel::Regulator => "Regulator",
        }
    }

    pub fn level_value(&self) -> u8 {
        self.clone() as u8
    }
}

impl Email {
    pub fn new(mail_type: Mail, address: String) -> Result<Self, String> {
        if address.trim().is_empty() {
            return Err("Address cannot be empty".to_string());
        }

        if address.len() > 16{
            return Err("mail ismi 16 karakterden büyük olamaz .".to_string());
        }

        Ok(Self { mail_type, address })
    }

    pub fn mail_address(&self) -> String {
        format!("{}@{}", self.address, self.mail_type.domain())
    }
}

impl Tweet {
    fn new(
        id: u8,
        author: String,
        title: String,
        description: String,
        tweet: String,
        status: Status,
    ) -> Result<Self, String> {
        if title.len() > 10 {
            return Err("Başlık 10 karakterden uzun olamaz.".to_string());
        }

        if description.len() > 25 {
            return Err("Açıklama 25 karakterden uzun olamaz.".to_string());
        }

        if tweet.len() > 220 {
            return Err("Tweet 30 karakterden uzun olamaz.".to_string());
        }

        Ok(Self {
            id,
            author,
            title,
            description,
            tweet,
            status,
        })
    }
}

impl TweetT for Tweet {

    fn full_show(&self){
        println!("id :{}",self.id);
        println!("Author: {}",self.author);
        println!("description : {}",self.description);
        println!("tweet : {}",self.tweet);
        println!("status : {}",self.status.stat());
    }

    fn customer_show(&self){
        println!("Author :{} , Title : {}",self.author,self.title);
        println!("Tweet :{}",self.tweet);
    }

    fn short_show(&self){
        println!("Author: {}",self.author);
        println!("description: {}",self.description);
    }
}

impl Display for User {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "name: {}, email address: {}, level: {}",
            self.name,
            self.email_adress.mail_address(),
            self.adminlevel.level_name()
        )
    }
}

impl User {
    pub fn new(name: String, email_adress: Email, adminlevel: AdminLevel) -> Self {
        Self {
            name,
            email_adress,
            adminlevel,
            tweets: Vec::new(),
            want_be_mod: false,
        }
    }

}

impl Tweeter{
    pub fn new() -> Self {
        Self {
            users: HashMap::new(),
            tweets:Vec::new(),
            wants_tweets:Vec::new(),
            wants_mod:Vec::new(),
        }
    }

    pub fn login_check(&self, user: &User) {
        if let Some((_, existing_user)) = self.users.iter().find(|(_, u)| u.name == user.name && u.email_adress.mail_address() == user.email_adress.mail_address()) {
            println!("Giriş onaylandı: Hoş geldiniz, {}!", existing_user.name);
        } else {
            println!("Giriş başarısız: Kullanıcı adı veya e-posta hatalı.");
        }
    }
    
    pub fn add_new_tweet(&mut self, tweet: &Tweet) -> &str {
        // if tweet.status != Status::Oke {
        //     return "Bu tweet düzenlenmeli.";
        // }
        self.wants_tweets.retain(|t| t.id != tweet.id);
        self.tweets.push(tweet.clone()); 
        "Tweet başarıyla eklendi."
    }
    
    pub fn accept_wtweets(&mut self, user: &User, id: u8) -> &str {
        if user.adminlevel != AdminLevel::Possibleator && user.adminlevel != AdminLevel::Moderator {
            return "Bunun için yetkiniz yok.";
        }
        
        if let Some(tweet_request) = self.wants_tweets.iter().find(|wt| wt.id == id) {
            let mut updated_tweet = tweet_request.clone();
            updated_tweet.status = Status::Oke;
            
            self.add_new_tweet(&updated_tweet);
            "Tweet isteği kabul edildi."
        } else {
            "O id'ye sahip tweet bulunamadı."
        }
    }
    

    pub fn add_want_tweets(&mut self,tweet:&Tweet)->&str{
        self.wants_tweets.push(tweet.clone());
        return "tweet eklendi";
    }

    pub fn add_user(&mut self, user: &User) -> &str {
        if let Some(existing_user) = self.users.get(&user.email_adress.mail_address()) {
            return "Bu e-posta zaten bir kullanıcı tarafından kullanılıyor.";
        }

        if user.adminlevel == AdminLevel::Moderator {
            if self.users.values().any(|u| u.adminlevel == AdminLevel::Moderator) {
                return "Zaten bir moderatör var. Başka bir moderatör eklenemez.";
            }
        }

        self.users.insert(user.email_adress.mail_address(), user.clone());
        return "Kullanıcı başarıyla eklendi.";
    }

    pub fn random_tweet(&self){
        if self.tweets.is_empty() {cep("Tweet listesi boş.");} else {            
            let mut rng = rand::thread_rng();
            let random_index = rng.gen_range(0..self.tweets.len());
            let selected_tweet = &self.tweets[random_index];

            println!("Rastgele seçilen tweet:");
            selected_tweet.customer_show()
        }
    }
}


impl UserT for User {
    fn read_all_your_tweet(&self, t: &Tweeter) {
        let user_tweets_set: HashSet<_> = self.tweets.iter().collect();
        let tweeter_tweets_set: HashSet<_> = t.tweets.iter().collect();
        let intersection: Vec<_> = user_tweets_set
            .intersection(&tweeter_tweets_set)
            .collect();

        if intersection.is_empty() {
            println!("\nHiçbir tweetiniz Tweeter'da bulunmuyor.");
        } else {
            println!("\nTweeter'da bulunan tweetleriniz:");
            for tweet in intersection {
                tweet.customer_show();
            }
        }
    }

    fn write_tweet(&mut self, t: &mut Tweeter,title:String,description:String,tweet_message:String) -> String {
        if self.adminlevel.level_value() < 1 {
            return "Levelin tweet atmaya yetmiyor. Tweet atmak istiyorsan başvur.".to_string();
        }

        if title.len() < 0 || description.len() <0 || tweet_message.len() <4 {
            return "lütfen gerekli kısımları boş bırakmayın. Tweet mesajınız en az 4 karakter olmalı .".to_string();
        }

        match Tweet::new(
            (t.wants_tweets.len()+1).try_into().unwrap(),
            self.name.clone(),
            title,
            description,
            tweet_message,
            Status::Editing,
        ) {
            Ok(new_tweet) => {
                self.tweets.push(new_tweet.clone());
                t.add_want_tweets(&new_tweet);
                "Tweet başarıyla oluşturuldu ve onay bekleyen tweetler listesine eklendi.".to_string()
            }
            Err(error_message) => error_message,
        }
    }
}

impl TweeterT for Tweeter{
    fn show_tweets_c(&self) {
        if self.tweets.len() !=0{
            for tweet in &self.tweets{
                tweet.customer_show();
                println!("-------------");
            }
        }else{
            println!("henüz hiç tweet atılmamış")
        }
    }

    fn show_tweets_a(&self) {
        if self.tweets.len() !=0{
            for tweet in &self.tweets{
                tweet.full_show();
                println!("-------------");
            }
        }else{
            println!("henüz hiç tweet atılmamış")
        }
    }
    
    fn short_show_tweets(&self) {
        if self.tweets.len() !=0{
            for tweet in &self.tweets{
                tweet.short_show();
                println!("-------------");
            }
        }else{
            println!("henüz hiç tweet atılmamış")
        }
    }

    fn show_want_tweets(&self) {
        if self.wants_tweets.len() != 0{
            for want_tweet in &self.wants_tweets{
                want_tweet.full_show();
                println!("-----------------------------")
            }
        }else{
            println!("henüz hiç tweet isteği yok")
        }
    }
    
    fn get_users(&self) {
        println!("All users :");
        for user in self.users.values(){
            println!("{}", user);
        }
    }
}

