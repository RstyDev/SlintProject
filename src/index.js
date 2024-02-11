const { invoke } = window.__TAURI__.tauri;

async function open_main() {
    return await invoke("open_main");
}

setTimeout(()=>{
    console.log('aca');
    open_main()}, 1000);