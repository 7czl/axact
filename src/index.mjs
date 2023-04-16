import {h, Component, render} from "https://unpkg.com/preact?module";
import htm from "https://unpkg.com/htm?module"


const html = htm.bind(h);
function App(props) {
    return html`
    <div>
        ${props.cpus.map((cpu) => {
            return html`<div class="bar"> ${cpu.toFixed(2)}% usage </div>`;
        })}
    </div>`;
}

let i = 0;
setInterval(async () => {
    let resp = await fetch("/api/cpus");
    if (resp.status !== 200)  {
        throw new Error(`Http error! status ${resp.status}`)
    }
    let json = await resp.json();
    const app = h("pre", null, JSON.stringify(json, null, 2));
    render(html`<${App} cpus=${json} usage></${App}>`, document.body);
}, 1000);
