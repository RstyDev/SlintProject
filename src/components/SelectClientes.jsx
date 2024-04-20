import { invoke } from "@tauri-apps/api/tauri";
import { useState } from "react";
import { useEffect } from "react";
function SelectClientes({setCredito}){
    const [clientes,setClientes]=useState([]);
    const [vec,setVec]=useState([{nombre: "Lucas", credito: true},{nombre: "Pablo",credito: false}]);
    const [rend, setRend] = useState(<select id="cliente"  onSelect={(e)=>{select(e)}}>
    <option value='0' defaultValue="selected" >Consumidor Final</option>
    {clientes}
</select>);
    useEffect(()=>{
        async function get_clientes() {
            return await invoke("get_clientes");
        }
        get_clientes().then(clientes=>{
            //setVec(clientes)
            setClientes(vec.map((cliente,i)=>{
                return <option value={i+1} key={i}>  {cliente.nombre}</option>
            }))
           
        })
    },[])
    useEffect(()=>{setRend(<select id="cliente"  onChange={(e)=>{select(e)}}>
    <option value='0' defaultValue="selected" >Consumidor Final</option>
    {clientes}
</select>)},[clientes,vec])
    function select(e){
        if (e.currentTarget.value>0){
        setCredito(vec[e.currentTarget.value - 1].credito)
    }else{
        setCredito(false)
    }
    }
    return(rend)
}

export default SelectClientes;