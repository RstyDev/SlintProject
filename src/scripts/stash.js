const { invoke } = window.__TAURI__.tauri;
const { emit, listen } = window.__TAURI__.event;

const resume=document.getElementById('resume');
const details=document.getElementById('details');

async function get_stash(){
    return await invoke("get_stash");
}



get_stash().then(stash=>{console.log(stash)})



const unlisten = await listen('get-venta', (pl) => {
    
})
  