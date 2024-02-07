const { invoke } = window.__TAURI__.tauri;
const mensaje1 = document.querySelector('#mensaje1-msg');
let timeoutId;
let proveedores_producto = [];
let codigosProv = [];
let codigosProd = [];
let formSection=document.getElementById('form-section');

window.addEventListener("DOMContentLoaded", () => {
    dibujarProdForm();
    


});

document.getElementById('clase').addEventListener('change',(e)=>{
    console.log(e.target)
})

function dibujarProdForm(){
    formSection.innerHTML=`<div id="agregar-producto-container" class="main-screen">
    
    <section id="input-codigo">
    </section>
    <section id="agregar-proveedor-producto">
        <form >
          
          <section id="agr-codigo">
            <select name="Proveedor" id="proveedor" required>
              <option value="" selected disabled hidden>Seleccione una opción</option>
            </select>
            <input type="number" id="codigo_prov" placeholder="Codigo interno del proveedor">
            
            <button type="submit">Agregar</button>
          </section>
        </form>
    </section>
    <section>
    <form id="agregar-producto-submit">
        <input id="tipo_producto" list="opciones-tipo-producto" placeholder="Tipo de producto" required />
        <datalist id="opciones-tipo-producto">
        </datalist>

        <input id="marca" list="opciones-marca" placeholder="Marca" required />
        <datalist id="opciones-marca">
        </datalist>

      <input id="variedad" placeholder="Variedad" required />

      <input type="number" id="cantidad" placeholder="Cantidad" required />
        <select name="presentacion" id="presentacion" required>
          <option value="" selected disabled hidden>Elige una opción</option>
          <option value="Gr">Grs.</option>
          <option value="Un">Un.</option>
          <option value="Lt">Lt.</option>
          <option value="Ml">Ml.</option>
          <option value="CC">CC.</option>
          <option value="Kg">Kg.</option>
        </select>


        <input type="number" id="precio_de_costo" placeholder="Precio de costo" required />


        <input type="number" id="porcentaje" placeholder="Porcentaje" value='40' required />


        <input type="number" id="precio_de_venta" placeholder="Precio de venta" required />

        <button id="agregar-producto-button">Agregar producto</button>
    </form>
    </section>
  </div>`;
  agrProvSubmit();
    agrCodSubmit();
    agrTipoProd();
    salePriceHandle();
    percentageHandle();
    costPriceHandle();
    agrProdSubmit();
}









async function agregarProducto(tpProd, mark, variety, amount, pres, precio_de_venta, percent, precio_de_costo,esPesable) {
    console.log("agregando " + mark)
    console.log("Producto agregado: " + await invoke("agregar_producto", { proveedores: proveedores_producto, codigosProv: codigosProv, codigosDeBarras: codigosProd, precioDeVenta: precio_de_venta.value, porcentaje: percent.value, precioDeCosto: precio_de_costo.value, tipoProducto: tpProd.value, marca: mark.value, variedad: variety.value, cantidad: amount.value, presentacion: pres.value }));
    proveedores_producto = [];
    codigosProv = [];
    codigosProd = [];
}

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

function handle_codigos(e, input) {
    codigosProd.push(input.children[input.children.length - 2].value);
    console.log(codigosProd);
    while (input.parentElement.children.length > 9) {
        input.parentElement.removeChild(input.parentElement.children[0])
    }
    for (let i = 0; i < codigosProd.length; i++) {
        let input2 = document.createElement('input');
        input2.disabled = 'true';
        console.log(codigosProd[i])
        input2.value = codigosProd[i];
        let btn = document.createElement('button');
        btn.innerText = 'Eliminar'
        btn.classList.add('boton');
        btn.value = 'Eliminar';
        btn.addEventListener('click', (el) => {
            el.preventDefault();
            console.log(el.target.parentElement);
            codigosProd.splice(i, 1)
            el.target.parentElement.parentElement.removeChild(el.target.parentElement);

        });
        let sc = document.createElement('section');
        sc.appendChild(input2);
        sc.appendChild(btn);
        input.parentElement.insertBefore(sc, input);
    }
    e.previousElementSibling.value = '';
    console.log(codigosProd);
}
function salePriceHandle() {
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
}
function agrProdSubmit() {
    let tpProd = document.querySelector('#tipo_producto');
    let mark = document.querySelector('#marca');
    let esPesable=document.getElementById('es-pesable');
    let variety = document.querySelector('#variedad');
    let amount = document.querySelector('#cantidad');
    let pres = document.querySelector('#presentacion');
    let precio_de_venta = document.querySelector('#precio_de_venta');
    let percent = document.querySelector('#porcentaje');
    let precio_de_costo = document.querySelector('#precio_de_costo');
    document.querySelector('#agregar-producto-button').addEventListener("click", (e) => {
        e.preventDefault();
        agregarProducto(tpProd, mark, variety, amount, pres, precio_de_venta, percent, precio_de_costo,esPesable);
    })

}
function agrTipoProd() {
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
}
function agrCodSubmit() {
    let input = document.querySelector('#input-codigo');
    input.innerHTML =
        `
  <input type="number" id="codigo_de_barras" placeholder="Codigo de barras" />
          <button class="boton">Agregar código</button>`

    input.children[input.children.length - 2].addEventListener('keydown', (e) => {

        if (e.keyCode == 13) {
            e.preventDefault();
            handle_codigos(e.target.nextElementSibling, input);
            e.value = '';

        }
    })
    input.children[input.children.length - 1].addEventListener('click', (e) => {
        e.preventDefault();
        handle_codigos(e.target, input)

    })
}
function agrProvSubmit() {
    document.querySelector("#agregar-proveedor-producto").firstElementChild.addEventListener("submit", (e) => {
        e.preventDefault();
        console.log(e.target)
        // console.log(e.target);
        // let res = document.querySelector('#proveedor').value;
        // let cod = document.querySelector('#codigo_prov').value;
        // if (!proveedores_producto.includes(res)) {
        //     proveedores_producto.push(res);
        //     codigosProv.push(cod);
        // }
        // let sect=document.createElement('section');
        // let inp=document.createElement('input');
        // inp.value=res;
        // let val=document.createElement('input');
        // val.value=cod;
        // inp.disabled='true';
        // val.type='double';
        // val.disabled='true';
        // let but=document.createElement('button');
        // but.innerText='Eliminar';
        // but.addEventListener('click',(a)=>{
        //     a.preventDefault();
        //     e.target.removeChild(sect);
        // })
        // sect.appendChild(inp);
        // sect.appendChild(val);
        // sect.appendChild(but);
        // e.target.insertBefore(sect,document.getElementById('agr-codigo'));

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
function percentageHandle() {
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
}
function costPriceHandle() {
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
}


function sleep(ms) {
    return new Promise(resolve => setTimeout(resolve, ms));
}
document.addEventListener('keydown',(e)=>{
    if (e.keyCode==27){
        close_window();
    }
})


async function close_window() {
    return await invoke("close_window");
}