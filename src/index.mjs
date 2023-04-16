import {h, Component, render} from "https://unpkg.com/preact?module";

document.addEventListener("DOMContentLoaded", () => {
    let i = 0;
    setInterval(async () => {
        let resp = await fetch("/api/cpus");
        if (resp.status !== 200)  {
            throw new Error(`Http error! status ${resp.status}`)
        }
        let json = await resp.json();
        const app = h("pre", null, JSON.stringify(json, null, 2));
        render(app, document.body);
    }, 1000);
});