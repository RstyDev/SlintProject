const { invoke } = window.__TAURI__.tauri;

let buscadorInput;
let greetMsgEl;
let tpProd;
let mark;
let variety;
let amount;
let pres;
let cod;
let precio_de_venta;
let precio_de_costo;
let percent;

function changeSaleprice() {
  let percent = document.querySelector('#porcentaje').value;
  console.log(percent);
  let sale = document.querySelector('#precio_de_compra');
  document.querySelector('#precio_de_venta').value = parseFloat(sale).value * parseFloat(percent)
}

async function buscador() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  greetMsgEl.textContent = await invoke("buscador", { name: buscadorInput.value });
}

async function agregarProducto(){
  greetMsgEl.textContent = ("Producto agregado: "+await invoke("agregar", { codigo: cod.value, precio_de_venta: precio_de_venta.value,porcentaje:percent.value,precio_de_costo:precio_de_costo.value, tipoProducto: tpProd.value,marca: mark.value, variedad:variety.value, cantidad:amount.value, presentacion:pres.value }));
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
  var element=document.querySelector('#precio_de_costo');
  element.addEventListener('input', changeSaleprice());
});



window.addEventListener("DOMContentLoaded", ()=>{
  tpProd = document.querySelector('#tipo_producto');
  mark = document.querySelector('#marca');
  variety = document.querySelector('#variedad');
  amount = document.querySelector('#cantidad');
  pres = document.querySelector('#presentacion');
  cod=document.querySelector('#codigo_de_barras');
  precio_de_venta=document.querySelector('#precio_de_venta');
  percent=document.querySelector('#porcentaje');
  precio_de_costo = document.querySelector('#precio_de_costo');
  document.querySelector('#nuevo-producto-form').addEventListener("submit", (e) => {
    e.preventDefault();
    agregarProducto();
  })
})


