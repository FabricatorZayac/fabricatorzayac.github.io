async function setTheme(theme) {
    let link = document.getElementById("theme");
    link.href = `/${theme}.css`;
    localStorage.setItem("theme", theme);
}

async function toggle() {
    if (localStorage.getItem("theme") == "dark") {
        setTheme("light");
    } else {
        setTheme("dark");
    }
}

setTheme(localStorage.getItem("theme"));
