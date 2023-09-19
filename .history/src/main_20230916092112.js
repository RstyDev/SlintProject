const { invoke } = window.__TAURI__.tauri;

let buscadorInput;
let greetMsgEl;
let tpProd;
let mark;
let variety;
let amount;
let pres;

async function buscador() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  greetMsgEl.textContent = await invoke("buscador", { name: buscadorInput.value });
}

async function agregarProducto(){
  console.log("llamando a agregar producto")
  greetMsgEl.textContent = await invoke("agregar", { tipo_producto: tpProd.value,marca: mark.value, variedad:variety.value, cantidad:amount.value, presentacion:pres.value });
}

window.addEventListener("DOMContentLoaded", () => {
  buscadorInput = document.querySelector("#buscador");
  greetMsgEl = document.querySelector("#greet-msg");
  document.querySelector("#buscador-form").addEventListener("submit", (e) => {
    e.preventDefault();
    buscador();
  });
  tpProd = document.querySelector('#tipo_producto');
  mark = document.querySelector('#marca');
  variety = document.querySelector('#variedad');
  amount = document.querySelector('#cantidad');
  pres = document.querySelector('#presentacion');
  document.querySelector('#nuevo-producto-form').addEventListener("submit", (e) => {
    e.preventDefault();
    agregarProducto();
  })
});


