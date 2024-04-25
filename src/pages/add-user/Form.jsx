import { invoke } from "@tauri-apps/api/tauri";
import { useState } from "react";
async function agregarUsuario(id,nombre,pass,rango){
    return await invoke("agregar_usuario",{ id: id, nombre: nombre, pass: pass, rango: rango});
}

export default function Form(){
    const [state,setState] = useState({id:0,nombre:"",pass:"",rango:"Cajero"})
    return(<form onSubmit={()=>{agregarUsuario(state.id,state.nombre,state.pass,state.rango)}}>
        <input type="text" value={state.id} onChange={(e)=>setState({...state, id:e.currentTarget.value})} placeholder="Usuario" required/>
        <input type="text" value={state.nombre} onChange={(e)=>setState({...state,nombre:e.currentTarget.value})} placeholder="Nombre" required/>
        <input type="text" value={state.pass} onChange={(e)=>setState({...state,pass:e.currentTarget.value})} placeholder="Contraseña" required/>
        <select name="rango" id="rango" onChange={(e)=>setState({...state,rango:e.currentTarget.value})}>
            <option selected="selected" value="Cajero">Cajero</option>
            <option value="Admin">Administrador</option>
        </select>
        <input type="submit" value="Agregar" />
    </form>)
}