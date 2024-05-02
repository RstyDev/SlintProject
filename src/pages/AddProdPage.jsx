import { useEffect } from "react";
import { useState } from "react";
import ProdForm from "../components/ProdFrom";
import PesForm from "../components/PesForm";
import RubForm from "../components/RubForm";
async function close_window() {
    return await invoke("close_window");
}
function Form(){
    document.addEventListener('keydown',(e)=>{
        if (e.keyCode==27){
            close_window();
        }
    })
    
    const [tipo,setTipo] = useState(0);
    const [form, setForm] = useState(seteaForm(<ProdForm />));
    function seteaForm(form){
        return <>
            <select name="tipo" id="tipo" onChange={(e)=>{console.log(e.currentTarget.value);setTipo(e.currentTarget.value)}}>
                <option value={0} defaultValue="selected">Producto</option>
                <option value={1}>Pesable</option>
                <option value={2}>Rubro</option>
            </select>
            {form}
        </>
    }
    
    useEffect(()=>{
        switch(tipo){
            case '0':
                setForm(seteaForm(<ProdForm />));
                break;
            case '1':
                setForm(seteaForm(<PesForm />));
                break;
            case '2':
                setForm(seteaForm(<RubForm />));
                break;
        }
    },[tipo])
    return(form)
}

export default Form;