const port = 3000;
import express from "express";
const e_app = express();



//Load /home
import {home} from "./views/home/sub_server";
home(e_app);

//server static files
e_app.use(express.static(__dirname + "/views"));

//When visited on /, redirect to /home
e_app.get('/',(req,res) => {
    res.redirect("/home");
});

e_app.listen(port, () => {
    console.log(`    Server listening at on ${port}`)
})