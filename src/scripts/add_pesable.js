const { invoke } = window.__TAURI__.tauri;
document.addEventListener('keydown',(e)=>{
    if (e.keyCode==27){
        close_window();
    }
})
async function close_window() {
    return await invoke("close_window");
}
async function add_pesable(precio,codigo,costo,porcentaje,descripcion){
    return await invoke("agregar_pesable",{precioPeso: precio, codigo:codigo, costoKilo: costo, porcentaje: porcentaje, descripcion: descripcion})
}
document.getElementsByClassName('add-form')[0].addEventListener('submit',(e)=>{
    let precio=document.getElementById('precio');
    let codigo=document.getElementById('cod');
    let costo=document.getElementById('costo');
    let porcentaje=document.getElementById('porc');
    let desc=document.getElementById('desc');
    e.preventDefault();
    add_pesable(precio.value,codigo.value,costo.value,porcentaje.value,desc.value)
})