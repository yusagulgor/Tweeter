
use super::{types::*,utils::*,traits::*,dependencies::*};

fn get_user_choice() -> Option<u8> {
    write_start_section();
    let choice = input("Seçmek istediğin işlem numarasını giriniz:");

    match choice.trim().parse::<u8>() {
        Ok(valid_choice) if valid_choice <= 2 => Some(valid_choice),
        _ => {
            println!("Hatalı giriş yaptınız. Lütfen 0, 1 veya 2 giriniz!\n");
            None 
        }
    }
}


//  ? _______FUNCTIONS_______

fn send_tweet(t: &mut Tweeter,user: &mut User){
    println!("Tweet atma işlemi :");
    let title = input("lütfen tweetinizin başlığını yazın(Uzun tutmayın):");
    let description = input("lütfen tweetinizin kısa bir açıklamasını yazın(Uzun tutmayın):");
    let tweet_message = input("lütfen tweetinizin metninini yazın:");

    let tweet =user.write_tweet(t, title, description, tweet_message);
    if tweet == "Tweet başarıyla oluşturuldu ve onay bekleyen tweetler listesine eklendi."{
        println!("{}",tweet);
    }else{println!("Tweet atarken bir sorun oluştu. Hata:{}",tweet);}
}

fn tweet_requests(t: &mut Tweeter, user: &mut User) {
    loop {
        println!("İşte gelen tweet istekleri:");
        t.show_want_tweets();
        println!("Seçmek istediğin tweetin ID'sini yaz. Eğer çıkış yapmak istiyorsan 0 yaz.");
        
        let id_input = input("Seçilen tweetin ID'si: ");
        
        if id_input.trim() == "0" {
            println!("İşlemden çıkılıyor...");
            break;
        }

        match id_input.trim().parse::<u8>() {
            Ok(valid_id) => {
                // Tweet'i işleme
                let result = t.accept_wtweets(user, valid_id);
                println!("{}", result);

                if result == "Tweet isteği kabul edildi." || result == "Bunun için yetkiniz yok." {
                    break;
                }
            }
            Err(_) => {
                println!("Hatalı giriş yaptınız. Lütfen geçerli bir ID girin!");
            }
        }
    }
}

fn process_choice(choice: u8, t: &mut Tweeter, user: &mut User) -> bool {
    match choice {
        0 => {
            println!("\nÇıkış yapılıyor. Görüşmek üzere!");
            true
        }
        1 => {
            println!("\nRandom tweet:");
            t.random_tweet();
            false
        }
        2 => {
            println!("\nTüm tweetler:");
            t.show_tweets_c();
            false
        }
        3 => {
            println!("\nTüm tweetler özet görüntüle:");
            t.short_show_tweets();
            false
        }
        4 => {
            if user.adminlevel.level_value() < 1 {
                println!("\nHenüz hiç tweet atmadınız. Tweet atmak için Author yetkisi lazım.");
            } else {
                println!("\nAttığınız tweetler:");
                user.read_all_your_tweet(t);
            }
            false
        }
        5 => {
            if user.adminlevel.level_value() > 0 {
                send_tweet(t, user);
            }else if user.adminlevel.level_value() == 0{
                println!("Author olmak için başvuruldu.");
            } else {
                println!("Author olmak için başvurmalısın.");
            }
            false
        }
        6 if user.adminlevel.level_value() >= 2 => {
            println!("\nTweet düzenleme:");
            // t.edit_tweets();
            false
        }
        7 if user.adminlevel.level_value() >= 3 => {
            println!("\nGelen tweet isteklerini yönet:");
            tweet_requests(t, user);
            false
        }
        8 if user.adminlevel.level_value() >= 4 => {
            println!("\nGelen mod isteklerini yönet:");
            // t.handle_mod_requests();
            false
        }
        9 if user.adminlevel.level_value() >= 4 => {
            println!("\nTüm tweetlerin tam gösterimi:");
            t.show_tweets_a();
            false
        }
        10 if user.adminlevel.level_value() >= 4 => {
            println!("\nKullanıcıları görüntüle:");
            t.get_users();
            false
        }
        _ => {
            println!("\nGeçersiz seçim, lütfen tekrar deneyin.");
            false
        }
    }
}

