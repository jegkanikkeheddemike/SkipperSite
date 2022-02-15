import express from "express";
import {chat_login} from "./login";
import * as db from "./db"

function chat_app(e_app: express.Express) {
    db.db_init();
    
    chat_login(e_app);

    e_app.get('/projekter/chat_app', (req, res) => {
        res.redirect("/projekter/chat_app/login");
    });
}

export { chat_app };