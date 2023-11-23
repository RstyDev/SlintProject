const { invoke } = window.__TAURI__.tauri;
let posicionVenta = 0;
let buscadorInput;
const mensaje1 = document.querySelector('#mensaje1-msg');
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
  let buscador = document.querySelector('#buscador');
  if (focuseado) {
    if (e.keyCode == 38 && focuseado.previousElementSibling.previousElementSibling) {
      e.preventDefault();
      console.log(focuseado)
      focus(focuseado.previousElementSibling);

    } else if (e.keyCode == 40 && focuseado.nextElementSibling.previousElementSibling) {
      e.preventDefault();
      focus(focuseado.nextElementSibling);

    } else if (e.keyCode == 27 && buscador.value.length != 0) {
      e.preventDefault();
      buscador.value = '';
      get_venta_actual().then(venta => dibujar_venta(venta));
    } else if (e.keyCode == 13) {
      agregarProdVentaAct(focuseado.children[0].innerHTML);
      e.preventDefault();
      buscador.value = '';
      get_venta_actual().then(venta => dibujar_venta(venta));
    }
  }
}

function sumarProducto(e) {
  agregarProdVentaAct(e.target.parentNode.parentNode.id);
  borrarBusqueda();
}
function restarProducto(e) {
  let cantidad = e.target.nextElementSibling;
  descontarProdVentaAct(e.target.parentNode.parentNode.id);
  borrarBusqueda();
}

function eliminarProducto(e) {
  eliminarProdVentaAct(e.target.parentNode.id);
  borrarBusqueda();
  console.log(e.target.parentNode.id)
}
function camalize(str) {
  return str.toLowerCase().replace(/[^a-zA-Z0-9]+(.)/g, (m, chr) => chr.toUpperCase());
}
async function formatear_strings(strings){
  let conf=await get_configs()
  console.log(conf)
  
  switch (conf.modo_mayus){
    case "Upper":
      return [strings[0].toUpperCase(), strings[1].toUpperCase()];
    case "Lower":
      return [strings[0].toLowerCase(), strings[1].toLowerCase()];
    case "Camel":
      return [camalize(strings[0]), camalize(strings[1])];
  }
  
}
async function dibujar_venta(venta) {
  console.log(venta);
  let cuadro = document.querySelector('#cuadro-principal');
  cuadro.replaceChildren([]);
  let disabled = "";
  let hijosRes;
  let hijos = "";
  let pres;
  let descripcion;
  let cant;
  for (let producto of venta.productos) {
    if (producto[0] < 2) {
      disabled = 'disabled';
    } else {
      disabled = '';
    }
    switch (Object.keys(producto[1].presentacion)[0]) {
      case 'Gr': {
        pres = "Gr";
        cant = producto[1].presentacion.Gr;
        break;
      }
      case 'Un': {
        pres = "Un";
        cant = producto[1].presentacion.Un;
        break;
      }
      case "Lt": {
        pres = "Lt";
        cant = producto[1].presentacion.Lt;
        break;
      }
      case "Ml": {
        pres = "Ml";
        cant = producto[1].presentacion.Ml;
        break;
      }
      case "Cc": {
        pres = "Cc";
        cant = producto[1].presentacion.Cc;
        break;
      }
      case "Kg": {
        pres = "Kg";
        cant = producto[1].presentacion.Kg;
        break;
      }
    }
    descripcion = `${producto[1].marca} ${producto[1].tipo_producto} ${producto[1].variedad}
         ${cant} ${pres}`
    await formatear_strings([descripcion, "" + producto[0]]).then(strings=>{
      hijos += `<article class="articulo" id="${producto[1].id}">
     <section class="descripcion">
        <p> ${strings[0]} </p>
     </section>
     <section class="cantidad">
       <p> cantidad: </p>
        <button class="button restar" ${disabled}>-</button>
        <p class="cantidad-producto"> ${strings[1]}</p>
        <button class="button sumar">+</button>
     </section>
     <section class="monto">
        <p> Precio: ${producto[1].precio_de_venta} </p>
     </section>
     <section>
      <p> ${producto[1].precio_de_venta*producto[0]}
     </section>
     <section id="borrar">
      <button class="button eliminar">Borrar</button>
    </section>
     </article>`;


    });

    

    hijosRes += `${producto[1].marca} ${producto[1].tipo_producto} ${producto[1].variedad}`
  }
  hijos += `<section id="monto-total"> TOTAL <p>${venta.monto_total}</p></section>`

  cuadro.innerHTML = `<section id="cuadro-venta">${hijos}</section> <section id="vista-resumen-venta"></section>`;
  for (let boton of document.querySelectorAll('.sumar')) {
    boton.addEventListener('click', (e) => { sumarProducto(e) });
  }
  for (let boton of document.querySelectorAll('.restar')) {
    boton.addEventListener('click', (e) => { restarProducto(e) });
  }
  for (let boton of document.querySelectorAll('.eliminar')) {
    boton.addEventListener('click', (e) => {
      eliminarProducto(e)
    });
  }
}


async function get_venta_actual() {
  let res = await invoke("get_venta_actual", { pos: posicionVenta });
  let ret = await res;
  // console.log(ret)
  return ret;
}

async function agregarProdVentaAct(id) {
  await invoke("agregar_producto_a_venta", { id: "" + id, pos: "" + posicionVenta });
}
async function descontarProdVentaAct(id) {
  await invoke("descontar_producto_de_venta", { id: "" + id, pos: "" + posicionVenta });
}

