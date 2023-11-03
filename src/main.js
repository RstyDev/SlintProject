const { invoke } = window.__TAURI__.tauri;
let posicionVenta = 0;
let buscadorInput;
let mensaje1 = document.querySelector('#mensaje1-msg');
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
let focuseado;


async function buscador() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  mensaje1.textContent = await invoke("buscador", { name: buscadorInput.value });
}

function navigate(e) {
  if (e.keyCode == 38 && focuseado.previousElementSibling) {
    e.preventDefault();
    focus(focuseado.previousElementSibling);

  } else if (e.keyCode == 40 && focuseado.nextElementSibling) {
    e.preventDefault();
    focus(focuseado.nextElementSibling);

  } else if (e.keyCode == 27) {
    e.preventDefault();
    borrarBusqueda();
  } else if (e.keyCode == 13) {
    agregarProdVentaAct(focuseado);
    e.preventDefault();
    borrarBusqueda();
  }
}

async function agregarProdVentaAct(tr2) {
  await invoke("agregar_producto_a_venta", { id: "" + tr2.children[0].innerHTML, pos: "" + posicionVenta })
}
function borrarBusqueda() {
  document.getElementById('buscador').value = '';
  document.querySelector('#msg-container').replaceChildren([]);
}
async function get_filtrado(filtro, tipo_filtro, objetivo) {
  let res = await invoke("get_filtrado", { filtro: filtro, tipoFiltro: tipo_filtro });
  let ops = document.getElementById(objetivo);
  let opciones = [];
  let esta = false;
  for (let i = 0; i < res.length; i++) {
    if (filtro.toUpperCase() === res[i].toUpperCase()) {
      esta = true;
    }
    let el = document.createElement('option');
    el.value = res[i];
    opciones.push(el);
  }
  if (!esta) {
    let el = document.createElement('option');
    el.value = filtro;
    opciones.push(el);
  }

  ops.replaceChildren([]);
  for (let i = 0; i < opciones.length; i++) {
    ops.appendChild(opciones[i]);
  }
}
window.addEventListener("DOMContentLoaded", () => {
  let id = "tipo_producto";
  let objetivo = "opciones-tipo-producto";
  let id_marca = "marca";
  let objetivo_marca = "opciones-marca";
  document.getElementById(id).addEventListener('input', () => {
    get_filtrado(document.getElementById(id).value, id, objetivo);
  });
  document.getElementById(id).addEventListener('keydown', (e) => {
    if (e.key == 13) {
      document.getElementById(id).value = document.getElementById(objetivo).value;
      document.getElementById(id_marca).focus();
    }
  })

  document.getElementById(id_marca).addEventListener('input', () => {
    get_filtrado(document.getElementById(id_marca).value, id_marca, objetivo_marca);
  });
});

