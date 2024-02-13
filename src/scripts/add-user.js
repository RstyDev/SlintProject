const { invoke } = window.__TAURI__.tauri;
document.getElementsByClassName('add-form')[0].addEventListener('submit',()=>{
    addUser(document.getElementById('id').value,document.getElementById('pass').value,document.getElementById('rango').value)
})
document.addEventListener('keydown',(e)=>{
    if (e.keyCode==27){
        close_window();
    }
})
async function close_window() {
    return await invoke("close_window");
}
async function addUser(id,pass,rango){
    return await invoke("agregar_usuario", {id:id, pass:pass,rango:rango})
}