const { invoke } = window.__TAURI__.tauri;

let buscadorInput;
let greetMsgEl;
let tpProd;
let mark;
let variety;
let amount;
let pres;
let cod;
let precio;

async function buscador() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  greetMsgEl.textContent = await invoke("buscador", { name: buscadorInput.value });
}

async function agregarProducto(){
  greetMsgEl.textContent = (await invoke("agregar", { codigo: cod.value, precio: precio.value, tipoProducto: tpProd.value,marca: mark.value, variedad:variety.value, cantidad:amount.value, presentacion:pres.value }));
}

window.addEventListener("DOMContentLoaded", () => {
  buscadorInput = document.querySelector("#buscador");
  greetMsgEl = document.querySelector("#greet-msg");
  document.querySelector("#buscador-form").addEventListener("submit", (e) => {
    e.preventDefault();
    buscador();
  });
});
window.addEventListener("DOMContentLoaded", ()=>{
  tpProd = document.querySelector('#tipo_producto');
  mark = document.querySelector('#marca');
  variety = document.querySelector('#variedad');
  amount = document.querySelector('#cantidad');
  pres = document.querySelector('#presentacion');
  cod=document.querySelector('#codigo_de_barras');
  precio=document.querySelector('#precio')
  document.querySelector('#nuevo-producto-form').addEventListener("submit", (e) => {
    e.preventDefault();
    agregarProducto();
  })
})


