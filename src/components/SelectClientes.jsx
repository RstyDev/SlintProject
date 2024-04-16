import { invoke } from "@tauri-apps/api/tauri";
import { useState } from "react";
import { useEffect } from "react";
function SelectClientes(){
    const [clientes,setClientes]=useState([]);
    useEffect(()=>{
        async function get_clientes() {
            return await invoke("get_clientes");
        }
        get_clientes().then(clientes=>{
            setClientes(clientes.map(cliente=>{
                <option value={cliente.id}> {cliente.nombre}</option>
            }))
        })
    },[])

    return(<select id="cliente">
        <option value='0' defaultValue="selected" >Consumidor Final</option>
        {clientes}
    </select>)
}

export default SelectClientes;