use super::{traits::*,types::*,utils::*};
use std::{collections::HashMap, fmt::{Display, Formatter}};
use rand::Rng;

// Basic impls ---------------------------------------
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

impl Mail {
    pub fn domain(&self) -> &str {
        match self {
            Mail::Gmail => "gmail.com",
            Mail::Hotmail => "hotmail.com",
            Mail::Outlook => "outlook.com",
        }
    }
}

impl Status {
    pub fn stat(&self) -> &str {
        match self {
            Status::Oke => "iyi durumda",
            Status::Editing => "düzenlenmeli",
            Status::Edited =>"düzenlendi",
            Status::Not => "silinmeli",
        }
    }

    pub fn str_to_stat(value:&str)->Status{
        match value {
            "Oke"=> Status::Oke,
            "Edited"=> Status::Edited,
            "Editing"=> Status::Editing,
            "Not"=> Status::Not,
            _ => Status::Not,
        }
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

// Basic impls ---------------------------------------

// Normal impls ---------------------------------------
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
            return Err("Tweet 220 karakterden uzun olamaz.".to_string());
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

    pub fn update_id(&mut self, new_id: u8) {
        self.id = new_id;
    }

    pub fn update_author(&mut self, new_author: String) {
        self.author = new_author;
    }

    pub fn update_title(&mut self, new_title:String) {
        self.title = new_title;
    }

    pub fn update_description(&mut self, new_desc: String) {
        self.description = new_desc;
    }

    pub fn update_tweet(&mut self, new_tweet: String) {
        self.tweet = new_tweet;
    }

    pub fn update_status(&mut self, new_status:Status) {
        self.status = new_status;
    }
    
}

impl TweetT for Tweet {

