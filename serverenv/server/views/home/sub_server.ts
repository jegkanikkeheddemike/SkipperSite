import express from "express";

var live_time = new Date();

function home(e_app: express.Express) {
    e_app.get('/home/time_live', (req, res) => {

        let now = new Date();
        let up_time = Math.floor((now.getTime() - live_time.getTime()) / 1000);

        res.send({ time_live: up_time });
    });
}

export { home };