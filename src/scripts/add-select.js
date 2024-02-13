const { invoke } = window.__TAURI__.tauri;
document.addEventListener('keydown',(e)=>{
    if (e.keyCode==27){
        close_window();
    }
})
async function close_window() {
    return await invoke("close_window");
}
async function select(dato){
    return await invoke("select_window", {dato:dato})
}
let clase=document.getElementById('clase');
clase.focus();
clase.addEventListener('change',()=>{
    select(clase.value);
})