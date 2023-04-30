// Github : Elazrod56

use serde_json::Value;
use std::fs;
use std::io;

fn main() -> io::Result<()> {
    // The main function must return a Result<()> if we want to use the '?' operator on lines 13 and 14

    println!("-------- THE TIKTOK JSON ANALYZER --------");
    println!("This program calculates some statistics using your TikTok JSON data export");
    println!("Please make sure the JSON file is located in json/user_data.json \n");

    let file = fs::read_to_string("json/user_data.json")?;
    let data = serde_json::from_str::<Value>(file.as_str())?;

    let username = &data["Profile"]["Profile Information"]["ProfileMap"]["userName"];
    println!("File detected \u{2705}
        \nThe data of {username} will be analyzed... \n");

    // Amount of logins
    println!("\n-------- LOGINS \u{2386} --------\n");
    let login_history = &data["Activity"]["Login History"]["LoginHistoryList"];
    let login_history_len = login_history
        .as_array()
        .map(|array| array.len())
        .unwrap_or(0);

    println!("In the last 6 months, you connected {login_history_len} times to Tiktok");
    println!(
        "You launched TikTok {} times a day on average\n",
        login_history_len / 183
    );
    // 6 months corresponds to roughly 183 days, this is why we divide by 183

    // Amount of videos watched
    let watched_videos = &data["Activity"]["Video Browsing History"]["VideoList"];
    let watched_videos_len = watched_videos
        .as_array()
        .map(|array| array.len())
        .unwrap_or(0);

    println!("In the last 6 months, you have watched {watched_videos_len} videos");
    println!(
        "You watched around {} videos a day on average\n",
        watched_videos_len / 183
    );

    // Favorites
    println!("\n-------- FAVORITE SOUNDS & VIDEOS \u{1F4FA} --------\n");
    let favorite_sounds = &data["Activity"]["Favorite Sounds"]["FavoriteSoundList"];
    let favorite_sounds_len = favorite_sounds
        .as_array()
        .map(|array| array.len())
        .unwrap_or(0);

    println!("You have {favorite_sounds_len} favorite sounds\n");

    let favorite_videos = &data["Activity"]["Favorite Videos"]["FavoriteVideoList"];
    let favorite_videos_len = favorite_videos
        .as_array()
        .map(|array| array.len())
        .unwrap_or(0);

    println!("You have {favorite_videos_len} favorite videos\n");

    // Likes
    println!("\n-------- LIKES \u{2764} --------\n");
    let liked_videos = &data["Activity"]["Like List"]["ItemFavoriteList"];
    let liked_videos_len = liked_videos
        .as_array()
        .map(|array| array.len())
        .unwrap_or(0);

    let date_of_8000th_liked_vid = &liked_videos[liked_videos_len - 1]["Date"];
    // TikTok only keeps track of your latest 8000 likes. (at least in the data export)
    println!("You liked 8000 videos since {date_of_8000th_liked_vid}\n");

    // Comments
    println!("\n-------- COMMENTS \u{1F4AC} --------\n");
    let comments = &data["Comment"]["Comments"]["CommentsList"];
    let comments_len = comments
        .as_array()
        .map(|array| array.len())
        .unwrap_or(0);

    println!("You published {comments_len} comments\n");

    // Videos & likes
    println!("\n-------- YOUR ACCOUNT'S STATS \u{1F464} --------\n");
    let videos = &data["Video"]["Videos"]["VideoList"];
    let videos_len = videos
        .as_array()
        .map(|array| array.len())
        .unwrap_or(0);

    let likes = &data["Profile"]["Profile Information"]["ProfileMap"]["likesReceived"];
    println!("You received {likes} likes and you posted {videos_len} videos");

    Ok(())
}