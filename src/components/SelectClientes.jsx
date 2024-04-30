import { invoke } from "@tauri-apps/api/tauri";
import { useState } from "react";
import { useEffect } from "react";


function SelectClientes({setCredito,disabledCli,pos,cliente,setCliente}){ 
    const [clientes,setClientes]=useState([]);
    const [vec,setVec]=useState([]);
    const [client,setClient]=useState(cliente);
    const [rend, setRend] = useState(<select id="cliente" value={selectValue(client)} disabled={disabledCli} onChange={(e)=>{select(e)}}>
    <option value='0' defaultValue={client=="Final"?"selected":""} >Consumidor Final</option>
    {clientes}
</select>);
    function selectValue(client){
        if (client=="Final"){
            return 0
        }else{
            return client.Regular.id
        }
    }
    useEffect(()=>{setClient(cliente)},[cliente])
    useEffect(()=>{
        async function get_clientes() {
            return await invoke("get_clientes");
        }
        get_clientes().then(cli=>{
            setVec(cli)
            setClientes(cli.map((cli,i)=>{
                console.log(client);
                console.log(cli)
                return <option value={cli.id} key={i} defaultValue={client!="Final" && client.Regular.id == cli.id ? "selected" : ""} > {cli.nombre}</option>
            }))
           
        })
    },[client,pos])
    useEffect(() => {
        setRend(<select id="cliente" value={selectValue(client)} disabled={disabledCli} onChange={(e)=>{select(e)}}>
    <option value='0' defaultValue="selected" >Consumidor Final</option>
    {clientes}
</select>)},[clientes,vec,disabledCli])
    const select=(e)=>{
        if(e.currentTarget.value>0){
        setCliente(vec[e.currentTarget.value-1])
    }else{
        setCliente({id:0})
    }
}
    
    return(rend)
}

export default SelectClientes;