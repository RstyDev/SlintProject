const { invoke } = window.__TAURI__.tauri;

let buscadorInput;
let greetMsgEl;


async function buscador() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  greetMsgEl.textContent = await invoke("buscador", { name: buscadorInput.value });
}

async function agregarProducto(tpProd,mark,variety,amount,pres,cod,precio){
  greetMsgEl.textContent = await invoke("agregar", { codigo: tipoProducto: tpProd.value,marca: mark.value, variedad:variety.value, cantidad:amount.value, presentacion:pres.value });
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
  let tpProd = document.querySelector('#tipo_producto');
  let mark = document.querySelector('#marca');
  let variety = document.querySelector('#variedad');
  let amount = document.querySelector('#cantidad');
  let pres = document.querySelector('#presentacion');
  let cod=document.querySelector('#codigo_de_barras');
  let precio=document.querySelector('#precio')
  document.querySelector('#nuevo-producto-form').addEventListener("submit", (e) => {
    e.preventDefault();
    agregarProducto();
  })
})


