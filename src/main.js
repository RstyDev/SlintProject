const { invoke } = window.__TAURI__.tauri;
const mensaje1 = document.querySelector('#mensaje1-msg');
let posicionVenta = 0;
let focuseado;
let timeoutId;
let codigosProv = [];
let codigosProd = [];
let configs;
let idUlt;
let buscador;

get_configs().then(conf => {
  configs = conf;
})




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
        agregarProdVentaAct(focuseado.id).then(venta => {
          e.preventDefault();
          buscador.value = '';
          dibujar_venta(venta)
        });
      } else {
        borrarBusqueda();
      }
    }
  }
}

async function agregar_pago(medio_pago, m) {
  let monto = parseFloat(m);
  return await invoke("agregar_pago", { "medioPago": medio_pago, "monto": monto, "pos": posicionVenta });
}
async function eliminar_pago(index) {
  return await invoke("eliminar_pago", { "pos": posicionVenta, "index": index });
}
async function get_configs() {
  return await invoke("get_configs");
}

async function open_add_product() {
  return await invoke("open_add_product");
}
async function open_add_prov() {
  return await invoke("open_add_prov");
}
async function open_edit_settings() {
  return await invoke("open_edit_settings");
}

function incrementarProducto(e) {
  incrementarProdVentaAct(e.target.parentNode.parentNode.id).then(venta => {
    dibujar_venta(venta);
    setFoco(buscador, document.getElementById('productos'));
  });
}



