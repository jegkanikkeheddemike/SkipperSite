var app = new Vue({
    el: "#app",
    data: {
        time_live: "loading",
        on_load_server_time: 0,
        on_load_client_time: 0
    }
});

$.getJSON("/home/time_live", (data) => {
    app.time_live = data.time_live;
    app.on_load_server_time = data.time_live;
    app.on_load_client_time = new Date().getTime();
});

setInterval(async () => {
    app.time_live = app.on_load_server_time + Math.floor((new Date().getTime() - app.on_load_client_time)/1000);
}, 1000);