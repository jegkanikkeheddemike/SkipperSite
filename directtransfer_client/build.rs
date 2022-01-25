use std::fs;


fn main(){
    println!("Hello from build!");


    let style = inline_style(include_str!("src\\html\\style.css"));
    let scripts = inline_script(include_str!("src\\html\\scripts.js"));
    let vue_scripts = inline_script(include_str!("src\\html\\vue_scripts.js"));
    let home_page = include_str!("src\\html\\Homepage.html");

    let home_page = home_page.replace("{style}", &style);
    let home_page = home_page.replace("{scripts}", &scripts);
    let home_page = home_page.replace("{vue_scripts}", &vue_scripts);

    fs::write("src\\html\\final_home.html",home_page).unwrap();
}



fn inline_style(s: &str) -> String {
    format!(r#"<style type="text/css">{}</style>"#, s)
}

fn inline_script(s: &str) -> String {
    format!(r#"<script type="text/javascript">{}</script>"#, s)
}
