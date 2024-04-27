import { invoke } from "@tauri-apps/api/tauri";
import { useState } from "react";

async function agregarProveedor(prov,contact){
    return await invoke("agregar_proveedor",{proveedor:prov ,contacto:contact});
}
async function close_window() {
    return await invoke("close_window");
}

export default function Form(){
    document.addEventListener('keydown',(e)=>{
        if (e.keyCode==13){
            //enter
        }else if (e.keyCode==27){
            close_window();
        }
    })
    const [state,setState]=useState({prov:"",cont:""});
    return(<form onSubmit={()=>agregarProveedor(state.prov,state.cont)}>
        <input type="text" name="Proveedor" value={state.prov} onChange={(e)=>setState({...state,prov: e.currentTarget.value})} required placeholder="Proveedor" />
        <input type="number" name="Contacto" value={state.cont} onChange={(e)=>setState({...state,cont: e.currentTarget.value})} id="contacto" placeholder="Contacto" />
        <input type="submit" value="Agregar" />
    </form>)
}