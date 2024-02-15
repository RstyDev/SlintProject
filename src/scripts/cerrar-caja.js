const { invoke } = window.__TAURI__.tauri;

document.addEventListener('keydown',(e)=>{
    if (e.keyCode==27){
        close_window();
    }
})
async function close_window() {
    return await invoke("close_window");
}
async function get_caja(){
    return await invoke("get_caja")
}
get_caja().then(caja =>{
    let info=document.getElementById('info');
    console.log(caja);
})