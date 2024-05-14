const { invoke } = window.__TAURI__.tauri;
const { emit, listen } = window.__TAURI__.event;
const resume = document.getElementById('resume');
const details = document.getElementById('details');
let configs;
let focused;
let stash;
let pos;


async function get_configs() {
    return await invoke("get_configs");
}
async function get_stash() {
    return await invoke("get_stash");
}


function dibujarVenta(venta) {
    let prods = details.firstChild;
    while (prods.childNodes[1]) {
        prods.removeChild(prods.childNodes[1]);
    }
    let pays = prods.nextElementSibling;
    while (pays.childNodes[1]) {
        pays.removeChild(pays.childNodes[1]);
    }
    let tot = pays.nextElementSibling;
    if (tot.childNodes[1]) {
        tot.removeChild(tot.childNodes[1]);
    }
    for (let i = 0; i < venta.productos.length; i++) {
        let act = venta.productos[i];
        switch (Object.keys(act)[0]) {
            case 'Prod': {
                // console.log(venta.productos[i].Prod);
                let row = document.createElement('tr');
                if (configs.formato_producto == 'Tmv') {
                    row.innerHTML = `${act.Prod[1].tipo_producto} ${act.Prod[1].marca} ${act.Prod[1].variedad}`
                } else {
                    row.innerHTML = `${act.Prod[1].marca} ${act.Prod[1].tipo_producto} ${act.Prod[1].variedad}`
                }
                prods.appendChild(row);
                break;
            }
            case 'Rub': {
                let row = document.createElement('tr');
                row.innerHTML = `${act.Rub[1].descripcion}`
                prods.appendChild(row);
                break;
            }
            case 'Pes': {
                let row = document.createElement('tr');
                row.innerHTML = `${act.Pes[1].descripcion}`
                prods.appendChild(row);
                break;
            }
        }

    }
    for (let i = 0; i < venta.pagos.length; i++) {
        let act = venta.pagos[i];
        let row = document.createElement('tr');
        row.innerHTML = `${act.medio_pago.medio} ${act.monto}`
        row.style.textAlign = 'center'
        pays.appendChild(row);
    }
    let row = document.createElement('tr');
    row.innerHTML = `$ ${venta.monto_total}`;
    row.style.textAlign = 'end'
    tot.appendChild(row);

}

get_configs().then(conf => {
    configs = conf;
});

get_stash().then(st => {
    stash = st;
    let tab = document.createElement('table');
    resume.appendChild(tab);
    let tr = document.createElement('tr');
    tab.appendChild(tr);
    let id = document.createElement('th');
    id.classList.add('id');
    tr.appendChild(id);
    let cant = document.createElement('th');
    cant.innerHTML = `Cantidad de productos`
    tr.appendChild(cant);
    let monto = document.createElement('th');
    monto.innerHTML = `Monto Total`
    tr.appendChild(monto);

    let tabd = document.createElement('table');
    let tabp = document.createElement('table');
    let tabt = document.createElement('table');
    details.appendChild(tabd);
    details.appendChild(tabp);
    details.appendChild(tabt);
    let trd = document.createElement('tr');
    let trp = document.createElement('tr');
    let trt = document.createElement('tr');
    tabd.appendChild(trd);
    tabp.appendChild(trp);
    tabt.appendChild(trt);
    let prods = document.createElement('th');
    prods.innerHTML = `Productos`
    trd.appendChild(prods);
    let pagos = document.createElement('th');
    pagos.innerHTML = `Pagos`
    trp.appendChild(pagos);
    let total = document.createElement('th');
    total.innerHTML = `Total de Venta`
    trt.appendChild(total);


    for (let i = 0; i < stash.length; i++) {
        let tr2 = document.createElement('tr');
        tab.appendChild(tr2);
        let id2 = document.createElement('td');
        id2.innerHTML = `${i}`
        id2.classList.add('id');
        tr2.appendChild(id2);
        if (i == 0){
            focused = tr2;
            dibujarVenta(stash[focused.firstChild.innerText]);
            focused.style.border = 'solid 2px white';
            focused.classList.add('focused');
        }
        tr2.addEventListener('click', (e) => {
            
            focused = e.target.parentElement;
            console.log(focused);
            dibujarVenta(stash[focused.firstChild.innerText])
            let els = document.getElementsByClassName('focused');
            for (let j = 0; j < els.length; j++) {
                els[j].style.border = 'none'
                els[j].classList.remove('focused');
            }
            focused.style.border = 'solid 2px white';
            focused.classList.add('focused');

        });
        let cant2 = document.createElement('td');
        tr2.appendChild(cant2);
        let monto2 = document.createElement('td');
        tr2.appendChild(monto2);
        cant2.innerHTML = `${stash[i].productos.length} Productos`
        monto2.innerHTML = `${stash[i].monto_total}`


        console.log(stash[i].monto_total);


    }
    focused = tr.nextElementSibling;
    tr.nextElementSibling.classList.add('focused');
    tr.nextElementSibling.style.border = 'solid 2px white'


})

function unfocus(item){
    item.style.border = 'none'
    item.classList.remove('focused');
}
function focus(item){
    item.style.border = 'solid 2px white';
    item.classList.add('focused');
}

document.addEventListener('keydown',(e)=>{
    if (e.keyCode==27){
        close_window();
    }else if (e.keyCode==40 && focused.nextElementSibling){ //abajo
        unfocus(focused);
        focused = focused.nextElementSibling;
        focus(focused);
        dibujarVenta(stash[focused.firstChild.innerText])
    }else if (e.keyCode==38 && focused.previousElementSibling.previousElementSibling){
        unfocus(focused);
        focused = focused.previousElementSibling;
        focus(focused);
        dibujarVenta(stash[focused.firstChild.innerText])
    }else if (e.keyCode==13||e.keyCode==121){
        unstashSale(focused.childNodes[0].innerText);
    }
})
async function close_window() {
    return await invoke("close_window");
}
async function unstashSale(index){
    return await invoke("unstash_sale",{pos:pos, index:index})
}

const unlisten = await listen('stash', (pl) => {
    pos = pl.payload.pos;
})
