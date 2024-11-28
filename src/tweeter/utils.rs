use std::any::type_name;
use super::types::*;

pub(crate) fn print_type_of<T>(_: &T) {
    println!("Type is: {}", type_name::<T>());
}

pub(crate) fn cep(message:&str){
    println!("HATA : {}",message);
}

pub(crate) fn mail_analize(mail: &str) -> Result<Email, String> {
    let user_mail_name = mail.split('@').next().ok_or_else(|| {
        "Bu mail ismi kullanılamadı.".to_string()
    })?;

    let mail_type = VALID_DOMAINS
        .iter()
        .find(|(domain, _)| mail.ends_with(domain))
        .map(|(_, mail_type)| mail_type.clone())
        .ok_or_else(|| {
            "Mail adresiniz @gmail.com, @hotmail.com veya @outlook.com olmalı.".to_string()
        })?;

    Email::new(mail_type, user_mail_name.to_string())
        .map_err(|_| "Email oluşturulamadı.".to_string())
}

pub(crate) fn nwt(name:&str)->&str{
    if name.is_empty(){
        return "Name is cannot be empty";
    }
    if name.len() > 12{
        return "Name is cannot be bigger than the 12 charracter";
    }

    if name.len() < 3{
        return "Name is cannot be lesser than the 3 charracter";
    }else{
        return "name is ok";
    }
}

pub(crate) fn mwt(mail: &str) -> &str {
    if mail.trim().is_empty() || !mail.contains('@') {
        return "Email cannot be empty and must include '@'.";
    }

    let mail_type = VALID_DOMAINS
        .iter()
        .find(|(domain, _)| mail.ends_with(domain))
        .map(|(_, mail_type)| mail_type.clone());

    if mail_type.is_none() {
        return "Invalid email domain. Must be @gmail.com, @hotmail.com, or @outlook.com.";
    }

    let name_part = mail.split('@').next().unwrap_or("");
    if name_part.is_empty() || name_part.len() > 16 {
        return "Invalid email format. Name part cannot be empty or longer than 16 characters.";
    }

    "Email is valid."
}


pub fn input(message:&str) -> String{
    println!("{}", message);
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    return input.trim().to_owned();
}

pub(crate) fn write_start_section(){
    println!("\nTweeter a hoşgeldin.");
    println!("Yapmak istediğin işlemi seç:");
    println!("0.Exit");
    println!("1.Sign in");
    println!("2.Sign up");
    println!("Eğer hesabın yoksa ilk önce 2. seçenek yani Sign up'ı seçin.");
}

pub(crate) fn write_main_section(){
    println!("0. Çıkış yap.");
    println!("1. Random tweet.");
    println!("2. Tüm tweetleri görüntüle.");
    println!("3. Tüm tweetleri özet görüntüle");
    println!("4. Tweetlerimi görüntüle.");
}
