import { invoke } from "@tauri-apps/api/tauri";

async function close_window() {
    return await invoke("close_window");
}

export default function Form({message}){
    document.addEventListener('keydown',(e)=>{
        if (e.keyCode==13){
            //enter
        }else if (e.keyCode==27){
            close_window();
        }
    })
    return(<>
        <p>{message}</p>
        <button>Si</button>
        <button>No</button>
    </>)
}