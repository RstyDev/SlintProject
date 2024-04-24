import { useEffect } from "react";
import { useState } from "react";
import ProdForm from "./ProdFrom";
import PesForm from "./PesForm";
import RubForm from "./RubForm";
function Form(){
    const prodForm = <ProdForm />
    const pesForm = <PesForm />
    const rubForm = <RubForm />
    const [tipo,setTipo] = useState(0);
    const [form, setForm] = useState(seteaForm(prodForm));
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
                setForm(seteaForm(prodForm));
                break;
            case '1':
                setForm(seteaForm(pesForm));
                break;
            case '2':
                setForm(seteaForm(rubForm));
                break;
        }
    },[tipo])
    return(form)
}

export default Form;