function dibujarProductos(objetos) {
  let container = document.querySelector('#msg-container');
  mensaje1.textContent = '';
  container.replaceChildren([]);
  let tabla = document.createElement('table');
  tabla.style.width = '100%';
  tabla.id = 'tabla-productos';
  let tr;
  tr = document.createElement('tr');
  {
    let th = document.createElement('th');
    th.style.width = '60%'
    th.innerHTML = 'Producto';
    tr.appendChild(th);
    let th3 = document.createElement('th');
    th3.innerHTML = 'Precio';
    tr.appendChild(th3);
  }
  tabla.appendChild(tr);

  for (let i = 0; i < objetos.length; i++) {
    let tr2 = document.createElement('tr')
    tr2.tabIndex = 2;
    let cantidad;
    let presentacion;
    switch (Object.keys(objetos[i].cantidad)[0]) {
      case 'Grs':
        cantidad = objetos[i].cantidad.Grs;
        presentacion = 'Grs';
        break;
      case 'Un':
        cantidad = objetos[i].cantidad.Un;
        presentacion = 'Un';
        break;
      case 'Lt':
        cantidad = objetos[i].cantidad.Lt;
        presentacion = 'Lt';
    }
    let id = document.createElement('td');
    id.innerHTML = objetos[i].id;
    id.style.display = 'none'
    tr2.appendChild(id);
    let producto = document.createElement('td');
    producto.innerHTML = objetos[i].tipo_producto + ' ' + objetos[i].marca + ' ' + objetos[i].variedad + ' ' + cantidad + ' ' + presentacion;
    tr2.appendChild(producto);
    let precio = document.createElement('td');
    precio.innerHTML = objetos[i].precio_de_venta;
    tr2.appendChild(precio);
    console.log(tr2);
    tr2.addEventListener('keydown', (e) => {
      navigate(e)
    });
    tabla.appendChild(tr2);
  }
  container.appendChild(tabla);
  if (tr.nextElementSibling) {
    focus(tr.nextElementSibling);
  }
}
function focus(obj) {
  console.log(focuseado)
  if (focuseado) {
    focuseado.classList.toggle('focuseado')
  }
  focuseado = obj;
  focuseado.classList.toggle('focuseado');
}
async function buscarProducto(filtrado) {
  let objetos = await invoke("get_productos_filtrado", { filtro: filtrado });
  dibujarProductos(objetos);
}
async function agregarProveedor() {
  let prov = document.querySelector('#input-nombre-proveedor');
  let cont = document.querySelector('#input-contacto-proveedor');
  mensaje1.textContent = await invoke("agregar_proveedor", { proveedor: prov.value, contacto: cont.value });
  prov.value = '';
  cont.value = '';
}
async function agregarProducto() {
  mensaje1.textContent = ("Producto agregado: " + await invoke("agregar_producto", { proveedores: proveedores_producto, codigosProv: codigosProv, codigoDeBarras: cod.value, precioDeVenta: precio_de_venta.value, porcentaje: percent.value, precioDeCosto: precio_de_costo.value, tipoProducto: tpProd.value, marca: mark.value, variedad: variety.value, cantidad: amount.value, presentacion: pres.value }));
  proveedores_producto = [];
  codigosProv = [];
}

window.addEventListener("DOMContentLoaded", () => {

  document.getElementById("menu-button").onclick = function () {
    document.getElementById("barra-de-opciones").classList.toggle('visible');
  };
  document.getElementById("agregar-producto-mostrar").onclick = function () {
    let elemento = document.getElementsByClassName("main-screen");
    for (let i = 0; i < elemento.length; i++) {
      elemento[i].style.display = "none"
    }
    document.getElementById("agregar-producto-container").style.display = "inline-flex";
    document.getElementById("barra-de-opciones").classList.remove('visible');
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
    document.getElementById("barra-de-opciones").classList.remove('visible');
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
});
window.addEventListener("DOMContentLoaded", () => {
  document.body.addEventListener('click', function (e) {
    let ids = [];
    ids.push(e.target);
    ids.push(e.target.parentNode);
    ids.push(e.target.parentNode.parentNode);
    let barra=document.querySelector('#barra-de-opciones');
    let esId = false;
    for (const id of ids) {
      if (id.id == 'menu-image'||id.id=='menu-button') {
        esId = true;
      }
    }
    if ((!esId && !barra.classList.contains('visible'))) {
      barra.classList.remove('visible');
      barra.classList.remove('para-hamburguesa');
    } 


  });
})

window.addEventListener("DOMContentLoaded", () => {
  let buscador = document.querySelector('#buscador');
  buscador.addEventListener('input', () => {
    clearTimeout(timeoutId);
    timeoutId = setTimeout(function () {
      buscarProducto(document.querySelector('#buscador').value)
    }, 500);
  });
  buscador.addEventListener('keydown', (e) => {
    navigate(e);
  });
  buscador.addEventListener('submit', (e) => {
    e.preventDefault();

  })

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



function sleep(ms) {
  return new Promise(resolve => setTimeout(resolve, ms));
}