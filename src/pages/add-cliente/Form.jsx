import "./Form.css"
import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";

async function get_rango() {
    return await invoke("get_rango");
}
async function close_window() {
    return await invoke("close_window");
}
async function agregar_cliente(nombre, dni, credito, limite) {
    return await invoke("agregar_cliente", { nombre: nombre, dni: dni, credito: credito, limite: limite })
}
function Form(){
    document.addEventListener("keydown",(e)=>{
        if (e.keyCode==27){
            close_window();
        }
    })
    const [limite, setLimite] = useState();
    const [credito,setCredito] = useState(<></>);
    const [credVal, setCredVal] = useState(false);
    const [nombre,setNombre] = useState();
    const [dni,setDni] = useState();
    const [checkbox, setCheckbox] = useState(<></>);
    get_rango().then(rango => {
        console.log(rango)
        setCheckbox(rango == 'Admin' ? 
        <article>
            <p id="cuenta">Cuenta Corriente: </p>
            <input type="checkbox" name="credito"  id="credito" onChange={(e) => {
                let check=e.currentTarget.checked;
                setCredVal(check);
                if (!check){
                    setLimite(undefined);
                }
                setCredito(check ? <>
                <input type="number" placeholder="Límite" name="limite" id="limite" onChange={(e)=>setLimite(e.currentTarget.value)} required step="0.01" />
            </> : <></>)
        }} /> </article >:<></>)})
    
    
    return (
    <form className="add-form" onSubmit={()=>agregar_cliente(nombre,dni,credVal,limite)}>
        <input type="text" name="nombre" id="nombre" placeholder="Nombre" onChange={(e)=>setNombre(e.currentTarget.value)} required/>
        <input type="number" name="dni" id="dni" placeholder="DNI" onChange={(e)=>setDni(e.currentTarget.value)} required/>
        {checkbox}
        {credito}
        <button type="submit">Agregar</button>
    </form>
    )
}

export default Form;