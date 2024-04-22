import { invoke } from "@tauri-apps/api/tauri";
import { useState } from "react";
import { useEffect } from "react";


function SelectClientes({setCredito,disabledCli,pos,cliente,setCliente}){ //TODO hay que terminar este comportamiento al cambiar de A y B
    const [clientes,setClientes]=useState([]);
    const [vec,setVec]=useState([]);
    const [rend, setRend] = useState(<select id="cliente" value={selectValue(cliente)} disabled={disabledCli} onChange={(e)=>{select(e)}}>
    <option value='0' defaultValue={cliente=="Final"?"selected":""} >Consumidor Final</option>
    {clientes}
</select>);
    function selectValue(cliente){
        if (cliente=="Final"){
            return 0
        }else{
            return cliente.Regular.id
        }
    }
    useEffect(()=>{
        async function get_clientes() {
            return await invoke("get_clientes");
        }
        get_clientes().then(cli=>{
            setVec(cli)
            setClientes(cli.map((cli,i)=>{
                console.log(cliente.Regular);
                console.log(cli)
                return <option value={cli.id} key={i} defaultValue={cliente!="Final" && cliente.Regular.id == cli.id ? "selected" : ""} > {cli.nombre}</option>
            }))
           
        })
    },[cliente,pos])
    useEffect(() => {
        setRend(<select id="cliente" value={selectValue(cliente)} disabled={disabledCli} onChange={(e)=>{select(e)}}>
    <option value='0' defaultValue="selected" >Consumidor Final</option>
    {clientes}
</select>)},[clientes,vec,disabledCli])
    function select(e){
        if (e.currentTarget.value>0){
            setCredito(vec[e.currentTarget.value - 1].credito);
            setCliente(vec[e.currentTarget.value-1].id)
        }else{
            setCredito(false) 
            setCliente(0)
        }
    }
    return(rend)
}

export default SelectClientes;