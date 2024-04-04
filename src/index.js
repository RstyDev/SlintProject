const { invoke } = window.__TAURI__.tauri;
const { emit, listen } = window.__TAURI__.event;




let mensaje1, vacia, user, focuseado, timeoutId, configs, idUlt, buscador;
let posA = true;
let posicionVenta = 0;
let codigosProv = [];
let codigosProd = [];
let beep = new Audio('assets/beep.mp3');
let error = new Audio('assets/error.mp3');
let productosDib = [];
let productosVentaAct = [];
beep.volume = 1;
error.volume = 0.2;

get_configs().then(conf => {
  configs = conf;
});




function navigate(e) {
  if (focuseado) {
    if (e.keyCode == 38 && focuseado.previousElementSibling.previousElementSibling) {
      e.preventDefault();
      focus(focuseado.previousElementSibling);

    } else if (e.keyCode == 40 && focuseado.nextElementSibling) {
      e.preventDefault();
      focus(focuseado.nextElementSibling);

    } else if (e.keyCode == 13) {
      e.preventDefault();

      if (document.getElementById('tabla-productos').children.length > 1) {
        agregarProdVentaAct(productosDib[focuseado.id]).then(venta => {
          e.preventDefault();
          buscador.value = '';
          if (Object.keys(productosDib[focuseado.id]) == 'Prod') {
            beep.play();
          }
          dibujar_venta(venta)
        });
      } else {
        error.play();
        borrarBusqueda();
        buscador.classList.add("error");
        setTimeout(() => { buscador.classList.toggle("error") }, 1000)

      }
    }
  }
}


async function open_confirm_stash(act) {
  return await invoke("open_confirm_stash", { "act": act });
}
async function set_cliente(id) {
  return await invoke("set_cliente", { id: id, pos: posA });
}
async function open_stash() {
  return await invoke("open_stash", { pos: posA });
}
async function open_login() {
  return await invoke("open_login");
}
async function agregar_pago(medio_pago, monto) {

  return await invoke("agregar_pago", { "medioPago": medio_pago, "monto": monto, "pos": posA });
}
async function eliminar_pago(id) {
  return await invoke("eliminar_pago", { "pos": posA, "id": id });
}
async function get_configs() {
  return await invoke("get_configs");
}
async function get_user() {
  return await invoke("get_user");
}
async function call_cerrar_sesion() {
  return await invoke("cerrar_sesion");
}
async function get_clientes() {
  return await invoke("get_clientes");
}
async function open_agregar_cliente() {
  return await invoke("open_agregar_cliente");
}
async function open_add_user() {
  return await invoke("open_add_user");
}
async function open_add_select() {
  return await invoke("open_add_select");
}
async function open_add_prov() {
  return await invoke("open_add_prov");
}
async function open_edit_settings() {
  return await invoke("open_edit_settings");
}
async function open_cancelar_venta() {
  return await invoke("open_cancelar_venta", { act: posA })
}

function incrementarProducto(e) {
  incrementarProdVentaAct(e.target.parentNode.parentNode.id).then(venta => {
    //dibujar_venta(venta);
    setFoco(buscador, document.getElementById('productos'));
  });
}



function sumarProducto(e) {
  agregarProdVentaAct(e.target.parentNode.parentNode.id).then(venta => {
    //dibujar_venta(venta);
    setFoco(buscador, document.getElementById('productos'));
  });
}
function restarProducto(e) {
  let cantidad = e.target.nextElementSibling;
  descontarProdVentaAct(e.target.parentNode.parentNode.id).then(venta => {

    setFoco(buscador, document.getElementById('productos'));
  });
}

function eliminarProducto(e) {
  eliminarProdVentaAct(e.target.parentNode.parentNode.id).then(venta => {
    dibujar_venta(venta);
    setFoco(buscador, document.getElementById('productos'));
  });
}



function cambiar_venta(boton) {
  if (boton.nextElementSibling && !posA) {
    boton.classList.toggle('v-actual');
    boton.nextElementSibling.classList.toggle('v-actual');
    posA = true;
    get_venta_actual().then(venta => dibujar_venta(venta));
    setFoco(buscador, document.getElementById('productos'));
  } else if (boton.previousElementSibling && posA) {
    boton.classList.toggle('v-actual');
    boton.previousElementSibling.classList.toggle('v-actual');
    posA = false;
    get_venta_actual().then(venta => dibujar_venta(venta));
    setFoco(buscador, document.getElementById('productos'));
  }
}

