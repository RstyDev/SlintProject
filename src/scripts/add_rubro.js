const { invoke } = window.__TAURI__.tauri;

let codigo=document.getElementById('cod');
let descripcion=document.getElementById('desc');
document.addEventListener('keydown',(e)=>{
    if (e.keyCode==27){
        close_window();
    }
})
async function close_window() {
    return await invoke("close_window");
}
async function agregar_rubro(){
    return await invoke("agregar_rubro",{codigo: codigo.value, descripcion: descripcion.value});
}

document.getElementsByClassName('add-form')[0].addEventListener('submit',(e)=>{
    e.preventDefault();
    agregar_rubro()
})