function println(print) {
    document.getElementById("Console").innerHTML = document.getElementById("Console").innerHTML +"<li>" + print + "</li>";
    document.getElementById("Console").lastElementChild.scrollIntoView();
}