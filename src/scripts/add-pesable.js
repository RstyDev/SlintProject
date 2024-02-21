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
    return await invoke("add_pesable",{precio_peso: precio, codigo:codigo, costo_kilo: costo, porcentaje: porcentaje, descripcion: descripcion})
}

precio_peso: f64,
    codigo: i64,
    costo_kilo: f64,
    porcentaje: f64,
    descripcion: &'a str,