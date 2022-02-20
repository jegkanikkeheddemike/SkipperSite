import express from "express";
import {login_init} from "./login";
import {chat_init} from "./chat";
import * as db from "./db"

function chat_app(e_app: express.Express) {
    db.db_init();
    
    login_init(e_app);
    chat_init(e_app);

    e_app.get('/projekter/chat_app', (req, res) => {
        res.redirect("/projekter/chat_app/login");
    });
}

export { chat_app };