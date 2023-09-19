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


async function buscador() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  greetMsgEl.textContent = await invoke("buscador", { name: buscadorInput.value });
}

async function agregarProducto() {
  greetMsgEl.textContent = ("Producto agregado: " + await invoke("agregar", { codigo: cod.value, precio_de_venta: precio_de_venta.value, porcentaje: percent.value, precio_de_costo: precio_de_costo.value, tipoProducto: tpProd.value, marca: mark.value, variedad: variety.value, cantidad: amount.value, presentacion: pres.value }));
}

window.addEventListener("DOMContentLoaded", () => {
  buscadorInput = document.querySelector("#buscador");
  greetMsgEl = document.querySelector("#greet-msg");
  document.querySelector("#buscador-form").addEventListener("submit", (e) => {
    e.preventDefault();
    buscador();
  });
});

window.addEventListener("DOMContentLoaded", () => {
  document.querySelector('#precio_de_costo').addEventListener('input', () => {
    let percent = document.querySelector('#porcentaje').value;
    let sale = document.querySelector('#precio_de_costo').value;
    document.querySelector('#precio_de_venta').value = parseFloat(sale) * (1 + (parseFloat(percent)) / 100)
  });
});

window.addEventListener("DOMContentLoaded", () => {
  document.querySelector('#porcentaje').addEventListener('input', () => {
    let percent = document.querySelector('#porcentaje').value;
    let sale = document.querySelector('#precio_de_costo').value;
    if (sale != null) {
      document.querySelector('#precio_de_venta').value = parseFloat(sale) * (1 + (parseFloat(percent)) / 100)
    }
  });
});

window.addEventListener("DOMContentLoaded", () => {
  let venta = document.querySelector('#precio_de_venta');
  venta.addEventListener('input', () => {
    let percent = document.querySelector('#porcentaje').value;
    console.log(percent);
    let sale = document.querySelector('#precio_de_costo').value;
    console.log(sale);
    if (sale != null) {
      percent=((parseFloatventa.value/sale)*100)-100;
    
    }
  });
});



window.addEventListener("DOMContentLoaded", () => {
  tpProd = document.querySelector('#tipo_producto');
  mark = document.querySelector('#marca');
  variety = document.querySelector('#variedad');
  amount = document.querySelector('#cantidad');
  pres = document.querySelector('#presentacion');
  cod = document.querySelector('#codigo_de_barras');
  precio_de_venta = document.querySelector('#precio_de_venta');
  percent = document.querySelector('#porcentaje');
  precio_de_costo = document.querySelector('#precio_de_costo');
  document.querySelector('#nuevo-producto-form').addEventListener("submit", (e) => {
    e.preventDefault();
    agregarProducto();
  })
})