    fn full_show(&self){
        println!("id :{}",self.id);
        println!("Author: {}",self.author);
        println!("Title :{}",self.title);
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
    pub fn new(id:u8,name: String, email_adress: Email, adminlevel: AdminLevel) -> Self {
        Self {
            id,
            name,
            email_adress,
            adminlevel,
            tweets: Vec::new(),
            want_be_mod: false,
        }
    }

    pub fn full_show(&self){
        println!("id : {}",self.id);
        println!("name:{}",self.name);
        println!("email adress:{}",self.email_adress.mail_address());
        println!("admin level :{}",self.adminlevel.level_value());
        println!("want be mod :{}",self.want_be_mod);
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

    // pub fn login_check(&self, user: &User) -> String {
    //     if let Some((_, existing_user)) = self.users.iter().find(|(_, u)| {
    //         u.name == user.name && u.email_adress.mail_address() == user.email_adress.mail_address()
    //     }) {
    //         format!("Giriş onaylandı: Hoş geldiniz, {}!", existing_user.name)
    //     } else {
    //         "Giriş başarısız: Kullanıcı adı veya e-posta hatalı.".to_string()
    //     }
    // }
    
    pub fn add_new_tweet(&mut self, tweet: &mut Tweet) -> &str {
        if tweet.status != Status::Edited && tweet.status != Status::Oke {
            return "Bu tweet düzenlenmeli.";
        }
        tweet.status = Status::Oke;
        self.wants_tweets.retain(|t| t.id != tweet.id);
        self.tweets.push(tweet.clone());
        "Tweet başarıyla eklendi."
    }    
    
    pub fn accept_wtweets(&mut self, user: &User, id: u8) -> &str {
        if self.users.iter().any(|us|us.0 != &user.id){
            "kullanıcı veritabanında bulunamadı";
        }

        if user.adminlevel != AdminLevel::Possibleator && user.adminlevel != AdminLevel::Moderator {
            return "Bunun için yetkiniz yok.";
        }
        
        if let Some(tweet_request) = self.wants_tweets.iter().find(|wt| wt.id == id) {
            let mut updated_tweet = tweet_request.clone();
            updated_tweet.status = Status::Oke;
            
            self.add_new_tweet(&mut updated_tweet);
            "Tweet isteği kabul edildi."
        } else {
            " id'ye sahip tweet bulunamadı."
        }
    }

    pub fn edit_tweet(
        &mut self,
        user: &User,
        which: EditableTweetSection,
        tweet: &mut Tweet,
    ) -> &str {
        if !self
            .users
            .iter()
            .any(|(email, _)| email == &user.id)
        {
            return "Kullanıcı veritabanında bulunamadı.";
        }
    
        if user.adminlevel != AdminLevel::Regulator && user.adminlevel != AdminLevel::Moderator {
            return "Bunun için yetkiniz yok.";
        }
    
        match which {
            EditableTweetSection::Tweet(value) => {
                if value.len() > 220 {
                    return "Tweet mesajı 220 karakterden uzun olamaz.";
                }
                tweet.update_tweet(value);
            }
            EditableTweetSection::Title(value) => {
                if value.len() > 10 {
                    return "Tweet başlığı 10 karakterden uzun olamaz.";
                }
                tweet.update_title(value);
            }
            EditableTweetSection::Description(value) => {
                if value.len() > 25 {
                    return "Açıklama 25 karakterden uzun olamaz.";
                }
                tweet.update_description(value);
            }
            EditableTweetSection::Id(value) => {
                if self.wants_tweets.iter().any(|t| t.id == value) {
                    return "Bu ID zaten başka bir tweete ait.";
                }
                tweet.update_id(value);
            }
            EditableTweetSection::Author(value) => {
                if value.len() > 50 {
                    return "Yazar adı 50 karakterden uzun olamaz.";
                }
                tweet.update_author(value);
            }
            EditableTweetSection::Status(value) => {
                tweet.update_status(value);
                println!("Tweet durumu güncellendi: {}", tweet.status.stat());
            }
        }

        tweet.update_status(Status::Edited);
    
        let index = self.wants_tweets.iter().position(|t| t.id == tweet.id);
        if let Some(idx) = index {
            self.wants_tweets[idx] = tweet.clone(); 
        } else {
            self.wants_tweets.push(tweet.clone()); 
        }
        
        "Tweet başarıyla düzenlendi."
    }
    
    pub fn add_want_tweets(&mut self,tweet:&Tweet)->&str{
        self.wants_tweets.push(tweet.clone());
        return "tweet eklendi";
    }

    pub fn add_user(&mut self, user: &User) -> &str {
        if let Some(_existing_user) = self.users.get(&user.id) {
            return "Bu e-posta zaten bir kullanıcı tarafından kullanılıyor.";
        }

        if user.adminlevel == AdminLevel::Moderator {
            if self.users.values().any(|u| u.adminlevel == AdminLevel::Moderator) {
                return "Zaten bir moderatör var. Başka bir moderatör eklenemez.";
            }
        }

        self.users.insert(user.id, user.clone());
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

    pub fn update_user_mod(&mut self, user: &mut User, moderator: &User) -> &str {
        if !user.want_be_mod {
            return "Kullanıcı böyle bir istekte bulunmamış.";
        }
        if moderator.adminlevel != AdminLevel::Moderator {
            return "Moderatörün kullanıcıya yetki verme yetkisi yok.";
        }
        if user.adminlevel == AdminLevel::Moderator {
            return "Kullanıcının yetkisi zaten moderatör.";
        }
        match user.adminlevel {
            AdminLevel::Customer => {
                user.adminlevel = AdminLevel::Author;
                user.want_be_mod = false;
            }
            AdminLevel::Author => {
                user.adminlevel = AdminLevel::Regulator;
                user.want_be_mod = false;
            }
            AdminLevel::Regulator => {
                user.adminlevel = AdminLevel::Possibleator;
                user.want_be_mod = false;
            }
            AdminLevel::Possibleator => {
                user.adminlevel = AdminLevel::Moderator;
                user.want_be_mod = false;
            }
            _ => return "Kullanıcıya yetki verilemedi.",
        }
        if let Some(user_entry) = self.users.get_mut(&user.id) {
            *user_entry = user.clone();
        }
    
        "Kullanıcının leveli arttırıldı."
    }
    

    
}

impl UserT for User {
    fn read_all_your_tweet(&self, t: &Tweeter) {
        let main_section_tweets: Vec<_> = t
            .tweets
            .iter()
            .filter(|tweet| tweet.author == self.name ) 
            .collect();

        let non_main_section_tweets: Vec<_> = t
            .wants_tweets
            .iter()
            .filter(|tweet| tweet.author == self.name ) 
            .collect();

        if main_section_tweets.is_empty() && non_main_section_tweets.is_empty() {
            println!("\nHiçbir tweetiniz bulunmuyor.");
            return;
        }

        println!("\nAna bölümde bulunan tweetleriniz:");
        if main_section_tweets.is_empty() {
            println!("Hiçbir tweet ana bölümde bulunmuyor.");
        } else {
            for tweet in main_section_tweets {
                tweet.customer_show();
            }
        }

        println!("-------------------");

        println!("\nAna bölümde bulunmayan tweetleriniz:");
        if non_main_section_tweets.is_empty() {
            println!("Hiçbir tweet ana bölümde bulunmuyor.");
        } else {
            for tweet in non_main_section_tweets {
                tweet.customer_show();
            }
        }
    }
    

    fn write_tweet(&mut self, t: &mut Tweeter,title:String,description:String,tweet_message:String) -> String {
        if self.adminlevel.level_value() < 1 {
            return "Levelin tweet atmaya yetmiyor. Tweet atmak istiyorsan başvur.".to_string();
        }

        if title.len() <= 0 || description.len() <= 0 || tweet_message.len() <4 {
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
    fn get_xid_wants_tweet(arr:&mut Vec<Tweet>, id: u8) -> Option<&mut Tweet> {
        for i in arr{
            if i.id == id {
                return Some(i);
            }
        }None
    }

    fn get_xid_user(users: &mut HashMap<u8, User>, id: u8) -> Option<&mut User> {
        users.get_mut(&id) 
    }
    

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

    fn want_mod_users(&self) {
        let userss: Vec<_> = self.users.iter().filter(|user| user.1.want_be_mod).collect();
        if userss.is_empty() {
            println!("Hiç mod isteyen yok.");
        } else {
            for user in &userss {
                user.1.full_show();
            }
        }
    }
    
}

