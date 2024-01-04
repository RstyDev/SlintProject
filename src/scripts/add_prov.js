const { invoke } = window.__TAURI__.tauri;

document.addEventListener('DOMContentLoaded',()=>{
    document.querySelector('#agregar-proveedor-submit').addEventListener('submit',(e)=>{
        e.preventDefault();
        let nombre=document.querySelector('#input-nombre-proveedor');
        let contacto=document.querySelector('#input-contacto-proveedor');
        agregarProveedor(nombre.value,contacto.value).catch(error=>{console.log(error)})
    })
})

async function agregarProveedor(nombre,contacto) {
    return await invoke("agregar_proveedor", { "proveedor": nombre, "contacto": contacto });
}

async function close_window() {
    return await invoke("close_window");
}