async function get_descripcion_valuable(prod, conf) {
  return await invoke("get_descripcion_valuable", { "prod": prod, "conf": conf });
}


function agregar_options(select, clientes, venta) {

  if (venta.cliente == 'Final') {
    select.innerHTML = `<option value='0' selected>Consumidor Final</option>`;
    for (let cliente of clientes) {
      select.innerHTML += `<option value='${cliente.id}'> ${cliente.nombre}</option>`
    }
  } else {
    console.log(clientes[0])
    select.innerHTML = `<option value='0'>Consumidor Final</option>`;
    let selected;
    for (let cliente of clientes) {
      if (venta.cliente.Regular.id == cliente.id) {
        selected = 'selected';
      } else {
        selected = '';
      }
      select.innerHTML += `<option value='${cliente.id}' ${selected}> ${cliente.nombre}</option>`

    }
  }
}
function dibujar_venta(venta) {
  if (venta.productos.length == 0) {
    vacia = true;
  } else {
    vacia = false;
  };
  console.log(vacia)
  get_clientes().then(clientes => {
    let select = document.getElementById('cliente');
    agregar_options(select, clientes, venta);
    select.addEventListener('change', () => {
      set_cliente(select.value).then(venta => dibujar_venta(venta));
    })
  })
  let cuadro = document.querySelector('#cuadro-principal');
  productosVentaAct = venta.productos;
  buscador.value = '';
  cuadro.replaceChildren([]);
  cuadro.innerHTML = `
  <section class="ayb">
  <a id="v-a" class="a-boton"> Venta A </a>
  <a id="v-b" class="a-boton"> Venta B </a>
  </section>
  <section id="cuadro-venta" >
    <section id="productos" class="focuseable">
    </section>
    <section id="monto-total"> TOTAL ${venta.monto_total}</section>
  </section> 
  <section id="resumen-y-pago">
    <div id='resumen'>
    </div>
    <div id='pagos' class="focuseable not-focused">
    </div>
    <p>Resta pagar: ${venta.monto_total - venta.monto_pagado}</p>        
  </section>`;
  let hijos = document.getElementById('productos');
  let hijosRes = document.getElementById('resumen');
  hijos.innerHTML += `<article class="articulo">
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
  for (let i = 0; i < venta.productos.length; i++) {
    let disabled;
    let art;
    if (Object.keys(venta.productos[i]) == 'Pes') {
      console.log(venta.productos[i].Pes)
      if (venta.productos[i].Pes[0] <= 1) {
        disabled = 'disabled';
      } else {
        disabled = '';
      }

      get_descripcion_valuable(venta.productos[i], configs).then(strings => {
        art = document.createElement('article');
        art.id = i;
        art.classList.add('articulo');
        art.innerHTML = `
      <section class="descripcion ${configs.modo_mayus}">
         <p > ${strings} </p>
      </section>
      <section class="cantidad">
         <button class="button restar" ${disabled}>-</button>
         <p class="cantidad-producto"> ${venta.productos[i].Pes[0]}</p>
         <button class="button sumar">+</button>
      </section>
      <section class="monto">
         <p>${venta.productos[i].Pes[1].precio_peso}</p>
      </section>
      <section>
       <p> ${(parseFloat(venta.productos[i].Pes[1].precio_peso) * venta.productos[i].Pes[0]).toFixed(2)}</p>
      </section>
      <section id="borrar">
       <button class="button eliminar">Borrar</button>
     </section>
      `;
        ///----
        console.log(art.children[0])


        art.children[1].children[2].addEventListener('click', (e) => {
          e.preventDefault();
          incrementarProducto(e);
        });

        art.children[1].children[0].addEventListener('click', (e) => {
          e.preventDefault();
          restarProducto(e);
        });

        art.children[4].children[0].addEventListener('click', (e) => {
          e.preventDefault();
          eliminarProducto(e);
        });




        hijos.appendChild(art);

        hijosRes.innerHTML += `<p>${strings}</p>`
      });
    } else if (Object.keys(venta.productos[i]) == 'Rub') {
      if (venta.productos[i].Rub[0] < 2) {
        disabled = 'disabled';
      } else {
        disabled = '';
      }

      get_descripcion_valuable(venta.productos[i], configs).then(strings => {
        let art = document.createElement('article');
        art.id = i;
        art.classList.add('articulo');
        art.innerHTML = `
      <section class="descripcion ${configs.modo_mayus}">
         <p> ${strings} </p>
      </section>
      <section class="cantidad">
         <button class="button restar" ${disabled}>-</button>
         <p class="cantidad-producto"> ${venta.productos[i].Rub[0]}</p>
         <button class="button sumar">+</button>
      </section>
      <section class="monto">
         <p>${venta.productos[i].Rub[1].monto}</p>
      </section>
      <section>
       <p> ${(parseFloat(venta.productos[i].Rub[1].monto * venta.productos[i].Rub[0])).toFixed(2)}</p>
      </section>
      <section id="borrar">
       <button class="button eliminar">Borrar</button>
     </section>
      `;
        ///----


        art.children[1].children[2].addEventListener('click', (e) => {
          e.preventDefault();
          incrementarProducto(e);
        });

        art.children[1].children[0].addEventListener('click', (e) => {
          e.preventDefault();
          restarProducto(e);
        });

        art.children[4].children[0].addEventListener('click', (e) => {
          e.preventDefault();
          eliminarProducto(e);
        });




        hijos.appendChild(art);

        hijosRes.innerHTML += `<p>${strings}</p>`
      });

    } else if (Object.keys(venta.productos[i]) == 'Prod') {
      if (venta.productos[i].Prod[0] < 2) {
        disabled = 'disabled';
      } else {
        disabled = '';
      }

      get_descripcion_valuable(venta.productos[i], configs).then(strings => {

        let art = document.createElement('article');
        art.id = i;
        art.classList.add('articulo');
        art.innerHTML = `
      <section class="descripcion ${configs.modo_mayus}">
         <p> ${strings} </p>
      </section>
      <section class="cantidad">
         <button class="button restar" ${disabled}>-</button>
         <p class="cantidad-producto"> ${venta.productos[i].Prod[0]}</p>
         <button class="button sumar">+</button>
      </section>
      <section class="monto">
         <p>${venta.productos[i].Prod[1].precio_de_venta}</p>
      </section>
      <section>
       <p> ${(parseFloat(venta.productos[i].Prod[1].precio_de_venta * venta.productos[i].Prod[0])).toFixed(2)}</p>
      </section>
      <section id="borrar">
       <button class="button eliminar">Borrar</button>
     </section>
      `;
        ///----


        art.children[1].children[2].addEventListener('click', (e) => {
          e.preventDefault();
          incrementarProducto(e);
        });

        art.children[1].children[0].addEventListener('click', (e) => {
          e.preventDefault();
          restarProducto(e);
        });

        art.children[4].children[0].addEventListener('click', (e) => {
          e.preventDefault();
          eliminarProducto(e);
        });




        hijos.appendChild(art);

        hijosRes.innerHTML += `<p>${strings}</p>`
      });
    }





  }



  hijosRes.style.fontSize = `${calcFont(hijosRes.offsetHeight, hijosRes.children.length * 2)}px`;
  for (let i = 0; i < hijosRes.length; i++) {
    hijosRes[i].style.height = `${calcFont(hijosRes.offsetHeight, hijosRes.children.length)}px`;
  }




  let pagos = document.getElementById('pagos');
  for (let i = 0; i < venta.pagos.length; i++) {
    console.log(venta.pagos)
    console.log(venta.pagos[i].medio_pago.medio);
    pagos.innerHTML += `
  <form class="pago" id="${venta.pagos[i].int_id}">
  <input class="input-monto" type="number" step="0.01" disabled value="${venta.pagos[i].monto}" required></input>
  <input class="opciones-pagos" value="${venta.pagos[i].medio_pago.medio}" disabled>
  </input>
  <input class="boton-eliminar-pago" value="Eliminar" type="submit">
    </form>
  `

  }

  pagos.innerHTML += `
  <form class="pago">
  <input class="input-monto" id="input-activo" type="number" step="0.01" placeholder="Monto" required></input>
  <select class="opciones-pagos">
  </select>
  <input id="boton-agregar-pago" value="Cash" type="submit">
    </form>
  `
  let input = document.querySelector('#input-activo');
  input.addEventListener('focus', () => {
    setFoco(input, pagos)
  })
  for (let i = 0; i < venta.pagos.length; i++) {
    let btns = document.getElementsByClassName('boton-eliminar-pago');
    btns[i].addEventListener('click', (e) => {
      e.preventDefault();
      console.log(e.target.parentNode)
      eliminar_pago(e.target.parentNode.id).then(venta => dibujar_venta(venta));
      setFoco(buscador, document.getElementById('productos'));
    })
  }
  document.getElementById('boton-agregar-pago').addEventListener('click', (e) => {
    e.preventDefault();
    if (parseFloat(e.target.parentNode.children[0].value) > 0) {
      agregar_pago(e.target.parentNode.children[1].value, e.target.parentNode.children[0].value, posA).then(pago => {
        if (isNaN(pago)) {
          console.log('error ' + pago);
        } else {
          if (pago > 0) {
            pasarAPagar();
          } else {
            if (posA) {
              posA = false;
            } else {
              posA = true;
            }
            get_venta_actual().then(venta => {
              dibujar_venta(venta);
              setFoco(buscador, document.getElementById('productos'));
            });

          }
        }



      })
    }

  })
  let va = document.getElementById('v-a');
  let vb = document.getElementById('v-b');
  if (posA) {
    va.classList.toggle('v-actual', true);
    vb.classList.toggle('v-actual', false);
  } else {
    va.classList.toggle('v-actual', false);
    vb.classList.toggle('v-actual', true);
  }
  va.addEventListener('click', () => {
    cambiar_venta(va);
  })
  vb.addEventListener('click', () => {
    cambiar_venta(vb);
  })


  let opciones = document.getElementsByClassName('opciones-pagos');
  for (let i = 0; i < configs.medios_pago.length; i++) {
    opciones[opciones.length - 1].innerHTML += `<option value='${configs.medios_pago[i]}'>${configs.medios_pago[i]}</option>`
  }
  if (venta.cliente != 'Final' && venta.cliente.Regular.limite != 'Unauth') {
    opciones[opciones.length - 1].innerHTML += `<option value='Cuenta Corriente'>Cuenta Corriente</option>`
  }
  for (let i = 0; i < venta.pagos; i++) {
    pagos.innerHTML += venta.pagos[i];
  }



}


async function get_venta_actual() {
  let res = await invoke("get_venta_actual", { pos: posA });

  return res;
}
async function incrementarProdVentaAct(index) {
  return await invoke("incrementar_producto_a_venta", { index: index, pos: posA });
}
async function agregarProdVentaAct(prod) {
  return await invoke("agregar_producto_a_venta", { prod: prod, pos: posA });
}
async function descontarProdVentaAct(i) {
  return await invoke("descontar_producto_de_venta", { index: i, pos: posA });
}

async function eliminarProdVentaAct(index) {
  return await invoke("eliminar_producto_de_venta", { index: index, pos: posA })
}
async function open_cerrar_caja() {
  return await invoke("open_cerrar_caja")
}
function borrarBusqueda() {
  buscador.value = '';
  document.querySelector('#cuadro-principal').replaceChildren([]);
  get_venta_actual().then(venta => {
    dibujar_venta(venta);
    setFoco(buscador, document.getElementById('productos'));
  });
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

function dibujarProductos() {

  let container = document.querySelector('#cuadro-principal');

  mensaje1.textContent = '';
  while (container.firstChild) {
    container.removeChild(container.firstChild);
  }
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

  for (let i = 0; i < productosDib.length; i++) { //hay que ver este for
    console.log(Object.keys(productosDib[i])[0]);
    switch (Object.keys(productosDib[i])[0]) {
      case 'Prod':
        agregarProd(tabla, productosDib[i], i);
        break;
      case 'Pes':
        agregarPes(tabla, productosDib[i], i);
        break;
      case 'Rub':
        agregarRub(tabla, productosDib[i], i);
        break;

    }
  }
  container.appendChild(tabla);
  if (tr.nextElementSibling) {
    if (tabla.children.length == 1) {
      focuseado = {
        id: -1
      }
      console.log(focuseado);
    } else {
      focus(tr.nextElementSibling);
    }
  }
}
function agregarRub(tabla, objeto, i) {
  let tr2 = document.createElement('tr')
  tr2.style.maxHeight = '1.5em';
  tr2.tabIndex = 2;
  tr2.id = i;
  let id = document.createElement('td');
  id.innerHTML = objeto.Rub[1].id;
  id.style.display = 'none'
  let producto = document.createElement('td');
  producto.classList.add(`${configs.modo_mayus}`);
  producto.innerHTML = objeto.Rub[1].descripcion;
  tr2.appendChild(producto);
  let precio = document.createElement('td');
  precio.style.textAlign = 'end'
  tr2.appendChild(precio);
  tr2.addEventListener('click', () => {
    let focused = tr2.parentNode.getElementsByClassName('focuseado');
    for (let i = 0; i < focused.length; i++) {
      focused[i].classList.toggle('focuseado', false);
    }
    tr2.classList.toggle('focuseado', true);
    focuseado = tr2;

  });
  tr2.addEventListener('dblclick', () => {

    agregarProdVentaAct(productosDib[focuseado.id]).then(venta => {

      dibujar_venta(venta);
      setFoco(buscador, document.getElementById('productos'));
    });
  });
  tr2.addEventListener('keydown', (e) => {
    navigate(e)
  });
  tr2.appendChild(id);
  tabla.appendChild(tr2);
}
function agregarPes(tabla, objeto, i) {
  let tr2 = document.createElement('tr')
  tr2.style.maxHeight = '1.5em';
  tr2.tabIndex = 2;
  tr2.id = i;
  let id = document.createElement('td');
  id.innerHTML = objeto.Pes[1].id;
  id.style.display = 'none'
  let producto = document.createElement('td');
  producto.classList.add(`${configs.modo_mayus}`);
  producto.innerHTML = objeto.Pes[1].descripcion;
  tr2.appendChild(producto);
  let precio = document.createElement('td');
  precio.innerHTML = "$  " + objeto.Pes[1].precio_peso;
  precio.style.textAlign = 'end'
  tr2.appendChild(precio);
  tr2.addEventListener('click', () => {
    let focused = tr2.parentNode.getElementsByClassName('focuseado');
    for (let i = 0; i < focused.length; i++) {
      focused[i].classList.toggle('focuseado', false);
    }
    tr2.classList.toggle('focuseado', true);
    focuseado = tr2;

  });
  tr2.addEventListener('dblclick', () => {

    agregarProdVentaAct(productosDib[focuseado.id]).then(venta => {

      dibujar_venta(venta);
      setFoco(buscador, document.getElementById('productos'));
    });
  });
  tr2.addEventListener('keydown', (e) => {
    navigate(e)
  });
  tr2.appendChild(id);
  tabla.appendChild(tr2);
}
function agregarProd(tabla, objeto, i) {
  let tr2 = document.createElement('tr')
  tr2.style.maxHeight = '1.5em';
  tr2.tabIndex = 2;
  tr2.id = i;
  let cantidad;
  let presentacion;
  switch (Object.keys(objeto.Prod[1].presentacion)[0]) {
    case 'Gr':
      cantidad = objeto.Prod[1].presentacion.Gr;
      presentacion = 'Gr';
      break;
    case 'Un':
      cantidad = objeto.Prod[1].presentacion.Un;
      presentacion = 'Un';
      break;
    case 'Lt':
      cantidad = objeto.Prod[1].presentacion.Lt;
      presentacion = 'Lt';
      break;
    case 'Ml':
      cantidad = objeto.Prod[1].presentacion.Ml;
      presentacion = 'Ml';
      break;
    case 'CC':
      cantidad = objeto.Prod[1].presentacion.CC;
      presentacion = 'CC';
      break;
    case 'Kg':
      cantidad = objeto.Prod[1].presentacion.Kg;
      presentacion = 'Kg';
      break;
  }
  let id = document.createElement('td');
  id.innerHTML = objeto.Prod[1].id;
  id.style.display = 'none'
  let producto = document.createElement('td');
  producto.classList.add(`${configs.modo_mayus}`);
  producto.innerHTML = objeto.Prod[1].tipo_producto + ' ' + objeto.Prod[1].marca + ' ' + objeto.Prod[1].variedad + ' ' + cantidad + ' ' + presentacion;
  tr2.appendChild(producto);
  let precio = document.createElement('td');
  precio.innerHTML = "$  " + objeto.Prod[1].precio_de_venta;
  precio.style.textAlign = 'end'
  tr2.appendChild(precio);
  tr2.addEventListener('click', () => {
    let focused = tr2.parentNode.getElementsByClassName('focuseado');
    for (let i = 0; i < focused.length; i++) {
      focused[i].classList.toggle('focuseado', false);
    }
    tr2.classList.toggle('focuseado', true);
    focuseado = tr2;

  });
  tr2.addEventListener('dblclick', () => {
    console.log(productosDib[focuseado.id]);
    agregarProdVentaAct(productosDib[focuseado.id]).then(venta => {
      beep.play();
      dibujar_venta(venta);
      setFoco(buscador, document.getElementById('productos'));
    });
  });
  tr2.addEventListener('keydown', (e) => {
    navigate(e)
  });
  tr2.appendChild(id);
  tabla.appendChild(tr2);
}
function focus(obj) {
  if (focuseado) {
    focuseado.classList.toggle('focuseado')
  }
  focuseado = obj;
  focuseado.classList.toggle('focuseado');
}






function buscadorHandle() {
  buscador.addEventListener('input', () => {
    if (buscador.value.length < 1) {
      clearTimeout(timeoutId);
      borrarBusqueda();
    } else {
      buscarProducto(buscador.value)
    }
  });
  buscador.addEventListener('keydown', (e) => {
    navigate(e);
  });
}

function pasarAPagar() {
  buscador.value = '';
  get_venta_actual().then(venta => {
    dibujar_venta(venta);
    let input = document.querySelector('#input-activo');
    input.value = venta.monto_total - venta.monto_pagado;
    setFoco(input, document.getElementById('pagos'));
  });
}

function escYf10Press() {
  document.body.addEventListener('keydown', (e) => {
    if (e.keyCode == 27) {
      e.preventDefault();
      buscador.value = '';
      get_venta_actual().then(venta => {
        dibujar_venta(venta)
        setFoco(buscador, document.getElementById('productos'));
      });

    } else if (e.keyCode == 121) {
      e.preventDefault();
      pasarAPagar();
    } else if (e.ctrlKey) {
      if (e.keyCode == 9) {
        let boton = document.querySelector('#v-b');
        if (posA) {
          cambiar_venta(boton)
        } else {
          cambiar_venta(boton.previousElementSibling)
        }
      } else if (e.keyCode == 71 && !vacia) {
        e.preventDefault();
        open_confirm_stash(posA);
      } else if (e.keyCode == 83 && vacia) {
        e.preventDefault();
        open_stash();
      }
    }
  })
}














function setFoco(foco, focuseable) {
  let focos = document.querySelectorAll('.focuseable');
  for (let i = 0; i < focos.length; i++) {
    focos[i].classList.toggle('not-focused', true);
  }
  focuseable.classList.toggle('not-focused', false);
  foco.focus();
  foco.select();
}



async function buscarProducto(filtrado) {
  clearTimeout(timeoutId);
  productosDib = await invoke("get_productos_filtrado", { filtro: '' + filtrado });
  dibujarProductos();

}

function PlaySound(soundObj) {
  var sound = document.getElementById(soundObj);
  sound.Play();
}


function cerrar_sesion() {
  user = '';
  document.getElementsByTagName('body')[0].innerHTML = '';
  call_cerrar_sesion()
}
function dibujar_base() {
  get_user().then(usuario => {

    user = usuario;

    document.getElementsByTagName('body')[0].innerHTML = `<header class="container">
        <div id="header">
            <div>
                <form autocomplete="off">
                    <input type="text" id="buscador">
                </form>
            </div>
            <div>
                <select id="cliente">
                  
                </select>
            </div>
        </div>
    </header>
    <main>
        <div id="msg-container">
            <p id="mensaje1-msg"></p>
            <div id="test"></div>
        </div>
        <section id="cuadro-principal" class="main-screen">
        </section>
    </main>`;



    mensaje1 = document.querySelector('#mensaje1-msg');
    buscador = document.querySelector('#buscador');
    document.addEventListener("keydown", (e) => {

      if (e.keyCode == 115 && !vacia) {
        open_cancelar_venta();
      }

    });

    buscador.addEventListener('focus', () => {
      let prod = document.getElementById('productos');
      if (prod) {
        setFoco(buscador, prod)
      }
    });
    borrarBusqueda();

    buscadorHandle();

    escYf10Press();

  })
}





function calcFont(height, ps) {
  let res = height / ps;
  if (res > 20) {
    res = 20
  }
  return res * 0.8;
}



open_login();

function sleep(ms) {
  return new Promise(resolve => setTimeout(resolve, ms));
}


const unlisten = await listen('main', (pl) => {
  if (pl.payload.message == 'dibujar venta') {
    get_venta_actual().then(venta => dibujar_venta(venta));
  } else if (pl.payload.message == "confirm stash") {
    open_confirm_stash(posA)
  } else if (pl.payload.message == "inicio sesion") {
    dibujar_base();
  } else if (pl.payload.message == "cerrar sesion") {
    cerrar_sesion()
  } else if (pl.payload.message == "open stash") {
    open_stash()
  }
})
