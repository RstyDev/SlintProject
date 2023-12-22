const { invoke } = window.__TAURI__.tauri;
const mensaje1 = document.querySelector('#mensaje1-msg');
let posicionVenta = 0;
let focuseado;
let timeoutId;
let proveedores_producto = [];
let codigosProv = [];
let codigosProd=[];
let configs;
let idUlt;

get_configs().then(conf => {
  configs = conf;
})




function navigate(e) {
  let buscador = document.querySelector('#buscador');
  if (focuseado) {
    if (e.keyCode == 38 && focuseado.previousElementSibling.previousElementSibling) {
      e.preventDefault();
      focus(focuseado.previousElementSibling);

    } else if (e.keyCode == 40 && focuseado.nextElementSibling) {
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
    } else if (e.keyCode == 121) {
      document.getElementById('cuadro-venta').classList.toggle('focused', false);

    }
  }
}

async function agregar_pago(medio_pago, m) {
  let monto=parseFloat(m);
  return await invoke("agregar_pago", { "medioPago": medio_pago, "monto": monto, "pos": posicionVenta });
}
async function eliminar_pago(index) {
  return await invoke("eliminar_pago", { "pos": posicionVenta, "index": index });
}
async function get_configs() {
  return await invoke("get_configs");
}




function sumarProducto(e) {
  agregarProdVentaAct(e.target.parentNode.parentNode.id);
  get_venta_actual().then(venta => dibujar_venta(venta));
  document.getElementById('buscador').focus();
}
function restarProducto(e) {
  let cantidad = e.target.nextElementSibling;
  descontarProdVentaAct(e.target.parentNode.parentNode.id);
  get_venta_actual().then(venta => dibujar_venta(venta));
  document.getElementById('buscador').focus();
}

function eliminarProducto(e) {
  eliminarProdVentaAct(e.target.parentNode.parentNode.id);
  get_venta_actual().then(venta => dibujar_venta(venta));
  document.getElementById('buscador').focus();
}
function camalize(str) {
  return str.replace(/(\w)(\w*)/g,
    function (g0, g1, g2) { return g1.toUpperCase() + g2.toLowerCase(); });
}
function formatear_descripcion(producto) {
  let pres;
  let cant;
  switch (Object.keys(producto.presentacion)[0]) {
    case 'Gr': {
      pres = "Gr";
      cant = producto.presentacion.Gr;
      break;
    }
    case 'Un': {
      pres = "Un";
      cant = producto.presentacion.Un;
      break;
    }
    case "Lt": {
      pres = "Lt";
      cant = producto.presentacion.Lt;
      break;
    }
    case "Ml": {
      pres = "Ml";
      cant = producto.presentacion.Ml;
      break;
    }
    case "Cc": {
      pres = "Cc";
      cant = producto.presentacion.Cc;
      break;
    }
    case "Kg": {
      pres = "Kg";
      cant = producto.presentacion.Kg;
      break;
    }
  }
  switch (configs.formato_producto) {
    case "Tmv":
      return `${producto.tipo_producto} ${producto.marca} ${producto.variedad} ${cant} ${pres}`;
    case "Mtv":
      return `${producto.marca} ${producto.tipo_producto} ${producto.variedad} ${cant} ${pres}`;
  }


}

function formatear_strings(strings) {
  switch (configs.modo_mayus) {
    case "Upper":
      return strings.toUpperCase();
    case "Lower":
      return strings.toLowerCase();
    case "Camel":
      return camalize(strings);
  }
}
function cambiar_venta(boton) {
  if (boton.nextElementSibling&&posicionVenta==1){
    boton.classList.toggle('v-actual');
    boton.nextElementSibling.classList.toggle('v-actual');
    posicionVenta=0;
    get_venta_actual().then(venta => dibujar_venta(venta));
    document.getElementById('buscador').focus();
  }else if (boton.previousElementSibling&&posicionVenta==0){
    boton.classList.toggle('v-actual');
    boton.previousElementSibling.classList.toggle('v-actual');
    posicionVenta=1;
    get_venta_actual().then(venta => dibujar_venta(venta));
    document.getElementById('buscador').focus();
  }
}


