const { invoke } = window.__TAURI__.tauri;
document.addEventListener('keydown',(e)=>{
    if (e.keyCode==27){
        close_window();
    }
})
async function close_window() {
    return await invoke("close_window");
}
let codigo=document.getElementById('cod');
let descripcion=document.getElementById('desc');
let costo=document.getElementById('costo');

