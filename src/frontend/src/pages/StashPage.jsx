import { invoke } from "@tauri-apps/api/tauri";
async function close_window() {
    return await invoke("close_window");
}

export default function StashPage(){
    document.addEventListener('keydown',(e)=>{
        if (e.keyCode==27){
            close_window();
        }
    })
}