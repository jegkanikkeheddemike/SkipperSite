import express from "express";
import * as db from "../db"

function login_init(e_app: express.Express) {
    console.log("Login init");

    e_app.post('/projekter/chat_app/login', (req, res) => {

        let cred = {
            username: req.body.username,
            password: req.body.password
        };

        for (let i = 0; i < db.data.users.length; i++) {
            let user = db.data.users[i];
            if (user.client.public.username != cred.username) {
                continue;
            }

            let hashed_pass = db.hash(cred.password, user.secret.salt);
            if (user.secret.password == hashed_pass) {

                res.send({
                    success: true,
                    data: user.client,
                });
                return;
            }
        }

        res.send({
            success: false,
            data: "Wrong username or password"
        });
    });

    e_app.post("/projekter/chat_app/login/signup", (req, res) => {
        let cred = {
            username: req.body.username,
            password: req.body.password,
            c_password: req.body.c_password
        };

        if (cred.username.length < 5) {
            res.send({
                success: false,
                data: "Username too short. Username must be longer than 5 characters"
            });
            return;
        }

        if (cred.password != cred.c_password) {
            res.send({
                success: false,
                data: "Passwords dont match"
            })
            return;
        }
        if (cred.password.length < 5) {
            res.send({
                success: false,
                data: "Password too short. Password must be longer than 5 characters"
            });
            return;
        }

        //if crediatials are fine, check if username already exists
        for (let i = 0; i < db.data.users.length; i++) {
            if (db.data.users[i].client.public.username == cred.username) {
                res.send({
                    success: false,
                    data: "A user with this username already exists"
                });
                return;
            }
        }

        //If alls good, then create the user;
        let userid = db.create_user(cred.username,cred.password);
        let client_data = db.data.users[userid].client;

        res.send({
            success: true,
            data: client_data
        });
    });
}

export { login_init };