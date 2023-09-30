const { invoke } = window.__TAURI__.tauri;

let buscadorInput;
let greetMsgEl = document.querySelector('#greet-msg');
let tpProd;
let mark;
let variety;
let amount;
let pres;
let cod;
let precio_de_venta;
let precio_de_costo;
let percent;
let timeoutId;
let proveedores = [];
let proveedores_producto = [];
let codigosProv = [];


async function buscador() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  greetMsgEl.textContent = await invoke("buscador", { name: buscadorInput.value });
}
async function agregarProveedor(){
  let prov = document.querySelector('#input-nombre-proveedor');
  let cont = document.querySelector('#input-contacto-proveedor');
  greetMsgEl.textContent = await invoke("agregar_proveedor", { proveedor: prov.value, contacto: cont.value });
  prov.value='';
  cont.value='';
}
async function agregarProducto() {
  greetMsgEl.textContent = ("Producto agregado: " + await invoke("agregarProducto", { proveedores: proveedores_producto, codigosProv: codigosProv, codigoDeBarras: cod.value, precioDeVenta: precio_de_venta.value, porcentaje: percent.value, precioDeCosto: precio_de_costo.value, tipoProducto: tpProd.value, marca: mark.value, variedad: variety.value, cantidad: amount.value, presentacion: pres.value }));
  proveedores_producto = [];
  codigosProv = [];
}

window.addEventListener("DOMContentLoaded", () => {

  document.getElementById("menu-button").onclick = function () {
    let dis = document.getElementById("barra-de-opciones");
    if (dis.style.display == "inline-flex") {
      dis.style.display = "none";
    } else {
      dis.style.display = "inline-flex";
    }
  };
  document.getElementById("agregar-producto-mostrar").onclick = function () {
    let elemento = document.getElementsByClassName("main-screen");
    for (let i = 0; i < elemento.length; i++) {
      elemento[i].style.display = "none"
    }
    document.getElementById("agregar-producto-container").style.display = "inline-flex";
    document.getElementById("barra-de-opciones").style.display = "none";
  }
  document.getElementById("cerrar-agregar-producto").onclick = function () {
    document.getElementById("agregar-producto-container").style.display = "none";
  }

  //** */

  document.getElementById("agregar-proveedor-mostrar").onclick = function () {
    let elemento = document.getElementsByClassName("main-screen");
    for (let i = 0; i < elemento.length; i++) {
      elemento[i].style.display = "none"
    }
    document.getElementById("agregar-proveedor-container").style.display = "inline-flex";
    document.getElementById("barra-de-opciones").style.display = "none";
  }
  document.getElementById("cerrar-agregar-proveedor").onclick = function () {
    document.getElementById("agregar-proveedor-container").style.display = "none";
  }

  
});




window.addEventListener("DOMContentLoaded", () => {

  document.querySelector('#precio_de_costo').addEventListener('input', () => {
    clearTimeout(timeoutId);
    timeoutId = setTimeout(function () {
      if (document.querySelector('#precio_de_costo').value != '') {
        let percent = document.querySelector('#porcentaje').value;
        let sale = document.querySelector('#precio_de_costo').value;
        document.querySelector('#precio_de_venta').value = parseFloat(sale) * (1 + (parseFloat(percent)) / 100)
      }
    }, 500);


  });
});

window.addEventListener("DOMContentLoaded", () => {
  document.querySelector('#porcentaje').addEventListener('input', () => {
    clearTimeout(timeoutId);
    timeoutId = setTimeout(function () {
      if (document.querySelector('#porcentaje').value != '') {
        let percent = document.querySelector('#porcentaje').value;
        let sale = document.querySelector('#precio_de_costo').value;
        if (sale != null) {
          document.querySelector('#precio_de_venta').value = parseFloat(sale) * (1 + (parseFloat(percent)) / 100)
        }
      }
    }, 500);
  });
});

window.addEventListener("DOMContentLoaded", () => {
  document.querySelector('#precio_de_venta').addEventListener('input', () => {
    clearTimeout(timeoutId);
    timeoutId = setTimeout(function () {
      if (document.querySelector('#precio_de_venta').value != '') {
        let costo = document.querySelector('#precio_de_costo');
        let venta = document.querySelector('#precio_de_venta');
        if (costo.value != '') {
          let floatventa = parseFloat(venta.value);
          let floatcosto = parseFloat(costo.value);
          document.querySelector('#porcentaje').value = ((floatventa / floatcosto) * 100.0) - 100.0;
        } else {
          document.querySelector('#precio_de_costo').value = ((100 + parseFloat(document.querySelector('#porcentaje').value) / 100) * parseFloat(venta.value));
        }
      }
    }, 500);
  });
  // document.querySelector('#input-contacto-proveedor').addEventListener('input', () => {
  //   let act=0;
    
  //   let valor = document.querySelector('#input-contacto-proveedor').value;
  //   let res=valor;
  //     if (valor.length>4&&valor[valor.length-4]!='-'){
  //       res = valor.substring(0, valor.length - 4)+'-'+valor.substring(valor.length-3,valor.length);
  //     }
  //   document.querySelector('#input-contacto-proveedor').value=res
  // })
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
  document.querySelector('#agregar-producto-submit').addEventListener("submit", (e) => {
    e.preventDefault();
    agregarProducto();
  })
  document.querySelector('#agregar-proveedor-submit').addEventListener("submit", (e) => {
    e.preventDefault();
    agregarProveedor();
  })
})

window.addEventListener("DOMContentLoaded", async () => {
  let provs = await invoke("get_proveedores")
  proveedores = provs;
  console.log(provs);
  for (let i = 0; i < provs.length; i++) {
    let option = document.createElement("option");
    option.text = provs[i];
    option.value = provs[i];
    document.querySelector('#proveedor').appendChild(option);
  }
})

//Agrega relacion
window.addEventListener("DOMContentLoaded", () => {
  document.querySelector("#agregar-proveedor-a-producto").addEventListener("submit", (e) => {
    e.preventDefault();
    let res = document.querySelector('#proveedor').value;
    let cod = document.querySelector('#codigo_prov').value;
    if (!proveedores_producto.includes(res)) {
      proveedores_producto.push(res);
      codigosProv.push(cod);
    }
    console.log(proveedores_producto + " y " + codigosProv + "|");
  });
})


