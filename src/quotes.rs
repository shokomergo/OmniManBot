use rand::seq::SliceRandom;
use rand::thread_rng;

pub const OMNI_MAN_QUOTES: &[&str] = &[
    "Think, Mark!",
    "You want to die for this planet? Fine. What’s 17 more years? I can always start again… make another kid.",
    "I do love your mother. But she’s more like a… a pet to me.",
    r#"This isn’t your world. It’s theirs. But we can help them. We can stop wars. Eliminate hunger. Give them medical technology centuries ahead of what they have now.
    We’ve already been doing it. If it wasn’t for you and me, this planet would be in flames."#,
    r#"Why did you make me do this? You’re fighting so you can watch everyone around you die! Think, Mark! You’ll outlast every fragile, insignificant being on this planet. 
    You’ll live to see this world crumble to dust and blow away! Everyone and everything you know will be gone!"#,
    "I will burn this planet down. Before I spend another Minute living among these animals!",
    "Are you sure?",
    "You're fighting like you're on earth. This is different.",
    "I Think I Miss My Wife"
];

pub fn omni_man_reply(user_msg: &str) -> String {
    let msg = user_msg.to_lowercase();

    let mut best_score = 0;
    let mut best_quotes: Vec<&str> = Vec::new();

    for quote in OMNI_MAN_QUOTES {
        let q = quote.to_lowercase();
        let mut score = 0;

        if msg.contains("world") || msg.contains("planet") {
            if q.contains("world") || q.contains("planet") {
                score += 2;
            }
        }

        if msg.contains("fight") || msg.contains("die") {
            if q.contains("die") || q.contains("fighting") {
                score += 2;
            }
        }

        if msg.contains("mother") || msg.contains("family") {
            if q.contains("mother") {
                score += 3;
            }
        }

        if msg.contains("sure") || msg.contains("yes") || msg.contains("no") {
            if q.contains("sure") {
                score += 2;
            }
        }

        if msg.contains("wife") || msg.contains("miss") {
            if q.contains("wife") {
                score += 3;
            }
        }

        if msg.contains("think") {
            if q.contains("think") {
                score += 4;
            }
        }

        if score > 0 {
            if score > best_score {
                best_score = score;
                best_quotes.clear();
                best_quotes.push(*quote);
            }else if score == best_score {
                best_quotes.push(*quote);
            }
        }
    }

    let mut rng = thread_rng();

    if !best_quotes.is_empty() {
        best_quotes.choose(&mut rng).unwrap().to_string()
    } else {
        OMNI_MAN_QUOTES.choose(&mut rng).unwrap().to_string()
    }

}
    