function dibujar_venta(venta) {
  let cuadro = document.querySelector('#cuadro-principal');

  cuadro.replaceChildren([]);
  let disabled = "";
  let hijosRes = "";
  let hijos = `<article class="articulo">
     <section class="descripcion">
        <p> DESCRIPCION </p>
     </section>
     <section class="cantidad">
        <p> CANTIDAD </p>
     </section>
     <section class="monto">
        <p> UNIDAD </p>
     </section>
     <section>
      <p> TOTAL PARCIAL</p>
     
      
    </section>
     </article>`;
  let strings = "";
  for (let producto of venta.productos) {
    if (producto[0] < 2) {
      disabled = 'disabled';
    } else {
      disabled = '';
    }
    strings = formatear_strings(formatear_descripcion(producto[1]))
    hijos += `<article class="articulo" id="${producto[1].id}">
     <section class="descripcion">
        <p> ${strings} </p>
     </section>
     <section class="cantidad">
       
        <button class="button restar" ${disabled}>-</button>
        <p class="cantidad-producto"> ${producto[0]}</p>
        <button class="button sumar">+</button>
     </section>
     <section class="monto">
        <p>${producto[1].precio_de_venta}</p>
     </section>
     <section>
      <p> ${producto[1].precio_de_venta * producto[0]}</p>
     </section>
     <section id="borrar">
      <button class="button eliminar">Borrar</button>
    </section>
     </article>`;



    hijosRes += `<p>${strings}</p>`
  }


  cuadro.innerHTML = `
  <section class="ayb">
  <a id="v-a" class="a-boton"> Venta A </a>
  <a id="v-b" class="a-boton"> Venta B </a>
  </section>
  <section id="cuadro-venta">
    <section id="productos">
    ${hijos}
    </section>
    <section id="monto-total"> TOTAL ${venta.monto_total}</section>
  </section> 
  <section id="resumen-y-pago">
    <div id='resumen'>
      ${hijosRes}
    </div>
    <div id='pagos'>
    </div>
    <p>Resta pagar: ${venta.monto_total - venta.monto_pagado}</p>        
  </section>`;
  let pagos = document.getElementById('pagos');
  for (let i=0;i<venta.pagos.length;i++){
    pagos.innerHTML += `
  <form class="pago">
  <input class="input-monto" type="number" step="0.01" disabled value="${venta.pagos[i].monto}"></input>
  <input class="opciones-pagos" value="${venta.pagos[i].medio_pago}" disabled>
  </input>
  <input class="boton-eliminar-pago" value="Eliminar" type="submit">
    </form>
  ` 
  
  }
  
  pagos.innerHTML += `
  <form class="pago">
  <input class="input-monto" type="number" step="0.01" placeholder="Monto"></input>
  <select class="opciones-pagos">
  </select>
  <input id="boton-agregar-pago" value="Cash" type="submit">
    </form>
  `
  for (let i=0;i<venta.pagos.length;i++){
    let btns=document.getElementsByClassName('boton-eliminar-pago');
    btns[i].addEventListener('click',(e)=>{
      e.preventDefault();
      eliminar_pago(i)
      get_venta_actual().then(venta => dibujar_venta(venta));
      document.getElementById('buscador').focus();
    })
  }
  document.getElementById('boton-agregar-pago').addEventListener('click',(e)=>{
    e.preventDefault();
    if (e.target.parentNode.children[0].value.length>0){
      agregar_pago(e.target.parentNode.children[1].value,e.target.parentNode.children[0].value);
      get_venta_actual().then(venta => dibujar_venta(venta));
      document.getElementById('buscador').focus();
    }
    
  })
  let va=document.getElementById('v-a');
  let vb=document.getElementById('v-b');
  if (posicionVenta==0){
    va.classList.toggle('v-actual', true);
    vb.classList.toggle('v-actual', false);
  }else{
    va.classList.toggle('v-actual', false);
    vb.classList.toggle('v-actual', true);
  }
  va.addEventListener('click', ()=>{
    cambiar_venta(va);
  })
  vb.addEventListener('click',()=>{
    cambiar_venta(vb);
  })
  
  
  
  pagos.firstChild.addEventListener('submit',()=>{
    console.log('hacer pago')
  })

  let opciones = document.getElementsByClassName('opciones-pagos');
  for (let i = 0; i < configs.medios_pago.length; i++) {
    opciones[opciones.length-1].innerHTML += `<option>${configs.medios_pago[i]}</option>`
  }
  for (let i = 0; i < venta.pagos; i++) {
    pagos.innerHTML += venta.pagos[i];
  }
  for (let boton of document.querySelectorAll('.sumar')) {
    boton.addEventListener('click', (e) => { 
      e.preventDefault();
      sumarProducto(e) 
    });
  }
  for (let boton of document.querySelectorAll('.restar')) {
    boton.addEventListener('click', (e) => { 
      e.preventDefault();
      restarProducto(e);
    });
  }
  for (let boton of document.querySelectorAll('.eliminar')) {
    boton.addEventListener('click', (e) => {
      e.preventDefault();
      eliminarProducto(e);
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
}
function borrarBusqueda() {
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

function dibujarProductos(objetos) {
  let container = document.querySelector('#cuadro-principal');
  mensaje1.textContent = '';
  container.replaceChildren([]);
  let tabla = document.createElement('table');
  tabla.style.width = '100%';
  tabla.id = 'tabla-productos';
  let tr = document.createElement('tr');
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
    tr2.style.maxHeight = '1.5em';
    tr2.addEventListener('click', () => {
      document.querySelector('#buscador').focus();
      let focused = tr2.parentNode.getElementsByClassName('focuseado');
      for (let i = 0; i < focused.length; i++) {
        focused[i].classList.toggle('focuseado', false);
      }
      tr2.classList.toggle('focuseado', true);
      focuseado = tr2;

    });
    tr2.addEventListener('dblclick', () => {
      agregarProdVentaAct(focuseado.children[0].innerHTML);
      borrarBusqueda();
    })
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

async function buscarProducto(filtrado) {
  clearTimeout(timeoutId);
  if (filtrado.length < 5 || isNaN(filtrado)) {
    let objetos = await invoke("get_productos_filtrado", { filtro: filtrado });
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
async function agregarProducto(tpProd,mark,variety,amount,pres,precio_de_venta,percent,precio_de_costo) {
  mensaje1.textContent = ("Producto agregado: " + await invoke("agregar_producto", { proveedores: proveedores_producto, codigosProv: codigosProv, codigosDeBarras: codigosProd, precioDeVenta: precio_de_venta.value, porcentaje: percent.value, precioDeCosto: precio_de_costo.value, tipoProducto: tpProd.value, marca: mark.value, variedad: variety.value, cantidad: amount.value, presentacion: pres.value }));
  proveedores_producto = [];
  codigosProv = [];
  codigosProd=[];
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
    document.querySelector('#cuadro-principal').style.display = 'grid';
  }


  document.getElementById("cambiar-configs-mostrar").onclick = function () {
    let elemento = document.getElementsByClassName("main-screen");
    for (let i = 0; i < elemento.length; i++) {
      elemento[i].style.display = "none"
    }
    document.getElementById("cambiar-configs-container").style.display = "inline-flex";
    document.getElementById("barra-de-opciones").classList.remove('visible');

    document.querySelector('#input-politica-redondeo').value = configs.politica_redondeo;
    let inputFormatoProducto = document.querySelector('#input-formato-producto')
    inputFormatoProducto.innerHTML = '';
    inputFormatoProducto.innerHTML += `<option value="Tmv">Tipo - Marca - Variedad</option>
      <option value="Mtv">Marca - Tipo - Variedad</option>`
    document.querySelector('#input-modo-mayus').innerHTML = '';
    switch (configs.modo_mayus) {
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
      case "Lower": {
        document.querySelector('#input-modo-mayus').innerHTML += `
          <option value="Lower" >minúsculas</option>
          <option value="Camel" >Pimera Letra Mayúscula</option>
          <option value="Upper" >MAYÚSCULAS</option>
          `;
        break;
      }
    }
    document.querySelector('#input-cantidad-productos').value = configs.cantidad_productos;


  }
  document.getElementById("cerrar-cambiar-configs").onclick = function () {
    document.getElementById("cambiar-configs-container").style.display = "none";
    document.querySelector('#cuadro-principal').style.display = 'grid';
  }

  document.querySelector('#cambiar-configs-submit').addEventListener('submit', (e) => {
    e.preventDefault();
    let configs2 = {
      "politica_redondeo": parseFloat(e.target.children[1].value),
      "formato_producto": "" + e.target.children[3].value,
      "modo_mayus": "" + e.target.children[5].value,
      "cantidad_productos": parseInt(e.target.children[7].value),
      "medios_pago": configs.medios_pago
    }
    set_configs(configs2)
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
    document.querySelector('#cuadro-principal').style.display = 'grid';
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
    }, 2000);


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
    if (e.target.parentNode){
      ids.push(e.target.parentNode);
      if (e.target.parentNode.parentNode)
      ids.push(e.target.parentNode.parentNode);
    }
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
  let tpProd = document.querySelector('#tipo_producto');
  let mark = document.querySelector('#marca');
  let variety = document.querySelector('#variedad');
  let amount = document.querySelector('#cantidad');
  let pres = document.querySelector('#presentacion');
  let precio_de_venta = document.querySelector('#precio_de_venta');
  let percent = document.querySelector('#porcentaje');
  let precio_de_costo = document.querySelector('#precio_de_costo');
  document.querySelector('#agregar-producto-submit').addEventListener("submit", (e) => {
    e.preventDefault();
    agregarProducto(tpProd,mark,variety,amount,pres,precio_de_venta,percent,precio_de_costo);
  })
  document.querySelector('#agregar-proveedor-submit').addEventListener("submit", (e) => {
    e.preventDefault();
    agregarProveedor();
  })
})

window.addEventListener("DOMContentLoaded", async () => {
  let provs = await invoke("get_proveedores")
  console.log(provs);
  for (let i = 0; i < provs.length; i++) {
    let option = document.createElement("option");
    option.text = provs[i];
    option.value = provs[i];
    document.querySelector('#proveedor').appendChild(option);
  }
})
function handle_codigos(e,input){
  codigosProd.push(input.children[input.children.length-2].value);
  while(input.parentElement.children.length>9){
    input.parentElement.removeChild(input.parentElement.children[0])
  }
  for (let i=0;i<codigosProd.length;i++){
    let input2=document.createElement('input');
    input2.disabled='true';
    console.log(codigosProd[i])
    input2.value=codigosProd[i];
    let btn =document.createElement('button');
    btn.innerText='Eliminar'
    btn.classList.add('boton');
    btn.value='Eliminar';
    btn.addEventListener('click',(el)=>{
      el.preventDefault();
      console.log(el.target.parentElement);
      codigosProd.splice(i, 1)
      el.target.parentElement.parentElement.removeChild(el.target.parentElement);
      
    });
    let sc=document.createElement('section');
    sc.appendChild(input2);
    sc.appendChild(btn);
    input.parentElement.insertBefore(sc, input);
  }
  e.previousElementSibling.value='';
  console.log(codigosProd);
}
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

  let input=document.querySelector('#input-codigo');
  console.log(`largo: `)
  console.log(input.parentElement.children.length)
  input.innerHTML=
  `
  <input type="number" id="codigo_de_barras" placeholder="Codigo de barras" />
          <button class="boton">Agregar código</button>`

  input.children[input.children.length-2].addEventListener('keydown',(e)=>{
    
    if (e.keyCode==13){
      e.preventDefault();
      handle_codigos(e.target.nextElementSibling,input);
      e.value='';

    }
  })
  input.children[input.children.length-1].addEventListener('click',(e)=>{
    e.preventDefault();
    handle_codigos(e.target,input)
    
  })

})



function sleep(ms) {
  return new Promise(resolve => setTimeout(resolve, ms));
}