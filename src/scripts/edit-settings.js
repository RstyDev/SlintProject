const { invoke } = window.__TAURI__.tauri;
let configs;
get_configs().then(conf => {
    configs = conf;
    
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

})

function changeConfigsHandle() {
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
}

document.addEventListener('keydown',(e)=>{
    if (e.keyCode==27){
        close_window();
    }
})


async function close_window() {
    return await invoke("close_window");
}

async function get_configs() {
    return await invoke("get_configs");
}