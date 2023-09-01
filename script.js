async function setTheme(theme) {
    if (theme == "dark") {
        document.body.className = "dark";
        localStorage.setItem("theme", "dark");
    } else {
        document.body.className = "";
        localStorage.setItem("theme", "light");
    }
}

async function toggle() {
    if (localStorage.getItem("theme") == "dark") {
        setTheme("light");
    } else {
        setTheme("dark");
    }
}

setTheme(localStorage.getItem("theme"));