fn main_widget(t: &mut Tweeter, user: &mut User) {
    let menu = match user.adminlevel {
        AdminLevel::Customer => vec!["5. Author olmak için başvur."],
        AdminLevel::Author => vec![
            "5. Yeni tweet at (NOT: istekler kabul edildikten sonra görüntülenebilir).",
            "6. Possibleator olmak için başvur.",
        ],
        AdminLevel::Regulator => vec![
            "5. Yeni tweet at (NOT: istekler kabul edildikten sonra görüntülenebilir).",
            "6. Tweetleri düzenle.",
            "7. Moderator olmak için başvur.",
        ],
        AdminLevel::Possibleator => vec![
            "5. Yeni tweet at (NOT: istekler kabul edildikten sonra görüntülenebilir).",
            "6. Gelen tweet isteklerine izin ver.",
            "7. Regulator olmak için başvur.",
        ],
        AdminLevel::Moderator => vec![
            "5. Tweet at.",
            "6. Tweetleri düzenle.",
            "7. Gelen tweet istekleri.",
            "8. Gelen mod istekleri.",
            "9. Full show tweet.",
            "10. Kullanıcıları görüntüle.",
        ],
        _ => vec![
            "0. Çıkış yap.",
            "1. Random tweet.",
            "2. Tüm tweetleri görüntüle.",
            "3. Tüm tweetleri özet görüntüle.",
        ],
    };

    loop {
        println!(
            "\n{} ana ekranına hoş geldiniz, {}!",
            user.adminlevel.level_name(),
            user.name.to_uppercase()
        );
        println!("İşte yapabileceğiniz seçenekler:");
        write_main_section();

        for (i, option) in menu.iter().enumerate() {
            println!("{}", option);
        }

        let choice = input("Seçmek istediğiniz işlem numarasını giriniz:");
        let choice = choice.parse::<u8>().unwrap_or(255);

        if process_choice(choice, t, user) {
            break;
        }
    }
}

// ! Kullanıcı girişi (sin) işlemi
fn sin(t: &mut Tweeter) {
    let name = input("\nWhat is your user name:");

    if nwt(&name) != "name is ok" {
        cep("İsmin geçerli değil");
        goto_widget(t);
        return;
    }

    let mail = input("What is your email address (Not: with @ and domain. Ex: example@example.com):");

    let analyzed_mail = match mail_analize(&mail) {
        Ok(mail) => mail,
        Err(err) => {
            println!("HATA: Mail analiz edilemedi. Sebep: {}", err);
            goto_widget(t);
            return;
        }
    };

    let mut twe = t.clone();

    if let Some((user_email, mut user)) = twe.users.iter_mut().find(|(user_email, user)| {
        user.name == name.trim() && user.email_adress == analyzed_mail
    }) {
        println!("Kullanıcı başarıyla giriş yaptı\n");
        main_widget(t, &mut user); 
    } else {
        println!("Bu bilgilerle kayıtlı kişi bulunamadı\n");
    }
}


// ! SUP

pub fn sup(t: &mut Tweeter) {
    let name = input("\nWhat do you want as a username:");

    if nwt(&name) != "name is ok" {
        cep("Bu kullanıcı adı kullanılamaz.");
        goto_widget(t);
        return;
    }

    let mail = input("What do you want as an email (NOT: must include '@' and domain. Ex: example@example.com):");

    if mwt(&mail) != "Email is valid." {
        cep("Bu mail adresi kullanılamaz.");
        goto_widget(t);
        return;
    }

    if t.users.iter().any(|user| user.1.name == name && user.1.email_adress.mail_address() == mail) {
        println!("Bu isim ve e-posta adresiyle zaten kayıtlı bir kullanıcı var.");
        goto_widget(t);
        return;
    }

    let analyzed_mail = match mail_analize(&mail) {
        Ok(mail) => mail,
        Err(err) => {
            println!("HATA: Mail analiz edilemedi. Sebep: {}", err);
            goto_widget(t);
            return;
        }
    };

    if name == ADMIN_NAME && mail == ADMIN_MAIL {
        let new_admin = User::new(name.trim().to_string(), analyzed_mail, AdminLevel::Moderator);

        let cek = t.add_user(&new_admin);
        if cek == "Kullanıcı başarıyla eklendi." {
            println!("\n{}", cek);
            println!("Hoş geldin admin: {}\n", new_admin);
        } else {
            println!("HATA: Admin eklenemedi. Sebep: {}", cek);
        }
    } else {
        let new_user = User::new(name.trim().to_string(), analyzed_mail, AdminLevel::Customer);

        let cek = t.add_user(&new_user);
        if cek == "Kullanıcı başarıyla eklendi." {
            println!("\n{}", cek);
            println!("Hoş geldin: {}\n", new_user);
        } else {
            println!("HATA: Kullanıcı eklenemedi. Sebep: {}", cek);
        }
    }
}


fn goto_widget(t: &mut Tweeter) {
    loop {
        if let Some(choice) = get_user_choice() {
            match choice {
                0 => {
                    println!("Çıkış yapılıyor. Tekrar görüşmek üzere. :D");
                    break; 
                }
                1 => sin(t),
                2 => sup(t), 
                _ => println!("Geçersiz seçim. Lütfen tekrar deneyin."), 
            }
        } else {
            println!("Geçersiz giriş yaptınız. Lütfen 0, 1 veya 2 giriniz!\n");
        }
    }
}

fn start() {
    let mut my_tweeter = Tweeter::new();
    goto_widget(&mut my_tweeter);
}

pub fn tweer_run() {
    start();
}
