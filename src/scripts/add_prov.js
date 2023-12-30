const { invoke } = window.__TAURI__.tauri;


async function agregarProveedor() {
    let prov = document.querySelector('#input-nombre-proveedor');
    let cont = document.querySelector('#input-contacto-proveedor');
    await invoke("agregar_proveedor", { proveedor: prov.value, contacto: cont.value });
    
  }

document.addEventListener('DOMContentLoaded',()=>{
    document.querySelector('#agregar-proveedor-submit').addEventListener('submit',(e)=>{
        e.preventDefault();
        agregarProveedor();
    })
})


document.addEventListener('keydown',(e)=>{
    if (e.keyCode==27){
        close_window();
    }
})


async function close_window() {
    return await invoke("close_window");
}