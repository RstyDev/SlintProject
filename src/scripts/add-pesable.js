const { invoke } = window.__TAURI__.tauri;
document.addEventListener('keydown',(e)=>{
    if (e.keyCode==27){
        close_window();
    }
})
async function close_window() {
    return await invoke("close_window");
}