async function eliminarProdVentaAct(id) {
  await invoke("eliminar_producto_de_venta", { id: "" + id, pos: "" + posicionVenta })
} function borrarBusqueda() {
  document.getElementById('buscador').value = '';
  document.querySelector('#cuadro-principal').replaceChildren([]);
  get_venta_actual().then(venta => dibujar_venta(venta));
  document.getElementById('buscador').focus();
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
  borrarBusqueda();
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
  let container = document.querySelector('#cuadro-principal');
  mensaje1.textContent = '';
  container.replaceChildren([]);
  let tabla = document.createElement('table');
  tabla.style.width = '100%';
  tabla.id = 'tabla-productos';
  let tr;
  tr = document.createElement('tr');
  {
    let th = document.createElement('th');
    th.style.width = '80%'
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
    switch (Object.keys(objetos[i].presentacion)[0]) {
      case 'Gr':
        cantidad = objetos[i].presentacion.Gr;
        presentacion = 'Gr';
        break;
      case 'Un':
        cantidad = objetos[i].presentacion.Un;
        presentacion = 'Un';
        break;
      case 'Lt':
        cantidad = objetos[i].presentacion.Lt;
        presentacion = 'Lt';
        break;
      case 'Ml':
        cantidad = objetos[i].presentacion.Ml;
        presentacion = 'Ml';
        break;
      case 'Cc':
        cantidad = objetos[i].presentacion.Cc;
        presentacion = 'Cc';
        break;
      case 'Kg':
        cantidad = objetos[i].presentacion.Kg;
        presentacion = 'Kg';
        break;

    }
    let id = document.createElement('td');
    id.innerHTML = objetos[i].id;
    id.style.display = 'none'
    tr2.appendChild(id);
    let producto = document.createElement('td');
    producto.innerHTML = objetos[i].tipo_producto + ' ' + objetos[i].marca + ' ' + objetos[i].variedad + ' ' + cantidad + ' ' + presentacion;
    tr2.appendChild(producto);
    let precio = document.createElement('td');
    precio.innerHTML = "$  " + objetos[i].precio_de_venta;
    precio.style.textAlign = 'end'
    tr2.appendChild(precio);
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
  if (focuseado) {
    focuseado.classList.toggle('focuseado')
  }
  focuseado = obj;
  focuseado.classList.toggle('focuseado');
}
async function buscarProducto(filtrado) {
  let objetos = await invoke("get_productos_filtrado", { filtro: filtrado });
  if (filtrado.length < 5 || isNaN(filtrado)) {
    clearTimeout(timeoutId);
    dibujarProductos(objetos);
  }
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

async function get_configs() {
  return await invoke("get_configs");
}

async function set_configs(configs) {
  await invoke("set_configs", { configs: configs })
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
    document.querySelector('#cuadro-principal').style.display = 'flex';
  }


  document.getElementById("cambiar-configs-mostrar").onclick = function () {
    let elemento = document.getElementsByClassName("main-screen");
    for (let i = 0; i < elemento.length; i++) {
      elemento[i].style.display = "none"
    }
    document.getElementById("cambiar-configs-container").style.display = "inline-flex";
    document.getElementById("barra-de-opciones").classList.remove('visible');
    get_configs().then(conf => {
      document.querySelector('#input-politica-redondeo').value = conf.politica_redondeo;
      document.querySelector('#input-formato-producto').innerHTML += `<option value="Tmv">Tipo - Marca - Variedad</option>
      <option value="Mtv">Marca - Tipo - Variedad</option>`
      switch (conf.modo_mayus) {
        case "Upper": {
          document.querySelector('#input-modo-mayus').innerHTML += `
          <option value="Upper" >MAYÚSCULAS</option>
          <option value="Camel" >Pimera Letra Mayúscula</option>
          <option value="Lower" >minúsculas</option>
          `;
          break;
        }
        case "Camel": {
          document.querySelector('#input-modo-mayus').innerHTML += `
          <option value="Camel" >Pimera Letra Mayúscula</option>
          <option value="Upper" >MAYÚSCULAS</option>
          <option value="Lower" >minúsculas</option>
          `;
          break;
        }
        case "Lower":{
          document.querySelector('#input-modo-mayus').innerHTML += `
          <option value="Lower" >minúsculas</option>
          <option value="Camel" >Pimera Letra Mayúscula</option>
          <option value="Upper" >MAYÚSCULAS</option>
          `;
          break;
        }
      }


    });

  }
  document.getElementById("cerrar-cambiar-configs").onclick = function () {
    document.getElementById("cambiar-configs-container").style.display = "none";
    document.querySelector('#cuadro-principal').style.display = 'flex';
  }

  document.querySelector('#cambiar-configs-submit').addEventListener('submit', (e) => {
    e.preventDefault();
    let configs = {
      "politica_redondeo": parseFloat(e.target.children[1].value),
      "formato_producto": "" + e.target.children[3].value,
      "modo_mayus": "" + e.target.children[5].value
    }
    console.log(configs)
    set_configs(configs)
  })


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
    document.querySelector('#cuadro-principal').style.display = 'flex';
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
        if (sale != 0) {
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
    let barra = document.querySelector('#barra-de-opciones');
    let esId = false;
    for (const id of ids) {
      if (id.id == 'menu-image' || id.id == 'menu-button') {
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

  buscador.addEventListener('input', (e) => {
    if (buscador.value.length == 0) {
      clearTimeout(timeoutId);
      borrarBusqueda();
    } else {
      buscarProducto(buscador.value)
    }
  });
  buscador.addEventListener('keydown', (e) => {
    navigate(e);
  });
  buscador.addEventListener('submit', (e) => {
    e.preventDefault();
    if (!isNaN(buscador.value) && buscador.value > 4) {
      //TODO
    }
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