function sumarProducto(e) {
  agregarProdVentaAct(e.target.parentNode.parentNode.id).then(venta => {
    dibujar_venta(venta);
    setFoco(buscador, document.getElementById('productos'));
  });
}
function restarProducto(e) {
  let cantidad = e.target.nextElementSibling;
  descontarProdVentaAct(e.target.parentNode.parentNode.id).then(venta => {
    dibujar_venta(venta);
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
  if (boton.nextElementSibling && posicionVenta == 1) {
    boton.classList.toggle('v-actual');
    boton.nextElementSibling.classList.toggle('v-actual');
    posicionVenta = 0;
    get_venta_actual().then(venta => dibujar_venta(venta));
    setFoco(buscador, document.getElementById('productos'));
  } else if (boton.previousElementSibling && posicionVenta == 0) {
    boton.classList.toggle('v-actual');
    boton.previousElementSibling.classList.toggle('v-actual');
    posicionVenta = 1;
    get_venta_actual().then(venta => dibujar_venta(venta));
    setFoco(buscador, document.getElementById('productos'));
  }
}

async function get_descripcion_valuable(prod, conf) {
  return await invoke("get_descripcion_valuable", { "prod": prod, "conf": conf });
}

function dibujar_venta(venta) {
  let cuadro = document.querySelector('#cuadro-principal');

  cuadro.replaceChildren([]);



  let strings = "";
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

  for (let producto of venta.productos) {
    let disabled;
    let art;
    if (Object.keys(producto) == 'Pes') {
      if (producto.Pes[0] <= 1) {
        disabled = 'disabled';
      } else {
        disabled = '';
      }

      get_descripcion_valuable(producto, configs).then(strings => {
        art = document.createElement('article');
        art.id = producto.Pes[1].id;
        art.classList.add('articulo');
        art.innerHTML = `
      <section class="descripcion ${configs.modo_mayus}">
         <p > ${strings} </p>
      </section>
      <section class="cantidad">
         <button class="button restar" ${disabled}>-</button>
         <p class="cantidad-producto"> ${producto.Pes[0]}</p>
         <button class="button sumar">+</button>
      </section>
      <section class="monto">
         <p>${producto.Pes[1].precio_peso}</p>
      </section>
      <section>
       <p> ${producto.Pes[1].precio_peso * producto.Pes[0]}</p>
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
    } else if (Object.keys(producto) == 'Rub') {
      if (producto.Rub[0] < 2) {
        disabled = 'disabled';
      } else {
        disabled = '';
      }

      get_descripcion_valuable(producto, configs).then(strings => {
        let art = document.createElement('article');
        art.id = producto.Rub[1].id;
        art.classList.add('articulo');
        art.innerHTML = `
      <section class="descripcion ${configs.modo_mayus}">
         <p> ${strings} </p>
      </section>
      <section class="cantidad">
         <button class="button restar" ${disabled}>-</button>
         <p class="cantidad-producto"> ${producto.Rub[0]}</p>
         <button class="button sumar">+</button>
      </section>
      <section class="monto">
         <p>${producto.Rub[1].monto}</p>
      </section>
      <section>
       <p> ${producto.Rub[1].monto * producto.Rub[0]}</p>
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

    } else if (Object.keys(producto) == 'Prod') {
      if (producto.Prod[0] < 2) {
        disabled = 'disabled';
      } else {
        disabled = '';
      }

      get_descripcion_valuable(producto, configs).then(strings => {

        let art = document.createElement('article');
        art.id = producto.Prod[1].id;
        art.classList.add('articulo');
        art.innerHTML = `
      <section class="descripcion ${configs.modo_mayus}">
         <p> ${strings} </p>
      </section>
      <section class="cantidad">
         <button class="button restar" ${disabled}>-</button>
         <p class="cantidad-producto"> ${producto.Prod[0]}</p>
         <button class="button sumar">+</button>
      </section>
      <section class="monto">
         <p>${producto.Prod[1].precio_de_venta}</p>
      </section>
      <section>
       <p> ${producto.Prod[1].precio_de_venta * producto.Prod[0]}</p>
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
  <input class="input-monto" id="input-activo" type="number" step="0.01" placeholder="Monto"></input>
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
      eliminar_pago(i).then(venta => dibujar_venta(venta));
      setFoco(buscador, document.getElementById('productos'));
    })
  }
  document.getElementById('boton-agregar-pago').addEventListener('click', (e) => {
    e.preventDefault();
    if (parseFloat(e.target.parentNode.children[0].value) > 0) {
      agregar_pago(e.target.parentNode.children[1].value, e.target.parentNode.children[0].value).then(pago => {
        if (isNaN(pago)) {
          console.log('error ' + pago);
        } else {
          if (pago > 0) {
            pasarAPagar();
          } else {
            if (posicionVenta == 0) {
              posicionVenta = 1;
            } else {
              posicionVenta = 0;
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
  if (posicionVenta == 0) {
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


  // pagos.firstChild.addEventListener('submit', () => {
  //   console.log('hacer pago')
  // })

  let opciones = document.getElementsByClassName('opciones-pagos');
  for (let i = 0; i < configs.medios_pago.length; i++) {
    opciones[opciones.length - 1].innerHTML += `<option>${configs.medios_pago[i]}</option>`
  }
  for (let i = 0; i < venta.pagos; i++) {
    pagos.innerHTML += venta.pagos[i];
  }



}


async function get_venta_actual() {
  let res = await invoke("get_venta_actual", { pos: posicionVenta });
  // console.log(ret)
  return res;
}
async function incrementarProdVentaAct(id) {
  return await invoke("incrementar_producto_a_venta", { id: "" + id, pos: "" + posicionVenta });
}
async function agregarProdVentaAct(id) {
  return await invoke("agregar_producto_a_venta", { id: "" + id, pos: "" + posicionVenta });
}
async function descontarProdVentaAct(id) {
  return await invoke("descontar_producto_de_venta", { id: "" + id, pos: "" + posicionVenta });
}

async function eliminarProdVentaAct(id) {
  return await invoke("eliminar_producto_de_venta", { id: "" + id, pos: "" + posicionVenta })
}
function borrarBusqueda() {
  buscador.value = '';
  document.querySelector('#cuadro-principal').replaceChildren([]);
  get_venta_actual().then(venta => {
    dibujar_venta(venta)
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
    tr2.tabIndex = 2;
    tr2.id = objetos[i].Prod[1].id;
    let cantidad;
    let presentacion;
    switch (Object.keys(objetos[i].Prod[1].presentacion)[0]) {
      case 'Gr':
        cantidad = objetos[i].Prod[1].presentacion.Gr;
        presentacion = 'Gr';
        break;
      case 'Un':
        cantidad = objetos[i].Prod[1].presentacion.Un;
        presentacion = 'Un';
        break;
      case 'Lt':
        cantidad = objetos[i].Prod[1].presentacion.Lt;
        presentacion = 'Lt';
        break;
      case 'Ml':
        cantidad = objetos[i].Prod[1].presentacion.Ml;
        presentacion = 'Ml';
        break;
      case 'CC':
        cantidad = objetos[i].Prod[1].presentacion.CC;
        presentacion = 'CC';
        break;
      case 'Kg':
        cantidad = objetos[i].Prod[1].presentacion.Kg;
        presentacion = 'Kg';
        break;
    }
    let id = document.createElement('td');
    id.innerHTML = objetos[i].Prod[1].id;
    id.style.display = 'none'
    let producto = document.createElement('td');
    producto.classList.add(`${configs.modo_mayus}`);
    producto.innerHTML = objetos[i].Prod[1].tipo_producto + ' ' + objetos[i].Prod[1].marca + ' ' + objetos[i].Prod[1].variedad + ' ' + cantidad + ' ' + presentacion;
    tr2.appendChild(producto);
    let precio = document.createElement('td');
    precio.innerHTML = "$  " + objetos[i].Prod[1].precio_de_venta;
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
      agregarProdVentaAct(focuseado.id).then(venta => {
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







function buscadorHandle() {
  buscador.addEventListener('input', () => {
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
      if (e.keyCode == 9 || e.keyCode == 97) {
        let boton = document.querySelector('#v-b');
        if (posicionVenta == 0) {
          cambiar_venta(boton)
        } else {
          cambiar_venta(boton.previousElementSibling)
        }
      }
    }
  })
}


function optionBarHandle() {
  document.body.addEventListener('click', function (e) {

    let ids = [];
    ids.push(e.target);
    if (e.target.parentNode) {
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
    if ((!esId && barra.classList.contains('visible'))) {
      barra.classList.remove('visible');
      barra.classList.remove('para-hamburguesa');
    }
  });
}






function menuButtonHandle() {
  document.getElementById("menu-button").onclick = function () {
    document.getElementById("barra-de-opciones").classList.toggle('visible');
  };
}


function agrProdContHandle() {
  document.getElementById("agregar-producto-mostrar").onclick = function () {
    open_add_product();
    let barra = document.querySelector('#barra-de-opciones');
    barra.classList.remove('visible');
    barra.classList.remove('para-hamburguesa');
  }

}


function cambiarConfHandle() {
  document.getElementById("cambiar-configs-mostrar").onclick = function () {
    open_edit_settings();
    let barra = document.querySelector('#barra-de-opciones');
    barra.classList.remove('visible');
    barra.classList.remove('para-hamburguesa');

  }
}


function mostrarContainerHandle(s2) {
  open_add_product();
  let barra = document.querySelector('#barra-de-opciones');
  barra.classList.remove('visible');
  barra.classList.remove('para-hamburguesa');
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

window.addEventListener("DOMContentLoaded", () => {
  buscador = document.querySelector('#buscador');
  buscador.addEventListener('focus', () => {
    setFoco(buscador, document.getElementById('productos'))
  });
  borrarBusqueda();
  agrProdContHandle();
  buscadorHandle();
  optionBarHandle()
  menuButtonHandle();
  cambiarConfHandle();
  escYf10Press();

  document.getElementById("agregar-proveedor-mostrar").onclick = function () {
    open_add_prov();
    let barra = document.querySelector('#barra-de-opciones');
    barra.classList.remove('visible');
    barra.classList.remove('para-hamburguesa');

  }


});

async function buscarProducto(filtrado) {
  clearTimeout(timeoutId);
  let objetos = await invoke("get_productos_filtrado", { filtro: filtrado });
  dibujarProductos(objetos);

}




async function set_configs(configs) {
  await invoke("set_configs", { configs: configs })
}





function calcFont(height, ps) {
  let res = height / ps;
  if (res > 20) {
    res = 20
  }
  return res * 0.8;
}




function sleep(ms) {
  return new Promise(resolve => setTimeout(resolve, ms));
}
