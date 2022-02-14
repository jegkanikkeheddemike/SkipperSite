var app = new Vue({
    el: "#app",
    data: {
        time_live: "loading"
    }
});

$.getJSON("/home/time_live", (data) => {
    app.time_live = data.time_live;
});

setInterval(async () => {
    app.time_live = app.time_live + 1;
}, 1000);